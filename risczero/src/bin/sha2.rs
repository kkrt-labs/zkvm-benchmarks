use risc0_zkvm::serde::to_vec;
use risc0_zkvm::{
    get_prover_server, sha::Digest, ExecutorEnv, ExecutorImpl, ProverOpts, VerifierContext,
};
use std::time::Instant;
use utils::{bench::benchmark_v2, bench::Metrics, metadata::SHA2_INPUTS, sha2_input};

pub fn main() {
    let csv_file = format!(
        "../.outputs/benchmark/sha2_risczero{}{}.csv",
        if cfg!(feature = "cuda") { "-gpu" } else { "" },
        ""
    );
    benchmark_v2(benchmark_sha2, &SHA2_INPUTS, &csv_file);
}

fn benchmark_sha2(num_bytes: usize) -> Metrics {
    let mut metrics = Metrics::new(num_bytes);
    const ELF: &[u8] = risc0_benchmark_methods::BIG_SHA2_ELF;
    let image_id: Digest = risc0_benchmark_methods::BIG_SHA2_ID.into();
    let message = sha2_input(num_bytes);
    let input = to_vec(&message).unwrap();

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
