// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

//! TODO(aldenhu): doc

use crate::{
    common::{TxnIdx, VersionedKey, EXPECTANT_BLOCK_KEYS, EXPECTANT_BLOCK_SIZE},
    finalizer::PtxFinalizerClient,
};
use anyhow::{anyhow, Result};
use aptos_state_view::{StateView, TStateView};
use aptos_types::{
    state_store::{
        state_key::StateKey, state_storage_usage::StateStorageUsage, state_value::StateValue,
    },
    transaction::Transaction,
};
use rayon::Scope;
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    sync::mpsc::{channel, Receiver, Sender},
};

pub(crate) struct PtxExecutor;

impl PtxExecutor {
    pub fn spawn<'scope, 'view: 'scope>(
        scope: &'scope Scope<'scope>,
        base_view: &'view (impl StateView + Sync),
        num_workers: usize,
        finalizer: PtxFinalizerClient,
    ) -> PtxExecutorClient {
        let (work_tx, work_rx) = channel();
        let mut manager = WorkerManager::new(scope, num_workers, finalizer, base_view);
        scope.spawn(move |_scope| loop {
            match work_rx.recv().expect("Work thread died.") {
                Command::InformStateValue { key, value } => {
                    manager.inform_state_value(key, value);
                },
                Command::AddTransaction {
                    transaction,
                    dependencies,
                } => {
                    manager.add_transaction(transaction, dependencies);
                },
                Command::FinishBlock => {
                    manager.finish_block();
                    break;
                },
            }
        });
        PtxExecutorClient { work_tx }
    }
}

#[derive(Clone)]
pub(crate) struct PtxExecutorClient {
    work_tx: Sender<Command>,
}

impl PtxExecutorClient {
    pub fn inform_state_value(&self, key: VersionedKey, value: Option<StateValue>) {
        self.send_to_manager(Command::InformStateValue { key, value })
    }

    fn send_to_manager(&self, command: Command) {
        self.work_tx.send(command).expect("Channel died.")
    }

    pub fn schedule_execute_transaction(
        &self,
        transaction: Transaction,
        dependencies: HashSet<(StateKey, TxnIdx)>,
    ) {
        self.send_to_manager(Command::AddTransaction {
            transaction,
            dependencies,
        })
    }

    pub fn finish_block(&self) {
        self.send_to_manager(Command::FinishBlock)
    }
}

enum Command {
    InformStateValue {
        key: VersionedKey,
        value: Option<StateValue>,
    },
    AddTransaction {
        transaction: Transaction,
        dependencies: HashSet<(StateKey, TxnIdx)>,
    },
    FinishBlock,
}

type WorkerIndex = usize;

enum StateValueState {
    Pending { subscribers: Vec<TxnIdx> },
    Ready { value: Option<StateValue> },
}

struct PendingTransaction {
    transaction: Transaction,
    pending_dependencies: HashSet<VersionedKey>,
    met_dependencies: HashMap<StateKey, Option<StateValue>>,
}

impl PendingTransaction {
    fn new(transaction: Transaction) -> Self {
        Self {
            transaction,
            pending_dependencies: HashSet::new(),
            met_dependencies: HashMap::new(),
        }
    }
}

struct WorkerManager {
    finalizer: PtxFinalizerClient,
    work_txs: Vec<Sender<WorkerCommand>>,
    worker_ready_rx: Receiver<WorkerIndex>,
    transactions: Vec<Option<PendingTransaction>>,
    state_values: HashMap<VersionedKey, StateValueState>,
}

