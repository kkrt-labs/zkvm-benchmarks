use std::time::{Duration, Instant};
use jolt::Serializable;
use utils::benchmark;

type BenchResult = (Duration, usize, usize);

fn main() {
    let lengths = [32, 256, 512, 1024, 2048];
    benchmark(benchmark_sha3, &lengths, "../benchmark_outputs/sha3_jolt.csv", "byte length");
}

fn benchmark_sha3(num_bytes: usize) -> BenchResult {
    let (prove_sha3, _verify_sha3) = sha3_guest::build_sha3();

    let input = vec![5u8; num_bytes];
    let input = input.as_slice();
    let program_summary = sha3_guest::analyze_sha3(input);

    let start = Instant::now();
    let (_output, proof) = prove_sha3(input);
    let end = Instant::now();

    (
        end.duration_since(start),
        proof.size().unwrap(),
        program_summary.processed_trace.len(),
    )
}