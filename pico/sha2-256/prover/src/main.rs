use pico_sdk::{client::DefaultProverClient, init_logger};
use sha256_lib::load_elf;
use std::time::{Duration, Instant};
use utils::benchmark;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let is_once = args.iter().any(|arg| arg == "--once");

    if is_once {
        println!("Profile mode activated: executing bench_hash(32) only...");

        // print current directory
        let current_dir = std::env::current_dir().unwrap();
        println!("Current directory: {:?}", current_dir);

        bench_hash(32);

        println!("Execution completed for bench_hash(32).");
    } else {
        let lengths = [32, 256, 512, 1024, 2048];
        benchmark(
            bench_hash,
            &lengths,
            "../../../benchmark_outputs/sha2-256_pico.csv",
            "num_bytes",
        );
    }
}

type BenchResult = (Duration, usize, usize);
fn bench_hash(num_bytes: usize) -> BenchResult {
    init_logger();
    let elf = load_elf("../app/elf/riscv32im-pico-zkvm-elf");
    let client = DefaultProverClient::new(&elf);
    let stdin_builder = client.get_stdin_builder();

    let input = vec![5u8; num_bytes];
    stdin_builder.borrow_mut().write(&input);

    let now = Instant::now();
    client.prove_fast().expect("Failed to generate proof");
    let duration = now.elapsed();

    println!("Successfully generated proof! Duration: {:?}", duration);

    (
        duration, 0x1000000,
        0x1000000, // placeholder values for proof size and instruction cycles
    )
}
