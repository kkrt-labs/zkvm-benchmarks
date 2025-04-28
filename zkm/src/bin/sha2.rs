use utils::{bench::benchmark, metadata::SHA2_INPUTS};
use zkm_script::{benchmark_sha2, init_logger};

fn main() {
    init_logger();

    benchmark(
        benchmark_sha2,
        &SHA2_INPUTS,
        "../benchmark_outputs/sha2_zkm.csv",
    );
}
