use std::time::Instant;

use serde::{Deserialize, Serialize};
use utils::{ecdsa_input, size, bench::BenchResult};
use zkm_build::include_elf;
use zkm_sdk::{ProverClient, ZKMStdin};

mod tendermint;
pub use tendermint::bench_tendermint;

const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci");
const SHA2_ELF: &[u8] = include_elf!("sha2-bench");
const SHA2_CHAIN_ELF: &[u8] = include_elf!("sha2-chain");
const SHA3_CHAIN_ELF: &[u8] = include_elf!("sha3-chain");
const SHA3_ELF: &[u8] = include_elf!("sha3-bench");
const BIGMEM_ELF: &[u8] = include_elf!("bigmem");
const ECDSA_ELF: &[u8] = include_elf!("ecdsa-bench");
const ETHTRANSFER_ELF: &[u8] = include_elf!("transfer-eth");

pub fn init_logger() {
    std::env::set_var("RUST_LOG", "info");
    zkm_core_machine::utils::setup_logger();
}

fn bench_zkm(elf: &[u8], stdin: ZKMStdin) -> BenchResult {
    let client = ProverClient::cpu();
    let (pk, vk) = client.setup(elf);

    println!("benchmark start");
    let start = Instant::now();
    let proof = client.prove(&pk, stdin.clone()).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark end, duration: {:?}", duration.as_secs_f64());

    // client.verify(&proof, &vk).expect("verification failed");

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(elf, stdin).run().unwrap();
    println!(
        "executed program with {} cycles",
        report.total_instruction_count()
    );

    (
        duration,
        size(&proof),
        report.total_instruction_count() as usize,
    )
}

pub fn benchmark_sha2_chain(iters: u32) -> BenchResult {
    let mut stdin = ZKMStdin::new();
    let input = [5u8; 32];
    stdin.write(&input);
    stdin.write(&iters);
    bench_zkm(SHA2_CHAIN_ELF, stdin)
}

pub fn benchmark_sha3_chain(iters: u32) -> BenchResult {
    let mut stdin = ZKMStdin::new();
    let input = [5u8; 32];
    stdin.write(&input);
    stdin.write(&iters);
    bench_zkm(SHA3_CHAIN_ELF, stdin)
}

pub fn benchmark_sha2(num_bytes: usize) -> BenchResult {
    let input = vec![5u8; num_bytes];
    let mut stdin = ZKMStdin::new();
    stdin.write(&input);
    bench_zkm(SHA2_ELF, stdin)
}

pub fn benchmark_sha3(num_bytes: usize) -> BenchResult {
    let input = vec![5u8; num_bytes];
    let mut stdin = ZKMStdin::new();
    stdin.write(&input);
    bench_zkm(SHA3_ELF, stdin)
}

pub fn bench_fibonacci(n: u32) -> BenchResult {
    let mut stdin = ZKMStdin::new();
    stdin.write(&n);
    bench_zkm(FIBONACCI_ELF, stdin)
}

pub fn bench_bigmem(value: u32) -> BenchResult {
    let mut stdin = ZKMStdin::new();
    stdin.write(&value);
    bench_zkm(BIGMEM_ELF, stdin)
}

pub fn bench_ecdsa(_n: usize) -> BenchResult {
    let input = ecdsa_input();
    let mut stdin = ZKMStdin::new();
    stdin.write(&input);
    bench_zkm(ECDSA_ELF, stdin)
}

pub fn bench_ethtransfer(n: usize) -> BenchResult {
    let mut stdin = ZKMStdin::new();
    stdin.write(&n);
    bench_zkm(ETHTRANSFER_ELF, stdin)
}
