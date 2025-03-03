
use std::time::{Duration, Instant};

use zkm2_build::include_elf;
use zkm2_sdk::{ProverClient, ZKMStdin};
use utils::size;

const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci");
const SHA2_ELF: &[u8] = include_elf!("sha2-bench");
const SHA2_CHAIN_ELF: &[u8] = include_elf!("sha2-chain");
const SHA3_CHAIN_ELF: &[u8] = include_elf!("sha3-chain");
const SHA3_ELF: &[u8] = include_elf!("sha3-bench");
const BIGMEM_ELF: &[u8] = include_elf!("bigmem");
const TRANSFER_ETH_ELF: &[u8] = include_elf!("transfer-eth");

pub fn init_logger() {
    std::env::set_var("RUST_LOG", "info");
    zkm2_core_machine::utils::setup_logger();
}

pub fn benchmark_with_shard_size(func: fn(u32) -> (Duration, usize), iters: &[u32], shard_sizes: &[usize], file_name: &str, input_name: &str) {
    assert_eq!(iters.len(), shard_sizes.len());
    let mut info = Vec::new();
    for bench_i in 0..iters.len() {
        println!("benchmark_with_shard_size, bench_i: {}, shard_size: {}", bench_i, shard_sizes[bench_i]);
        std::env::set_var("SHARD_SIZE", format!("{}", shard_sizes[bench_i]));
        let duration_and_size = func(iters[bench_i]);
        info.push(duration_and_size);
        println!(
            "benchmark_with_shard_size end, duration: {:?}, shard_size: {}",
            duration_and_size.0.as_secs_f64(), duration_and_size.1,
        );
    }
    utils::write_csv(file_name, input_name, iters, &info);
}

pub fn benchmark_sha2_chain(iters: u32) -> (Duration, usize) {
    let client = ProverClient::cpu();
    let (pk, vk) = client.setup(SHA2_CHAIN_ELF);

    let mut stdin = ZKMStdin::new();
    let input = [5u8; 32];
    stdin.write(&input);
    stdin.write(&iters);

    println!("benchmark_sha2_chain start, iters: {}", iters);
    let start = Instant::now();
    let proof = client.prove(&pk, stdin).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_sha2_chain end, duration: {:?}", duration.as_secs_f64());

    client.verify(&proof, &vk).expect("verification failed");

    (duration, size(&proof))
}

pub fn benchmark_sha3_chain(iters: u32) -> (Duration, usize) {
    let client = ProverClient::cpu();
    let (pk, vk) = client.setup(SHA3_CHAIN_ELF);

    let mut stdin = ZKMStdin::new();
    let input = [5u8; 32];
    stdin.write(&input);
    stdin.write(&iters);

    println!("benchmark_sha3_chain start, iters: {}", iters);
    let start = Instant::now();
    let proof = client.prove(&pk, stdin).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_sha3 end, duration: {:?}", duration.as_secs_f64());

    client.verify(&proof, &vk).expect("verification failed");

    (duration, size(&proof))
}

pub fn benchmark_sha2(num_bytes: usize) -> (Duration, usize) {
    let client = ProverClient::cpu();
    let (pk, vk) = client.setup(SHA2_ELF);

    let mut stdin = ZKMStdin::new();
    let input = vec![5u8; num_bytes];
    stdin.write(&input);

    println!("benchmark_sha2 start, num_bytes: {}", num_bytes);
    let start = Instant::now();
    let proof = client.prove(&pk, stdin).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_sha2 end, duration: {:?}", duration.as_secs_f64());

    client.verify(&proof, &vk).expect("verification failed");

    (duration, size(&proof))
}

pub fn benchmark_sha3(num_bytes: usize) -> (Duration, usize) {
    let client = ProverClient::cpu();
    let (pk, vk) = client.setup(SHA3_ELF);

    let mut stdin = ZKMStdin::new();
    let input = vec![5u8; num_bytes];
    stdin.write(&input);

    println!("benchmark_sha3 start, num_bytes: {}", num_bytes);
    let start = Instant::now();
    let proof = client.prove(&pk, stdin).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_sha3 end, duration: {:?}", duration.as_secs_f64());

    client.verify(&proof, &vk).expect("verification failed");

    (duration, size(&proof))
}

pub fn bench_fibonacci(n: u32) -> (Duration, usize) {
    let client = ProverClient::cpu();
    let (pk, vk) = client.setup(FIBONACCI_ELF);

    let mut stdin = ZKMStdin::new();
    stdin.write(&n);

    println!("benchmark_fibonacci start, n: {}", n);
    let start = Instant::now();
    let proof = client.prove(&pk, stdin).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_fibonacc end, duration: {:?}", duration.as_secs_f64());

    client.verify(&proof, &vk).expect("verification failed");

    (duration, size(&proof))
}

pub fn bench_bigmem(value: u32) -> (Duration, usize) {
    let client = ProverClient::cpu();
    let (pk, vk) = client.setup(BIGMEM_ELF);

    let mut stdin = ZKMStdin::new();
    stdin.write(&value);

    println!("benchmark_bigmem start, value: {}", value);
    let start = Instant::now();
    let proof = client.prove(&pk, stdin).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_bigmem end, duration: {:?}", duration.as_secs_f64());

    client.verify(&proof, &vk).expect("verification failed");

    (duration, size(&proof))
}

pub fn bench_transfer_eth(n: usize) -> (Duration, usize) {
    let client = ProverClient::cpu();
    let (pk, vk) = client.setup(TRANSFER_ETH_ELF);

    let mut stdin = ZKMStdin::new();
    stdin.write(&n);

    println!("benchmark_transfer_eth start, n: {}", n);
    let start = Instant::now();
    let proof = client.prove(&pk, stdin).run().unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_transfer_eth end, duration: {:?}", duration.as_secs_f64());

    client.verify(&proof, &vk).expect("verification failed");

    (duration, size(&proof))
}
