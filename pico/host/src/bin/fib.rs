use pico_sdk::{client::DefaultProverClient, init_logger};
use std::time::Instant;
use utils::{bench::benchmark, load_elf, bench::BenchResult, metadata::FIBONACCI_INPUTS};

fn main() {
    benchmark(
        bench_fib,
        &FIBONACCI_INPUTS,
        "../../benchmark_outputs/fib_pico.csv",
    );
}

fn bench_fib(n: u32) -> BenchResult {
    init_logger();
    let elf = load_elf("../fibonacci-guest/elf/riscv32im-pico-zkvm-elf");
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
