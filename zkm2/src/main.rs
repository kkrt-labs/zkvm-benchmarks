use utils::benchmark;
use zkm2_script::*;

fn main() {
    init_logger();

    let iters = [230, 460, /* 920, 1840,  3680 */];
    benchmark(benchmark_sha3_chain, &iters, "../benchmark_outputs/sha3_chain_zkm2.csv", "iters");

    let lengths = [32, 256, 512, 1024, 2048];
    benchmark(benchmark_sha2, &lengths, "../benchmark_outputs/sha2_zkm2.csv", "byte length");
    benchmark(benchmark_sha3, &lengths, "../benchmark_outputs/sha3_zkm2.csv", "byte length");

    let ns = [100, 1000, 10000, 50000];
    benchmark(bench_fibonacci, &ns, "../benchmark_outputs/fibonacci_zkm2.csv", "n");

    let values = [5u32];
    benchmark(bench_bigmem, &values, "../benchmark_outputs/bigmem_zkm2.csv", "value");

    // 1 Shard
    let shard_sizes = [1 << 20, 1 << 21, /* 1 << 22, 1 << 23, 1 << 24 */]; // Max shard_size = 2^24-1
    benchmark_with_shard_size(benchmark_sha2_chain, &iters, &shard_sizes, "../benchmark_outputs/sha2_chain_zkm2_1_shard.csv", "iters");

    // 2 Shards
    let iters = [230, 460, /* 920, 1840, 3680 */];
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
