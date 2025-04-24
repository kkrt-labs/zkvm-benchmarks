use utils::benchmark_v2;
use zkm_script::{benchmark_sha3_chain, init_logger};

fn main() {
    init_logger();

    let iters = [230, 460 /* 920, 1840,  3680 */];
    benchmark_v2(
        benchmark_sha3_chain,
        &iters,
        "../benchmark_outputs/sha3_chain_zkm.csv",
        "iters",
    );
}
