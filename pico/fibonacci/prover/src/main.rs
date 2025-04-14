use fibonacci_lib::load_elf;
use pico_sdk::{client::DefaultProverClient, init_logger};
use std::time::{Duration, Instant};
use utils::benchmark;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let is_once = args.iter().any(|arg| arg == "--once");

    if is_once {
        println!("Profile mode activated: executing bench_fib(100) only...");

        // print current directory
        let current_dir = std::env::current_dir().unwrap();
        println!("Current directory: {:?}", current_dir);

        let result = bench_fib(100);
        println!("Result: {:?}", result);
    } else {
        let lengths = [10, 100, 1000, 10000, 100000];
        benchmark(
            bench_fib,
            &lengths,
            "../../../benchmark_outputs/fib_pico.csv",
            "n",
        );
    }
}

type BenchResult = (Duration, usize, usize);
fn bench_fib(n: u32) -> BenchResult {
    init_logger();
    let elf = load_elf("../app/elf/riscv32im-pico-zkvm-elf");
    let client = DefaultProverClient::new(&elf);
    let stdin_builder = client.get_stdin_builder();
    stdin_builder.borrow_mut().write(&n);

    println!("n: {}", n);

    let now = Instant::now();
    client.prove_fast().expect("Failed to generate proof");
    let duration = now.elapsed();

    println!("Successfully generated proof! Duration: {:?}", duration);

    (
        duration, 0x0, 0x0, // placeholder values for proof size and instruction cycles
    )
}
