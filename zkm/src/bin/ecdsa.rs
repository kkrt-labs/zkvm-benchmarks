use utils::{bench::benchmark_v2, metadata::ECDSA_INPUTS};
use zkm_script::{bench_ecdsa, init_logger};

fn main() {
    init_logger();

    benchmark_v2(
        bench_ecdsa,
        &ECDSA_INPUTS,
        "../.outputs/benchmark/ecdsa_zkm.csv",
    );
}
