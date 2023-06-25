// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::common::TxnIdx;
use aptos_state_view::StateView;
use aptos_types::transaction::{Transaction, TransactionOutput};
use aptos_vm_types::output::VMOutput;
use rayon::Scope;
use std::sync::mpsc::{channel, Sender};

pub(crate) struct PtxFinalizer;

impl PtxFinalizer {
    pub fn spawn<'scope, 'view: 'scope>(
        scope: &Scope<'scope>,
        result_tx: Sender<TransactionOutput>,
        base_view: &'view (impl StateView + Sync),
    ) -> PtxFinalizerClient {
        let (work_tx, work_rx) = channel();
        scope.spawn(move |_scope| {
            let mut worker = Worker {
                result_tx,
                base_view,
            };
            loop {
                match work_rx.recv().expect("Channel closed.") {
                    Command::AddVMOutput {
                        txn_idx,
                        txn,
                        vm_output,
                    } => worker.add_vm_output(txn_idx, txn, vm_output),
                    Command::FinishBlock => worker.finish_block(),
                }
            }
        });
        PtxFinalizerClient { work_tx }
    }
}

#[derive(Clone)]
pub(crate) struct PtxFinalizerClient {
    work_tx: Sender<Command>,
}

impl PtxFinalizerClient {
    pub fn add_vm_output(&self, _txn_idx: TxnIdx, _txn: Transaction, _vm_output: VMOutput) {}

    pub fn finish_block(&self) {
        todo!()
    }

    fn send_to_worker(&self, command: Command) {
        self.work_tx.send(command).expect("Work thread died.");
    }
}

struct Worker<'view> {
    result_tx: Sender<TransactionOutput>,
    base_view: &'view (dyn StateView + Sync),
}

impl<'view> Worker<'view> {
    fn add_vm_output(&mut self, _txn_idx: TxnIdx, _txn: Transaction, _vm_output: VMOutput) {
        todo!()
    }

    fn finish_block(&mut self) {
        todo!()
    }
}

enum Command {
    AddVMOutput {
        txn_idx: TxnIdx,
        txn: Transaction,
        vm_output: VMOutput,
    },
    FinishBlock,
}
