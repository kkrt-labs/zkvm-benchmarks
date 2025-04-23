use pico_sdk::{client::DefaultProverClient, init_logger};
use std::time::Instant;
use utils::{benchmark, ecdsa_input, load_elf, BenchResult, ECDSA_INPUTS};

fn main() {
    benchmark(
        bench_ecdsa,
        &ECDSA_INPUTS,
        "../../benchmark_outputs/ecdsa_pico.csv",
    );
}

fn bench_ecdsa(_fixed: usize) -> BenchResult {
    init_logger();
    let elf = load_elf("../ecdsa-guest/elf/riscv32im-pico-zkvm-elf");
    let client = DefaultProverClient::new(&elf);
    let stdin_builder = client.get_stdin_builder();

    let input = ecdsa_input();
    stdin_builder.borrow_mut().write(&input);

    let now = Instant::now();
    client.prove_fast().expect("Failed to generate proof");
    let duration = now.elapsed();

    println!("Successfully generated proof! Duration: {:?}", duration);

    (
        duration, 0x0, 0x0, // placeholder values for proof size and instruction cycles
    )
}
