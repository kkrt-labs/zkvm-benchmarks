use utils::benchmark;
use zkm_script::{benchmark_ecdsa, init_logger};

fn main() {
    init_logger();

    let lengths = [1];
    benchmark(
        benchmark_ecdsa,
        &lengths,
        "../benchmark_outputs/ecdsa_zkm.csv",
    );
}
