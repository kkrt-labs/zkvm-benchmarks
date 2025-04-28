use jolt::Serializable;
use std::time::Instant;
use utils::{bench::benchmark, bench::Metrics, metadata::SHA2_INPUTS, sha2_input};

fn main() {
    let csv_file = format!(
        "../.outputs/benchmark/sha2_jolt{}{}.csv",
        if cfg!(feature = "icicle") { "-gpu" } else { "" },
        ""
    );

    benchmark(benchmark_sha2, &SHA2_INPUTS, &csv_file);
}

fn benchmark_sha2(num_bytes: usize) -> Metrics {
    let mut metrics = Metrics::new(num_bytes as usize);

    let (prove_sha2, verify_sha2) = sha2_guest::build_sha2();

    let input = sha2_input(num_bytes);
    let start = Instant::now();
    let program_summary = sha2_guest::analyze_sha2(&input);
    metrics.exec_duration = start.elapsed();
    metrics.cycles = program_summary.processed_trace.len() as u64;

    let start = Instant::now();
    let (_output, proof) = prove_sha2(&input);
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = proof.size().unwrap();

    let start = Instant::now();
    let _verify_result = verify_sha2(proof);
    metrics.verify_duration = start.elapsed();

    metrics
}
