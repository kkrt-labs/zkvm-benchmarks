use utils::{bench::benchmark_v2, metadata::SHA2_INPUTS};
use zkm_script::{benchmark_sha2, init_logger};

fn main() {
    init_logger();

    benchmark_v2(
        benchmark_sha2,
        &SHA2_INPUTS,
        "../.outputs/benchmark/sha2_zkm.csv",
    );
}
