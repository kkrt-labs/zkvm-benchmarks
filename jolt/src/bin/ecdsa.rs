use jolt::Serializable;
use std::time::Instant;
use utils::{bench::benchmark_v2, bench::Metrics, ecdsa_input, metadata::ECDSA_INPUTS};

fn main() {
    let csv_file = format!(
        "../.outputs/benchmark/ecdsa_jolt{}{}.csv",
        if cfg!(feature = "icicle") { "-gpu" } else { "" },
        ""
    );

    benchmark_v2(bench_ecdsa, &ECDSA_INPUTS, &csv_file);
}

fn bench_ecdsa(size: usize) -> Metrics {
    let mut metrics = Metrics::new(size as usize);
    let input = ecdsa_input();
    let (prove_ecdsa_verify, verify_ecdsa_verify) = ecdsa_guest::build_ecdsa_verify();

    let start = Instant::now();
    let program_summary = ecdsa_guest::analyze_ecdsa_verify(input.clone());
    metrics.exec_duration = start.elapsed();
    metrics.cycles = program_summary.processed_trace.len() as u64;
    // save_summary_to_json(&program_summary, "../.outputs/traces/ecdsa_jolt.json")
    //     .expect("Failed to save program summary");

    let start = Instant::now();
    let (_output, proof) = prove_ecdsa_verify(input.clone());
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = proof.size().unwrap();

    let start = Instant::now();
    let _verify_result = verify_ecdsa_verify(proof);
    metrics.verify_duration = start.elapsed();

    metrics
}
