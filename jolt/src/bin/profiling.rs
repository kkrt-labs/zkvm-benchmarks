use utils::{ecdsa_input, profile::profile_func};

const TARGET_DIR: &str = "./ecdsa-guest";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting profiling...");

    // Set Fibonacci input
    let input = ecdsa_input();

    let program = ecdsa_guest::compile_ecdsa_verify(TARGET_DIR);
    let prover_preprocessing = ecdsa_guest::preprocess_prover_ecdsa_verify(&program);

    let prover = ecdsa_guest::build_prover_ecdsa_verify(program, prover_preprocessing);

    profile_func(
        || {
            let (_output, _proof) = prover(input.clone());
        },
        "../.outputs/profiling/profile_jolt.pb",
    )?;

    println!("Profiling complete!");
    Ok(())
}
