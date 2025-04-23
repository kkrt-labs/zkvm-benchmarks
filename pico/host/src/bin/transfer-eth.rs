use pico_sdk::{client::DefaultProverClient, init_logger};
use std::time::Instant;
use utils::{benchmark, load_elf, BenchResult, ETHTRANSFER_INPUTS};

fn main() {
    benchmark(
        bench_transfer_eth,
        &ETHTRANSFER_INPUTS,
        "../../benchmark_outputs/ethtransfer_pico.csv",
    );
}

fn bench_transfer_eth(n: usize) -> BenchResult {
    init_logger();
    let elf = load_elf("../transfer-eth-guest/elf/riscv32im-pico-zkvm-elf");
    let client = DefaultProverClient::new(&elf);
    let stdin_builder = client.get_stdin_builder();
    stdin_builder.borrow_mut().write(&n);

    let now = Instant::now();
    client.prove_fast().expect("Failed to generate proof");
    let duration = now.elapsed();

    println!("Successfully generated proof! Duration: {:?}", duration);

    (
        duration, 0x0, 0x0, // placeholder values for proof size and instruction cycles
    )
}
