use guests::ecdsa::EcdsaVerifyInput;
use k256::{ecdsa::Signature, elliptic_curve::sec1::EncodedPoint, Secp256k1};
use std::{
    fmt::Display,
    fs::File,
    io::Write,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use serde::Serialize;

pub type BenchResult = (Duration, usize, usize);
type BenchMetrics = (Duration, usize, usize, usize);

pub const FIBONACCI_INPUTS: [u32; 3] = [10, 100, 1000];
pub const SHA2_INPUTS: [usize; 3] = [32, 256, 512];
pub const ECDSA_INPUTS: [usize; 1] = [1];
pub const ETHTRANSFER_INPUTS: [usize; 2] = [1, 10];

fn get_current_memory_usage() -> Result<usize, std::io::Error> {
    let content = std::fs::read_to_string("/proc/self/status")?;
    for line in content.lines() {
        if line.starts_with("VmRSS:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(kb) = parts[1].parse::<usize>() {
                    return Ok(kb / 1024); // KB -> MB
                }
            }
        }
    }
    Ok(0)
}

pub fn ecdsa_input() -> EcdsaVerifyInput<'static> {
    const MESSAGE: &[u8] = include_bytes!("../../utils/ecdsa_signature/message.txt");
    const KEY: &[u8] = include_bytes!("../../utils/ecdsa_signature/verifying_key.txt");
    const SIGNATURE: &[u8] = include_bytes!("../../utils/ecdsa_signature/signature.txt");

    // Use a static variable to store the decoded message so it has a 'static lifetime
    static MESSAGE_DECODED: once_cell::sync::Lazy<Vec<u8>> = once_cell::sync::Lazy::new(|| {
        hex::decode(MESSAGE).expect("Failed to decode hex of 'message'")
    });

    let encoded_point = EncodedPoint::<Secp256k1>::from_bytes(
        &hex::decode(KEY).expect("Failed to decode hex of 'verifying_key'"),
    )
    .expect("Invalid encoded verifying_key bytes");

    let bytes = hex::decode(SIGNATURE).expect("Failed to decode hex of 'signature'");
    let signature = Signature::from_slice(&bytes).expect("Invalid signature bytes");

    EcdsaVerifyInput {
        encoded_point,
        message: &*MESSAGE_DECODED,
        signature,
    }
}

fn measure_peak_memory<R, F: FnOnce() -> R>(func: F) -> (R, usize) {
    let peak = Arc::new(AtomicUsize::new(0));
    let stop = Arc::new(AtomicBool::new(false));

    let peak_clone = Arc::clone(&peak);
    let stop_clone = Arc::clone(&stop);
    let monitor = thread::spawn(move || {
        while !stop_clone.load(Ordering::Relaxed) {
            if let Ok(mem) = get_current_memory_usage() {
                peak_clone.fetch_max(mem, Ordering::Relaxed);
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    let result = func();

    stop.store(true, Ordering::Relaxed);
    monitor.join().unwrap();

    (result, peak.load(Ordering::Relaxed))
}

pub fn benchmark<T: Display + Clone, F>(func: F, inputs: &[T], file: &str)
where
    F: Fn(T) -> BenchResult,
{
    let mut results = Vec::new();
    for input in inputs {
        let ((duration, size, cycles), peak_memory) = measure_peak_memory(|| func(input.clone()));
        results.push((duration, size, cycles, peak_memory));
    }

    write_csv(file, inputs, &results);
}

pub fn write_csv<T: Display>(file: &str, inputs: &[T], results: &[BenchMetrics]) {
    let mut file = File::create(file).unwrap();
    file.write_all(
        format!("n,cycles,prover time (ms),proof size (bytes),peak memory (MB)\n").as_bytes(),
    )
    .unwrap();
    inputs
        .iter()
        .zip(results)
        .for_each(|(input, (duration, size, cycles, peak_memory))| {
            file.write_all(
                format!(
                    "{},{},{},{},{}\n",
                    input,
                    cycles,
                    duration.as_millis(),
                    size,
                    peak_memory
                )
                .as_bytes(),
            )
            .unwrap();
        });
}

pub fn size<T: Serialize>(item: &T) -> usize {
    bincode::serialized_size(item).unwrap() as usize
}
