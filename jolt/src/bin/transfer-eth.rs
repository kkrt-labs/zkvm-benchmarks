use jolt::Serializable;
use std::time::Instant;
use utils::{bench::benchmark, bench::Metrics, metadata::ETHTRANSFER_INPUTS};

const TARGET_DIR: &str = "./transfer-eth-guest";

fn main() {
    let csv_file = format!(
        "../.outputs/benchmark/ethblock_jolt{}{}.csv",
        if cfg!(feature = "icicle") { "-gpu" } else { "" },
        ""
    );

    benchmark(benchmark_transfer_eth, &ETHTRANSFER_INPUTS, &csv_file);
}

fn benchmark_transfer_eth(n: usize) -> Metrics {
    let mut metrics = Metrics::new(n as usize);

    let program = transfer_eth_guest::compile_transfer_eth_n_times(TARGET_DIR);
    let prover_preprocessing = transfer_eth_guest::preprocess_prover_transfer_eth_n_times(&program);
    let verifier_preprocessing =
        transfer_eth_guest::preprocess_verifier_transfer_eth_n_times(&program);

    let prover =
        transfer_eth_guest::build_prover_transfer_eth_n_times(program, prover_preprocessing);
    let verifier = transfer_eth_guest::build_verifier_transfer_eth_n_times(verifier_preprocessing);

    let start = Instant::now();
    let program_summary = transfer_eth_guest::analyze_transfer_eth_n_times(n);
    metrics.exec_duration = start.elapsed();
    metrics.cycles = program_summary.processed_trace.len() as u64;

    let start = Instant::now();
    let (output, proof) = prover(n);
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = proof.size().unwrap();

    let start = Instant::now();
    let _verify_result = verifier(n, output, proof);
    metrics.verify_duration = start.elapsed();

    metrics
}
