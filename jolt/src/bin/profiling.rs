use utils::{ecdsa_input, profile::profile_func};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting profiling...");

    // Set Fibonacci input
    let input = ecdsa_input();

    // Get the Fibonacci function from the guest code
    let (prove_ecdsa_verify, _verify_ecdsa_verify) = ecdsa_guest::build_ecdsa_verify();

    profile_func(
        || {
            let (_output, _proof) = prove_ecdsa_verify(input);
        },
        "../profile_outputs/profile_jolt.pb",
    )?;

    println!("Profiling complete!");
    Ok(())
}
