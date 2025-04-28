use k256::ecdsa::{signature::Signer, Signature, SigningKey};
use rand_core::OsRng;
use risc0_zkvm::serde::to_vec;
use risc0_zkvm::{get_prover_server, ExecutorEnv, ExecutorImpl, ProverOpts, VerifierContext};
use utils::profile::profile_func;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Start profiling fibonacci(100)...");
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = signing_key.verifying_key().to_encoded_point(true);
    let message = b"This is a message that will be signed, and verified within the zkVM".to_vec();
    let signature: Signature = signing_key.sign(&message);

    let guest_input = to_vec(&(1, verifying_key, message, signature)).unwrap();

    let env = ExecutorEnv::builder()
        .write_slice(&guest_input)
        .build()
        .unwrap();
    let mut exec = ExecutorImpl::from_elf(env, &risc0_benchmark_methods::ECDSA_VERIFY_ELF).unwrap();
    let session = exec.run().unwrap();

    let prover = get_prover_server(&ProverOpts::succinct()).unwrap();
    let ctx = VerifierContext::default();
    profile_func(
        || {
            let _ = prover.prove_session(&ctx, &session).unwrap();
        },
        "../profile_outputs/profile_risczero.pb",
    )?;

    Ok(())
}
