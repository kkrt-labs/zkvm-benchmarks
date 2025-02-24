use utils::benchmark;
use zkm_script::*;

fn main() {
    init_logger();

    let _ = std::fs::remove_dir_all("/tmp/zkm.old");
    let _ = std::fs::rename("/tmp/zkm", "/tmp/zkm.old");

    // let lengths = [32, 256, 512, 1024, 2048];
    // benchmark(benchmark_sha2, &lengths, "../benchmark_outputs/sha2_zkm.csv", "byte length");
    // benchmark(benchmark_sha3, &lengths, "../benchmark_outputs/sha3_zkm.csv", "byte length");

    // let ns = [10, 50, 90];
    // benchmark(benchmark_fibonacci, &ns, "../benchmark_outputs/fiboancci_zkm.csv", "n");

    // let values = [5];
    // benchmark(benchmark_bigmem, &values, "../benchmark_outputs/bigmem_zkm.csv", "value");

    // let iters = [230, 460, /* 920, 1840, 3680 */];
    // benchmark(benchmark_sha2_chain, &iters, "../benchmark_outputs/sha2_chain_zkm.csv", "iters");
    // benchmark(benchmark_sha3_chain, &iters, "../benchmark_outputs/sha3_chain_zkm.csv", "iters");
}
