use zkm2_script::{benchmark_sha2_chain, benchmark_with_shard_size, init_logger};

fn main() {
    init_logger();

    let iters = [230, 460, /* 920, 1840,  3680 */];

    // 1 Shard
    let shard_sizes = [1 << 20, 1 << 21, /* 1 << 22, 1 << 23, 1 << 24 */]; // Max shard_size = 2^24-1
    benchmark_with_shard_size(benchmark_sha2_chain, &iters, &shard_sizes, "../benchmark_outputs/sha2_chain_zkm2_1_shard.csv", "iters");

    // 2 Shards
    let shard_sizes = [1 << 19, 1 << 20, /* 1 << 21, 1 << 22, 1 << 23 */];
    benchmark_with_shard_size(benchmark_sha2_chain, &iters, &shard_sizes, "../benchmark_outputs/sha2_chain_zkm2_2_shard.csv", "iters");

    // 4 Shards
    let shard_sizes = [1 << 18, 1 << 19, /* 1 << 20, 1 << 21, 1 << 22 */];
    benchmark_with_shard_size(benchmark_sha2_chain, &iters, &shard_sizes, "../benchmark_outputs/sha2_chain_zkm2_4_shard.csv", "iters");

    // 8 Shards
    let shard_sizes = [1 << 17, 1 << 18, /* 1 << 19, 1 << 20, 1 << 21*/];
    benchmark_with_shard_size(benchmark_sha2_chain, &iters, &shard_sizes, "../benchmark_outputs/sha2_chain_zkm2_8_shard.csv", "iters");

    // 16 Shards
    let shard_sizes = [1 << 16, 1 << 17, /* 1 << 18, 1 << 19, 1 << 20*/];
    benchmark_with_shard_size(benchmark_sha2_chain, &iters, &shard_sizes, "../benchmark_outputs/sha2_chain_zkm2_16_shard.csv", "iters");
}
