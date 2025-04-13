use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct EthblockData {
    pub is_ok: bool,
}

pub fn trace_block(num_txs: usize) -> bool {
    ethblock_utils::trace_ethblock(num_txs)
}

/// Loads an ELF file from the specified path.
pub fn load_elf(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|err| {
        panic!("Failed to load ELF file from {}: {}", path, err);
    })
}
