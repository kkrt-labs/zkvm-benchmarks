use pico_sdk::{client::DefaultProverClient, init_logger};
use std::{
    fs,
    time::{Duration, Instant},
};
use utils::benchmark;

pub fn load_elf(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|err| {
        panic!("Failed to load ELF file from {}: {}", path, err);
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let is_once = args.iter().any(|arg| arg == "--once");

    if is_once {
        println!("Profile mode activated: executing bench_transfer_eth(10) only...");

        // print current directory
        let current_dir = std::env::current_dir().unwrap();
        println!("Current directory: {:?}", current_dir);

        let result = bench_transfer_eth(10);
        println!("Result: {:?}", result);
    } else {
        let num_transfers = [1, 10, 100];
        benchmark(
            bench_transfer_eth,
            &num_transfers,
            "../../../benchmark_outputs/transfer_eth_pico.csv",
            "n",
        );
    }
}

type BenchResult = (Duration, usize, usize);
fn bench_transfer_eth(n: usize) -> BenchResult {
    init_logger();
    let elf = load_elf("../app/elf/riscv32im-pico-zkvm-elf");
    let client = DefaultProverClient::new(&elf);
    let stdin_builder = client.get_stdin_builder();
    stdin_builder.borrow_mut().write(&n);

    let now = Instant::now();
    client.prove_fast().expect("Failed to generate proof");
    let duration = now.elapsed();

    println!("Successfully generated proof! Duration: {:?}", duration);

    (
        duration, 0x1000000,
        0x1000000, // placeholder values for proof size and instruction cycles
    )
}
