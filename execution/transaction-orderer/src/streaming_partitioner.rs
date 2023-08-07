


// Copyright © Aptos Foundation

pub struct StreamingPartitioner {
    n_shards: usize,
    min_per_shard_batch_size: usize,
    alpha: f64,
    gamma: f64,
}
