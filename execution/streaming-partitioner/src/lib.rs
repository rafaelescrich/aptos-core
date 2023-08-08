// Copyright © Aptos Foundation

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::marker::PhantomData;
use aptos_block_executor::task::Transaction;
use aptos_types::block_executor::partitioner::{ShardId, TxnIndex};

/// Indicates the position of the transaction in the serialization order of the block.
pub type SerializationIdx = u32;

struct TxnInfo<T: Transaction> {
    write_set: Vec<T::Key>,
    serialization_idx: SerializationIdx,

    _phantom: PhantomData<T>,
}

pub struct StreamingPartitioner<T: Transaction> {
    n_shards: usize,
    min_per_shard_batch_size: usize,

    shard_queues: Vec<VecDeque<TxnInfo<T>>>,
    min_shard_queue_size: usize,
    max_shard_queue_size: usize,
    last_write_location: HashMap<T::Key, (ShardId, SerializationIdx)>,

    size_mult_factor: f64,  // alpha in the Fennel paper
    size_power_factor: f64,  // gamma in the Fennel paper
}

impl<T> StreamingPartitioner<T>
    where
        T: PTran,
        T::Key: Eq + Hash + Clone,
{
    pub fn transactions_finished(&mut self, shard_id: ShardId, count: usize) {
        let queue = &mut self.shard_queues[shard_id];
        for _ in 0..count {
            let tx_info = queue.pop_front().unwrap();
            for key in tx_info.write_set {
                if self.last_write_location[&key] == (shard_id, tx_info.serialization_idx) {
                    self.last_write_location.remove(&key);
                }
            }
        }
    }

    pub fn add_transactions(&mut self, transactions: Vec<T>) {

    }
}
