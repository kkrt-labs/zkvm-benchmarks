use utils::{bench::benchmark, metadata::SHA2_INPUTS};
use zkm_script::{benchmark_sha3, init_logger};

fn main() {
    init_logger();

    benchmark(
        benchmark_sha3,
        &SHA2_INPUTS,
        "../benchmark_outputs/sha3_zkm.csv",
    );
}
