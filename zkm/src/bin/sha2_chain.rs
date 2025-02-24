use utils::benchmark;
use zkm_script::{benchmark_sha2_chain, init_logger};

fn main() {
    init_logger();

    let iters = [230, 460, /* 920, 1840, 3680 */];
    benchmark(benchmark_sha2_chain, &iters, "../benchmark_outputs/sha2_chain_zkm.csv", "iters");
}
