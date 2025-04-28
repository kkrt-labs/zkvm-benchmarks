use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use utils::{ecdsa_input, profile::profile_func};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const ELF: &[u8] = include_elf!("ecdsa-guest");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = ecdsa_input();
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&input);

    // Execute the program
    let (_output, report) = client.execute(ELF, &stdin).run().unwrap();
    println!("Program executed successfully.");

    // Record the number of cycles executed.
    println!("Number of cycles: {}", report.total_instruction_count());

    // Setup the program for proving.
    let (pk, _vk) = client.setup(ELF);

    profile_func(
        || {
            let _proof = client
                .prove(&pk, &stdin)
                .run()
                .expect("failed to generate proof");
        },
        "../profile_outputs/profile_sp1.pb",
    )?;

    println!("Profiling complete!");
    Ok(())
}
