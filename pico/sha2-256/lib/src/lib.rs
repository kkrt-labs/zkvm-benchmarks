use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Sha256Data {
    pub input: Vec<u8>,
    pub result: [u8; 32],
}

/// copied from sp1-turobo/sha256/lib/src/lib.rs
pub fn hash_sha256(input: Vec<u8>) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();

    result.into()
}

/// Loads an ELF file from the specified path.
pub fn load_elf(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|err| {
        panic!("Failed to load ELF file from {}: {}", path, err);
    })
}
