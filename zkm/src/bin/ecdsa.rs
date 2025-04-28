use utils::{bench::benchmark, metadata::ECDSA_INPUTS};
use zkm_script::{bench_ecdsa, init_logger};

fn main() {
    init_logger();

    benchmark(
        bench_ecdsa,
        &ECDSA_INPUTS,
        "../.outputs/benchmark/ecdsa_zkm.csv",
    );
}
