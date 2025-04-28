use std::time::Instant;

use risc0_zkvm::serde::to_vec;
use risc0_zkvm::{
    get_prover_server, sha::Digest, ExecutorEnv, ExecutorImpl, ProverOpts, VerifierContext,
};
use utils::{bench::benchmark_v2, bench::Metrics, metadata::ETHTRANSFER_INPUTS};

pub fn main() {
    benchmark_v2(
        benchmark_ethtransfer,
        &ETHTRANSFER_INPUTS,
        "../.outputs/benchmark/ethtransfer_risczero.csv",
    );
}

fn benchmark_ethtransfer(iterations: usize) -> Metrics {
    let mut metrics = Metrics::new(iterations);
    const ELF: &[u8] = risc0_benchmark_methods::TRANSFER_ETH_ELF;
    let image_id: Digest = risc0_benchmark_methods::TRANSFER_ETH_ID.into();
    let input = to_vec(&iterations).unwrap();

    let env = ExecutorEnv::builder().write_slice(&input).build().unwrap();
    let mut exec = ExecutorImpl::from_elf(env, &ELF).unwrap();
    let start = Instant::now();
    let session = exec.run().unwrap();
    metrics.exec_duration = start.elapsed();
    metrics.cycles = session.user_cycles;

    let prover = get_prover_server(&ProverOpts::succinct()).unwrap();
    let ctx = VerifierContext::default();

    let start = Instant::now();
    let receipt = prover.prove_session(&ctx, &session).unwrap().receipt;
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = receipt.inner.succinct().unwrap().seal_size();

    let start = Instant::now();
    receipt.verify(image_id).unwrap();
    metrics.verify_duration = start.elapsed();

    metrics
}
