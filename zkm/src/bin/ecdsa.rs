use utils::{benchmark, ECDSA_INPUTS};
use zkm_script::{bench_ecdsa, init_logger};

fn main() {
    init_logger();

    benchmark(
        bench_ecdsa,
        &ECDSA_INPUTS,
        "../benchmark_outputs/ecdsa_zkm.csv",
    );
}
