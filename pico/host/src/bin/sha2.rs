use pico_sdk::{client::DefaultProverClient, init_logger};
use std::time::Instant;
use utils::{bench::benchmark, load_elf, bench::BenchResult, metadata::SHA2_INPUTS};

fn main() {
    benchmark(
        bench_hash,
        &SHA2_INPUTS,
        "../../benchmark_outputs/sha2_pico.csv",
    );
}

fn bench_hash(num_bytes: usize) -> BenchResult {
    init_logger();
    let elf = load_elf("../sha2-guest/elf/riscv32im-pico-zkvm-elf");
    let client = DefaultProverClient::new(&elf);
    let stdin_builder = client.get_stdin_builder();

    let input = vec![5u8; num_bytes];
    stdin_builder.borrow_mut().write(&input);

    let now = Instant::now();
    client.prove_fast().expect("Failed to generate proof");
    let duration = now.elapsed();

    println!("Successfully generated proof! Duration: {:?}", duration);

    (
        duration, 0x0, 0x0, // placeholder values for proof size and instruction cycles
    )
}
