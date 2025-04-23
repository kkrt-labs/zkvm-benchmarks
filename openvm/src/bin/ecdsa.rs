// ANCHOR: dependencies
use std::sync::Arc;
use std::time::{Duration, Instant};

use eyre::Result;
use openvm_build::GuestOptions;
use openvm_sdk::{
    config::{AppConfig, SdkVmConfig},
    prover::AppProver,
    Sdk, StdIn,
};
use openvm_stark_sdk::config::FriParameters;
use serde::{Deserialize, Serialize};
use utils::{benchmark, size};

use k256::{ecdsa::Signature, elliptic_curve::sec1::EncodedPoint, Secp256k1};

// ANCHOR_END: dependencies

type BenchResult = (Duration, usize, usize);

const MESSAGE: &[u8] = include_bytes!("../../../utils/ecdsa_signature/message.txt");
const KEY: &[u8] = include_bytes!("../../../utils/ecdsa_signature/verifying_key.txt");
const SIGNATURE: &[u8] = include_bytes!("../../../utils/ecdsa_signature/signature.txt");

#[derive(Serialize, Deserialize)]
pub struct SomeStruct {
    pub encoded_verifying_key: EncodedPoint<Secp256k1>,
    pub message: Vec<u8>,
    pub signature: Signature,
}

#[allow(unused_variables, unused_doc_comments)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ns = [1];
    benchmark(
        benchmark_ecdsa,
        &ns,
        "../benchmark_outputs/ecdsa_openvm.csv",
    );

    Ok(())
}

fn benchmark_ecdsa(_n: u32) -> BenchResult {
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
    let exe = sdk.transpile(elf, vm_config.transpiler()).unwrap();
    // ANCHOR_END: transpilation

    let message = hex::decode(MESSAGE).expect("Failed to decode hex of 'message'");

    let encoded_point = EncodedPoint::<Secp256k1>::from_bytes(
        &hex::decode(KEY).expect("Failed to decode hex of 'verifying_key'"),
    )
    .expect("Invalid encoded verifying_key bytes");

    let bytes = hex::decode(SIGNATURE).expect("Failed to decode hex of 'signature'");
    let signature = Signature::from_slice(&bytes).expect("Invalid signature bytes");

    let input = SomeStruct {
        encoded_verifying_key: encoded_point,
        message: message,
        signature: signature,
    };

    // ANCHOR: execution
    // 4. Format your input into StdIn
    let mut stdin = StdIn::default();
    stdin.write(&input);

    // 5. Run the program
    let output = sdk
        .execute(exe.clone(), vm_config.clone(), stdin.clone())
        .unwrap();
    println!("public values output: {:?}", output);
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
    let start = Instant::now();
    let app_prover = AppProver::new(app_pk.app_vm_pk.clone(), app_committed_exe.clone())
        .with_program_name("test_program");
    let proof = app_prover.generate_app_proof(stdin.clone());
    // ANCHOR_END: proof_generation
    let end = Instant::now();

    // ANCHOR: verification
    // 10. Verify your program
    let app_vk = app_pk.get_app_vk();
    sdk.verify_app_proof(&app_vk, &proof).unwrap();
    // ANCHOR_END: verification

    (end.duration_since(start), size(&proof), 0x0)
}
