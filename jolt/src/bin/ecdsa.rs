use jolt::Serializable;
use std::time::Instant;
use utils::{benchmark, ecdsa_input, BenchResult, ECDSA_INPUTS};

fn main() {
    let csv_file = format!(
        "../benchmark_outputs/ecdsa_jolt{}{}.csv",
        if cfg!(feature = "icicle") { "-gpu" } else { "" },
        ""
    );

    benchmark(bench_ecdsa, &ECDSA_INPUTS, &csv_file);
}

fn bench_ecdsa(_dummy: usize) -> BenchResult {
    let input = ecdsa_input();
    let (prove_ecdsa_verify, _verify_ecdsa_verify) = ecdsa_guest::build_ecdsa_verify();

    let program_summary = ecdsa_guest::analyze_ecdsa_verify(input.clone());

    let start = Instant::now();
    let (_output, proof) = prove_ecdsa_verify(input.clone());
    let end = Instant::now();

    (
        end.duration_since(start),
        proof.size().unwrap(),
        program_summary.processed_trace.len(),
    )
}
