// ANCHOR: dependencies
use std::sync::Arc;
use std::time::Instant;

use eyre::Result;
use openvm_build::GuestOptions;
use openvm_sdk::{
    config::{AppConfig, SdkVmConfig},
    prover::AppProver,
    Sdk, StdIn,
};
use openvm_stark_sdk::config::FriParameters;
use utils::{bench::benchmark, bench::Metrics, ecdsa_input, metadata::ECDSA_INPUTS, size};

#[allow(unused_variables, unused_doc_comments)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    benchmark(
        benchmark_ecdsa,
        &ECDSA_INPUTS,
        "../.outputs/benchmark/ecdsa_openvm.csv",
    );

    Ok(())
}

fn benchmark_ecdsa(n: usize) -> Metrics {
    let mut metrics: Metrics = Metrics::new(n as usize);
    // ANCHOR: vm_config
    let vm_config = SdkVmConfig::builder()
        .system(Default::default())
        .rv32i(Default::default())
        .rv32m(Default::default())
        .io(Default::default())
        .build();
    // ANCHOR_END: vm_config

    // ANCHOR: build
    // 1. Build the VmConfig with the extensions needed.
    let sdk = Sdk;

    // 2a. Build the ELF with guest options and a target filter.
    let guest_opts = GuestOptions::default();
    let target_path = "ecdsa-guest";
    let elf = sdk
        .build(guest_opts, target_path, &Default::default())
        .unwrap();
    // ANCHOR_END: build

    // ANCHOR: transpilation
    // 3. Transpile the ELF into a VmExe
    let exe = sdk.transpile(elf.clone(), vm_config.transpiler()).unwrap();
    // ANCHOR_END: transpilation

    let input = ecdsa_input();

    // ANCHOR: execution
    // 4. Format your input into StdIn
    let mut stdin = StdIn::default();
    stdin.write(&input);

    // 5. Run the program
    let start = Instant::now();
    let _ = sdk
        .execute(exe.clone(), vm_config.clone(), stdin.clone())
        .unwrap();
    metrics.exec_duration = start.elapsed();
    metrics.elf_line_counts = elf.instructions.len() as u64;
    // ANCHOR_END: execution

    // ANCHOR: proof_generation
    // 6. Set app configuration
    let app_log_blowup = 2;
    let app_fri_params = FriParameters::standard_with_100_bits_conjectured_security(app_log_blowup);
    let app_config = AppConfig::new(app_fri_params, vm_config);

    // 7. Commit the exe
    let app_committed_exe = sdk.commit_app_exe(app_fri_params, exe).unwrap();

    // 8. Generate an AppProvingKey
    let app_pk = Arc::new(sdk.app_keygen(app_config).unwrap());

    // 9a. Generate a proof
    // let proof = sdk.generate_app_proof(app_pk.clone(), app_committed_exe.clone(), stdin.clone()).unwrap();
    // 9b. Generate a proof with an AppProver with custom fields
    let app_prover = AppProver::new(app_pk.app_vm_pk.clone(), app_committed_exe.clone())
        .with_program_name("test_program");
    let start: Instant = Instant::now();
    let proof = app_prover.generate_app_proof(stdin.clone());
    // ANCHOR_END: proof_generation
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = size(&proof);

    // ANCHOR: verification
    // 10. Verify your program
    let app_vk = app_pk.get_app_vk();
    let start = Instant::now();
    sdk.verify_app_proof(&app_vk, &proof).unwrap();
    metrics.verify_duration = start.elapsed();
    // ANCHOR_END: verification
    metrics
}
