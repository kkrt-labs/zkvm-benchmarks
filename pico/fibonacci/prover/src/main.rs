use fibonacci_lib::{FibonacciData, fibonacci, load_elf};
use pico_sdk::{client::DefaultProverClient, init_logger};
use std::time::Duration;
use utils::{benchmark, size};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let is_once = true; //args.iter().any(|arg| arg == "--once");

    if is_once {
        println!("Profile mode activated: executing bench_fib(100) only...");

        // print current directory
        let current_dir = std::env::current_dir().unwrap();
        println!("Current directory: {:?}", current_dir);

        let result = bench_fib(100);
        println!("Result: {:?}", result);
    } else {
        let lengths = [10, 50, 90];
        benchmark(
            bench_fib,
            &lengths,
            "../benchmark_outputs/fib_pico.csv",
            "n",
        );
    }
}

type BenchResult = (Duration, usize, usize);
fn bench_fib(n: u32) -> BenchResult {
    init_logger();
    let elf = load_elf("../elf/riscv32im-pico-zkvm-elf");
    let client = DefaultProverClient::new(&elf);
    let stdin_builder = client.get_stdin_builder();
    stdin_builder.borrow_mut().write(&n);

    println!("n: {}", n);

    let start = std::time::Instant::now();
    let proof = client.prove_fast().expect("Failed to generate proof");
    let end = std::time::Instant::now();
    let duration = end.duration_since(start);

    println!("Successfully generated proof!");

    (
        duration, 0, 0, // Placeholder for cycles
    )
}
