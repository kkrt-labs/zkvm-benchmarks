use utils::bench::benchmark;
use zkm_script::{bench_bigmem, init_logger};

fn main() {
    init_logger();

    let values = [5u32];
    benchmark(bench_bigmem, &values, "../benchmark_outputs/bigmem_zkm.csv");
}
