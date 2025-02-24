use utils::benchmark;
use zkm_script::{benchmark_sha2, init_logger};

fn main() {
    init_logger();

    let lengths = [32, 256, 512, 1024, 2048];
    benchmark(benchmark_sha2, &lengths, "../benchmark_outputs/sha2_zkm.csv", "byte length");
}