impl WorkerManager {
    fn new<'scope, 'view: 'scope>(
        scope: &'scope Scope<'scope>,
        num_workers: usize,
        finalizer: PtxFinalizerClient,
        base_view: &'view (impl StateView + Sync),
    ) -> Self {
        let (worker_ready_tx, worker_ready_rx) = channel();
        let work_txs = (0..num_workers)
            .map(|worker_idx| {
                Worker::spawn(
                    scope,
                    finalizer.clone(),
                    worker_ready_tx.clone(),
                    worker_idx,
                    base_view,
                )
            })
            .collect();

        Self {
            finalizer,
            work_txs,
            worker_ready_rx,
            transactions: Vec::with_capacity(EXPECTANT_BLOCK_SIZE),
            state_values: HashMap::with_capacity(EXPECTANT_BLOCK_KEYS),
        }
    }

    // Top level API
    fn inform_state_value(&mut self, key: VersionedKey, value: Option<StateValue>) {
        match self.state_values.entry(key.clone()) {
            Entry::Occupied(mut existing) => {
                let old_state = existing.insert(StateValueState::Ready {
                    value: value.clone(),
                });
                match old_state {
                    StateValueState::Pending { subscribers } => {
                        for subscriber in subscribers {
                            // TODO(ptx): reduce clone / memcpy
                            self.inform_state_value_to_txn(subscriber, key.clone(), value.clone());
                        }
                    },
                    StateValueState::Ready { .. } => {
                        unreachable!("StateValue pushed twice.")
                    },
                }
            },
            Entry::Vacant(vacant) => {
                vacant.insert(StateValueState::Ready { value });
            },
        }
    }

    fn inform_state_value_to_txn(
        &mut self,
        txn_idx: TxnIdx,
        versioned_key: VersionedKey,
        value: Option<StateValue>,
    ) {
        let pending_txn = self.expect_pending_txn(txn_idx);
        assert!(
            pending_txn.pending_dependencies.remove(&versioned_key),
            "Pending dependency not found."
        );
        let (key, _txn_idx) = versioned_key;
        pending_txn.met_dependencies.insert(key, value);

        if pending_txn.pending_dependencies.is_empty() {
            self.execute_transaction(txn_idx);
        }
    }

    fn expect_pending_txn(&mut self, txn_idx: TxnIdx) -> &mut PendingTransaction {
        self.transactions[txn_idx]
            .as_mut()
            .expect("Transaction is not Pending.")
    }

    // Top level API
    fn add_transaction(&mut self, transaction: Transaction, dependencies: HashSet<VersionedKey>) {
        let txn_idx = self.transactions.len();
        self.transactions
            .push(Some(PendingTransaction::new(transaction)));
        let pending_txn = self.transactions[txn_idx]
            .as_mut()
            .expect("Transaction is not Pending.");

        for versioned_key in dependencies {
            match self.state_values.entry(versioned_key.clone()) {
                Entry::Occupied(mut existing) => match existing.get_mut() {
                    StateValueState::Pending { subscribers } => {
                        pending_txn.pending_dependencies.insert(versioned_key);
                        subscribers.push(txn_idx);
                    },
                    StateValueState::Ready { value } => {
                        let (key, _txn_idx) = versioned_key;
                        pending_txn.met_dependencies.insert(key, value.clone());
                    },
                },
                Entry::Vacant(vacant) => {
                    pending_txn.pending_dependencies.insert(versioned_key);
                    vacant.insert(StateValueState::Pending {
                        subscribers: vec![txn_idx],
                    });
                },
            }
        }

        if pending_txn.pending_dependencies.is_empty() {
            self.execute_transaction(txn_idx);
        }
    }

    fn execute_transaction(&mut self, txn_idx: TxnIdx) {
        let PendingTransaction {
            transaction,
            pending_dependencies,
            met_dependencies,
        } = self.transactions[txn_idx]
            .take()
            .expect("Transaction is not Pending.");
        assert!(
            pending_dependencies.is_empty(),
            "Transaction has pending dependencies."
        );

        let worker_index = self.worker_ready_rx.recv().expect("Channel closed.");
        self.work_txs[worker_index]
            .send(WorkerCommand::ExecuteTransaction {
                txn_idx,
                transaction,
                met_dependencies,
            })
            .expect("Worker died.");
    }

    fn finish_block(&self) {
        // inform all workers to quit
        for work_tx in &self.work_txs {
            work_tx.send(WorkerCommand::Finish).ok();
        }

        // TODO(aldenhu): check the quitting logic
        // wait for all workers to quit
        while self.worker_ready_rx.recv().is_ok() {}

        // inform finalizer to quit
        self.finalizer.finish_block();
    }
}

enum WorkerCommand {
    ExecuteTransaction {
        txn_idx: TxnIdx,
        transaction: Transaction,
        met_dependencies: HashMap<StateKey, Option<StateValue>>,
    },
    Finish,
}

struct Worker<'view> {
    finalizer: PtxFinalizerClient,
    work_rx: Receiver<WorkerCommand>,
    worker_ready_tx: Sender<WorkerIndex>,
    worker_index: WorkerIndex,
    base_view: &'view (dyn StateView + Sync),
}

impl<'scope, 'view: 'scope> Worker<'view> {
    fn spawn(
        scope: &'scope Scope<'scope>,
        finalizer: PtxFinalizerClient,
        worker_ready_tx: Sender<WorkerIndex>,
        worker_index: WorkerIndex,
        base_view: &'view (impl StateView + Sync),
    ) -> Sender<WorkerCommand> {
        let (work_tx, work_rx) = channel();
        scope.spawn(move |_scope| {
            let worker = Self {
                finalizer,
                work_rx,
                worker_ready_tx,
                worker_index,
                base_view,
            };
            worker.work()
        });
        work_tx
    }

    fn work(self) {
        /*
        let vm = todo!();
        loop {
            match self.work_rx.recv().expect("Sender died.") {
                WorkerCommand::ExecuteTransaction {
                    txn_idx,
                    transaction,
                    met_dependencies,
                } => {
                    let state_view = DependenciesStateView { met_dependencies };

                    // TODO(aldenhu): call the VM
                    let vm_output = { todo!() };

                    self.finalizer
                        .add_vm_output(txn_idx, transaction, vm_output);
                    self.worker_ready_tx.send(self.worker_index).ok();
                },
                WorkerCommand::Finish => {
                    break;
                },
            }
        }
         */
    }
}

struct DependenciesStateView {
    met_dependencies: HashMap<StateKey, Option<StateValue>>,
    // TODO: add ability to read modules from base view (as a hack)
}

impl TStateView for DependenciesStateView {
    type Key = StateKey;

    fn get_state_value(&self, state_key: &Self::Key) -> Result<Option<StateValue>> {
        self.met_dependencies
            .get(state_key)
            .cloned()
            .ok_or_else(|| anyhow!("Dependency not met."))
    }

    fn get_usage(&self) -> anyhow::Result<StateStorageUsage> {
        // TODO(aldenhu): maybe remove get_usage() from StateView
        unimplemented!()
    }
}
