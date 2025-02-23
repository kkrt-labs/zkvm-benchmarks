use std::time::{Duration, Instant};
use jolt::Serializable;
use utils::benchmark;

use k256::{
    ecdsa::Signature,
    elliptic_curve::sec1::EncodedPoint,
    Secp256k1,
};
type BenchResult = (Duration, usize, usize);

fn main() {
    let csv_file = format!(
        "../benchmark_outputs/ecdsa_jolt{}{}.csv",
        if cfg!(feature = "icicle") { "_gpu" } else { "" },
        ""
    );

    let lengths = [1];
    benchmark(
        bench_ecdsa,
        &lengths,
        &csv_file,
        "n",
    );
}

const MESSAGE: &[u8] = include_bytes!("../../../helper/ecdsa_signature/message.txt");
const KEY: &[u8] = include_bytes!("../../../helper/ecdsa_signature/verifying_key.txt");
const SIGNATURE: &[u8] = include_bytes!("../../../helper/ecdsa_signature/signature.txt");

fn bench_ecdsa(_dummy: usize) -> BenchResult {

    let (prove_ecdsa_verify, _verify_ecdsa_verify) = ecdsa_guest::build_ecdsa_verify();

    let message = hex::decode(MESSAGE).expect("Failed to decode hex of 'message'");

    let encoded_point = EncodedPoint::<Secp256k1>::from_bytes(
        &hex::decode(KEY).expect("Failed to decode hex of 'verifying_key'"),
    )
    .expect("Invalid encoded verifying_key bytes");

    let bytes = hex::decode(SIGNATURE).expect("Failed to decode hex of 'signature'");
    let signature = Signature::from_slice(&bytes).expect("Invalid signature bytes");

    let program_summary = ecdsa_guest::analyze_ecdsa_verify(encoded_point, &message.clone(), signature);

    let start = Instant::now();
    let (_output, proof) = prove_ecdsa_verify(encoded_point, &message, signature);
    let end = Instant::now();

    (
        end.duration_since(start),
        proof.size().unwrap(),
        program_summary.processed_trace.len(),
    )
}
