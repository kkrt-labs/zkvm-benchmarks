use std::fs;
use std::path::Path;
use std::time::Instant;
use tempfile::NamedTempFile;
use utils::bench::{benchmark, Metrics};
use utils::metadata::SHA2_INPUTS;
use utils::sha2_input;
use valida_bench_host::utils::bytes_to_temp_file;
#[cfg(target_arch = "aarch64")]
use valida_vm_api_linux_arm::*;
#[cfg(target_arch = "x86_64")]
use valida_vm_api_linux_x86::*;

fn main() {
    benchmark(
        bench_valida_sha256,
        &SHA2_INPUTS,
        "../../.outputs/benchmark/sha2_valida.csv",
    );
}

fn bench_valida_sha256(num_bytes: usize) -> Metrics {
    let mut metrics = Metrics::new(num_bytes);
    let program =
        Path::new("../sha2/target/valida-unknown-baremetal-gnu/release/").join("sha2-valida");

    let valida = create_valida().unwrap();

    let input_bytes = sha2_input(num_bytes);
    let stdin = bytes_to_temp_file(&input_bytes).unwrap();
    let stdout = NamedTempFile::new().unwrap();

    let start = Instant::now();
    let run_status = valida.run(
        &program,
        stdout.as_ref(),
        stdin.as_ref(),
        Default::default(),
    );
    metrics.exec_duration = start.elapsed();

    assert_eq!(run_status, RunStatus::TerminatedWithStop);

    let proof = NamedTempFile::new().unwrap();

    let start = Instant::now();
    let prove_status = valida.prove(
        &program,
        proof.as_ref(),
        stdin.as_ref(),
        Default::default(),
        Default::default(),
    );
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = fs::metadata(proof.path()).unwrap().len() as usize;

    assert_eq!(prove_status, ProveStatus::Success);

    let start = Instant::now();
    let verify_status_correct_statement = valida.verify(
        &program,
        proof.as_ref(),
        stdout.as_ref(),
        Default::default(),
        Default::default(),
    );
    metrics.verify_duration = start.elapsed();

    assert_eq!(verify_status_correct_statement, VerifyStatus::Success);
    println!("All checks completed successfully for {} bytes.", num_bytes);

    metrics
}
