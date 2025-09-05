// ANCHOR: dependencies
use std::time::Instant;

use eyre::Result;
use openvm_build::GuestOptions;
use openvm_sdk::codec::Encode;
use openvm_sdk::{
    config::{AppConfig, SdkVmConfig},
    prover::verify_app_proof,
    Sdk, StdIn,
};
use openvm_stark_sdk::config::FriParameters;
use utils::{bench::benchmark, bench::Metrics, metadata::SHA2_INPUTS, sha2_input};

// ANCHOR_END: dependencies

#[allow(unused_variables, unused_doc_comments)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    benchmark(
        bench_openvm_sha256,
        &SHA2_INPUTS,
        "../.outputs/benchmark/sha2_openvm.csv",
    );

    Ok(())
}

fn bench_openvm_sha256(num_bytes: usize) -> Metrics {
    let mut metrics: Metrics = Metrics::new(num_bytes as usize);

    // ANCHOR: vm_config
    let vm_config = SdkVmConfig::builder()
        .system(Default::default())
        .rv32i(Default::default())
        .rv32m(Default::default())
        .io(Default::default())
        .sha256(Default::default())
        .build();
    // ANCHOR_END: vm_config

    // ANCHOR: build
    // 1. Set app configuration
    let log_blowup_factor = 2;
    let app_fri_params =
        FriParameters::standard_with_100_bits_conjectured_security(log_blowup_factor);
    let app_config = AppConfig::new(app_fri_params, vm_config.clone());

    // 2. Build the SDK with the app config
    let sdk = Sdk::new(app_config.clone()).unwrap();

    // 3. Build the ELF with guest options and a target filter.
    let guest_opts = GuestOptions::default();
    let target_path = "sha2-guest";
    let elf = sdk
        .build(guest_opts, target_path, &Default::default(), None)
        .unwrap();
    // ANCHOR_END: build

    // ANCHOR: transpilation
    // 4. Convert the ELF into a VmExe
    let exe = sdk.convert_to_exe(elf.clone()).unwrap();
    // ANCHOR_END: transpilation

    // ANCHOR: execution
    // 5. Format your input into StdIn
    let input = sha2_input(num_bytes);
    let mut stdin = StdIn::default();
    stdin.write(&input);

    // 6. Run the program
    let start = Instant::now();
    let _ = sdk.execute(exe.clone(), stdin.clone()).unwrap();
    metrics.exec_duration = start.elapsed();
    // ANCHOR_END: execution

    // ANCHOR: proof_generation
    // 7. Generate a proof using app_prover
    let mut app_prover = sdk
        .app_prover(elf.clone())
        .unwrap()
        .with_program_name("sha2");
    let start = Instant::now();
    let proof = app_prover.prove(stdin.clone()).unwrap();
    // ANCHOR_END: proof_generation
    metrics.proof_duration = start.elapsed();
    let proof_bytes = proof.encode_to_vec().unwrap();
    let proof_size = proof_bytes.len();
    metrics.proof_bytes = proof_size;

    // ANCHOR: verification
    // 8. Get the app verifying key for verification
    let (_, app_vk) = sdk.app_keygen();

    // 9. Verify your program
    let start = Instant::now();
    verify_app_proof(&app_vk, &proof).unwrap();
    metrics.verify_duration = start.elapsed();
    // ANCHOR_END: verification

    metrics
}
