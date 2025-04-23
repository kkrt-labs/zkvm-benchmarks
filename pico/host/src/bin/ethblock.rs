use pico_sdk::{client::DefaultProverClient, init_logger};
use std::time::Instant;
use utils::{benchmark, load_elf, BenchResult};

fn main() {
    let lengths = [1];
    benchmark(
        bench_ethblock,
        &lengths,
        "../../benchmark_outputs/ethblock_pico.csv",
    );
}

fn bench_ethblock(num_txs: usize) -> BenchResult {
    init_logger();
    let elf = load_elf("../ethblock-guest/elf/riscv32im-pico-zkvm-elf");
    let client = DefaultProverClient::new(&elf);
    let stdin_builder = client.get_stdin_builder();
    stdin_builder.borrow_mut().write(&num_txs);

    let now = Instant::now();
    client.prove_fast().expect("Failed to generate proof");
    let duration = now.elapsed();

    println!("Successfully generated proof! Duration: {:?}", duration);

    (
        duration, 0x0, 0x0, // placeholder values for proof size and instruction cycles
    )
}
