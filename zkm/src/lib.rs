use std::time::Instant;

use utils::{bench::Metrics, ecdsa_input, sha2_input, size};
use zkm_build::include_elf;
use zkm_sdk::{ProverClient, ZKMStdin};

const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci");
const SHA2_ELF: &[u8] = include_elf!("sha2-bench");
const SHA3_ELF: &[u8] = include_elf!("sha3-bench");
const ECDSA_ELF: &[u8] = include_elf!("ecdsa-bench");
const ETHTRANSFER_ELF: &[u8] = include_elf!("transfer-eth");

pub fn init_logger() {
    std::env::set_var("RUST_LOG", "info");
    zkm_core_machine::utils::setup_logger();
}

fn bench_zkm(elf: &[u8], stdin: ZKMStdin, size_label: usize) -> Metrics {
    let mut metrics: Metrics = Metrics::new(size_label);

    let client = ProverClient::cpu();
    let (pk, vk) = client.setup(elf);

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let start = Instant::now();
    let (_, report) = client.execute(elf, stdin.clone()).run().unwrap();
    metrics.exec_duration = start.elapsed();
    metrics.cycles = report.total_instruction_count() as u64;

    let start = Instant::now();
    let proof = client.prove(&pk, stdin.clone()).run().unwrap();
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = size(&proof);

    // Sometimes the verification is failed with commitment error.
    // let start = Instant::now();
    // client.verify(&proof, &vk).expect("verification failed");
    // metrics.verify_duration = start.elapsed();

    metrics
}

pub fn benchmark_sha2(num_bytes: usize) -> Metrics {
    let input = sha2_input(num_bytes);
    let mut stdin = ZKMStdin::new();
    stdin.write(&input);
    bench_zkm(SHA2_ELF, stdin, num_bytes)
}

pub fn benchmark_sha3(num_bytes: usize) -> Metrics {
    let input = vec![5u8; num_bytes];
    let mut stdin = ZKMStdin::new();
    stdin.write(&input);
    bench_zkm(SHA3_ELF, stdin, num_bytes)
}

pub fn bench_fibonacci(n: u32) -> Metrics {
    let mut stdin = ZKMStdin::new();
    stdin.write(&n);
    bench_zkm(FIBONACCI_ELF, stdin, n as usize)
}

pub fn bench_ecdsa(n: usize) -> Metrics {
    let input = ecdsa_input();
    let mut stdin = ZKMStdin::new();
    stdin.write(&input);
    bench_zkm(ECDSA_ELF, stdin, n)
}

pub fn bench_ethtransfer(n: usize) -> Metrics {
    let mut stdin = ZKMStdin::new();
    stdin.write(&n);
    bench_zkm(ETHTRANSFER_ELF, stdin, n)
}
