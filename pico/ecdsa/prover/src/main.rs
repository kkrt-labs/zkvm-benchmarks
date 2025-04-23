use ecdsa_lib::{load_elf, EcdsaData};
use k256::{ecdsa::Signature, elliptic_curve::sec1::EncodedPoint, Secp256k1};
use pico_sdk::{client::DefaultProverClient, init_logger};
use std::time::{Duration, Instant};
use utils::benchmark;

const MESSAGE_HEX: &[u8] = include_bytes!("../../../../utils/ecdsa_signature/message.txt");
const KEY_HEX: &[u8] = include_bytes!("../../../../utils/ecdsa_signature/verifying_key.txt");
const SIGNATURE_HEX: &[u8] = include_bytes!("../../../../utils/ecdsa_signature/signature.txt");

fn main() {
    benchmark(
        bench_ecdsa,
        &["1"],
        "../../../benchmark_outputs/ecdsa_pico.csv",
    );
}

type BenchResult = (Duration, usize, usize);
fn bench_ecdsa(_fixed: &str) -> BenchResult {
    init_logger();
    let elf = load_elf("../app/elf/riscv32im-pico-zkvm-elf");
    let client = DefaultProverClient::new(&elf);
    let stdin_builder = client.get_stdin_builder();

    let message_raw = hex::decode(MESSAGE_HEX).expect("Failed to decode message");
    let key_raw = hex::decode(KEY_HEX).expect("Failed to decode key");
    let signature_raw = hex::decode(SIGNATURE_HEX).expect("Failed to decode signature");

    let encoded_point =
        EncodedPoint::<Secp256k1>::from_bytes(&key_raw).expect("Failed to decode encoded point");
    let signature = Signature::from_slice(&signature_raw).expect("Failed to decode signature");

    let input = EcdsaData {
        encoded_point,
        message: message_raw,
        signature,
    };
    stdin_builder.borrow_mut().write(&input);

    let now = Instant::now();
    client.prove_fast().expect("Failed to generate proof");
    let duration = now.elapsed();

    println!("Successfully generated proof! Duration: {:?}", duration);

    (
        duration, 0x0, 0x0, // placeholder values for proof size and instruction cycles
    )
}
