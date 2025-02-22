use std::time::{Duration, Instant};
use jolt::Serializable;
use utils::benchmark;

type BenchResult = (Duration, usize, usize);

fn main() {
    let csv_file = format!(
        "../benchmark_outputs/sha2_jolt{}{}.csv",
        if cfg!(feature = "icicle") { "_gpu" } else { "" },
        ""
    );

    let lengths = [32, 256, 512, 1024, 2048];
    benchmark(
        benchmark_sha2,
        &lengths,
        &csv_file,
        "n",
    );
}

fn benchmark_sha2(num_bytes: usize) -> BenchResult {
    let (prove_sha2, _verify_sha2) = sha2_guest::build_sha2();

    let input = vec![5u8; num_bytes];
    let input = input.as_slice();
    let program_summary = sha2_guest::analyze_sha2(input);

    let start = Instant::now();
    let (_output, proof) = prove_sha2(input);
    let end = Instant::now();

    (
        end.duration_since(start),
        proof.size().unwrap(),
        program_summary.processed_trace.len(),
    )
}