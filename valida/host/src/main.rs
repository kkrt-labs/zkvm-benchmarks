use std::fs;
use std::path::Path;
use std::time::Instant;
use tempfile::NamedTempFile;
use tmpfile_helper::*;
use utils::bench::{benchmark, Metrics};
use utils::metadata::FIBONACCI_INPUTS;
#[cfg(target_arch = "aarch64")]
use valida_vm_api_linux_arm::*;
#[cfg(target_arch = "x86_64")]
use valida_vm_api_linux_x86::*;

fn main() {
    benchmark(
        bench_fib,
        &FIBONACCI_INPUTS,
        "../../.outputs/benchmark/fib_valida.csv",
    );
}

fn bench_fib(n: u32) -> Metrics {
    let mut metrics = Metrics::new(n as usize);
    let program =
        Path::new("../fibonacci/target/valida-unknown-baremetal-gnu/release/").join("fibonacci");

    let valida = create_valida().unwrap();

    let stdin = bytes_to_temp_file(n.to_string().as_bytes()).unwrap();
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
    println!("All checks completed successfully for n = {}.", n);

    metrics
}
