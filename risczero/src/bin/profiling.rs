use risc0_zkvm::serde::to_vec;
use risc0_zkvm::{get_prover_server, ExecutorEnv, ExecutorImpl, ProverOpts, VerifierContext};
use utils::{ecdsa_input, profile::profile_func};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Start profiling...");
    let ecdsa_input = ecdsa_input();
    let input = to_vec(&ecdsa_input).unwrap();

    let env = ExecutorEnv::builder().write_slice(&input).build().unwrap();
    let mut exec = ExecutorImpl::from_elf(env, &risc0_benchmark_methods::ECDSA_VERIFY_ELF).unwrap();
    let session = exec.run().unwrap();

    let prover = get_prover_server(&ProverOpts::succinct()).unwrap();
    let ctx = VerifierContext::default();
    profile_func(
        || {
            let _ = prover.prove_session(&ctx, &session).unwrap();
        },
        "../.outputs/profiling/profile_risczero.pb",
    )?;

    Ok(())
}
