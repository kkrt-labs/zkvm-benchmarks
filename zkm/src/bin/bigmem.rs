use utils::benchmark;
use zkm_script::{benchmark_bigmem, init_logger};

fn main() {
    init_logger();

    let values = [5];
    benchmark(
        benchmark_bigmem,
        &values,
        "../benchmark_outputs/bigmem_zkm.csv",
    );
}
