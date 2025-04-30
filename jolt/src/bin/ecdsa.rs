use jolt::Serializable;
use std::time::Instant;
use utils::{bench::benchmark, bench::Metrics, ecdsa_input, metadata::ECDSA_INPUTS};

const TARGET_DIR: &str = "./ecdsa-guest";

fn main() {
    let csv_file = format!(
        "../.outputs/benchmark/ecdsa_jolt{}{}.csv",
        if cfg!(feature = "icicle") { "-gpu" } else { "" },
        ""
    );

    benchmark(bench_ecdsa, &ECDSA_INPUTS, &csv_file);
}

fn bench_ecdsa(size: usize) -> Metrics {
    let mut metrics = Metrics::new(size as usize);
    let input = ecdsa_input();

    let program = ecdsa_guest::compile_ecdsa_verify(TARGET_DIR);
    let prover_preprocessing = ecdsa_guest::preprocess_prover_ecdsa_verify(&program);
    let verifier_preprocessing = ecdsa_guest::preprocess_verifier_ecdsa_verify(&program);

    let prover = ecdsa_guest::build_prover_ecdsa_verify(program, prover_preprocessing);
    let verifier = ecdsa_guest::build_verifier_ecdsa_verify(verifier_preprocessing);

    let start = Instant::now();
    let program_summary = ecdsa_guest::analyze_ecdsa_verify(input.clone());
    metrics.exec_duration = start.elapsed();
    metrics.cycles = program_summary.processed_trace.len() as u64;
    // save_summary_to_json(&program_summary, "../.outputs/traces/ecdsa_jolt.json")
    //     .expect("Failed to save program summary");

    let start = Instant::now();
    let (output, proof) = prover(input.clone());
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = proof.size().unwrap();

    let start = Instant::now();
    let _verify_result = verifier(input.clone(), output, proof);
    metrics.verify_duration = start.elapsed();

    metrics
}
