use utils::{bench::benchmark, metadata::SHA2_INPUTS};
use zkm_script::{bench_zkm_sha256, init_logger};

fn main() {
    init_logger();

    benchmark(
        bench_zkm_sha256,
        &SHA2_INPUTS,
        "../.outputs/benchmark/sha2_zkm.csv",
    );
}
