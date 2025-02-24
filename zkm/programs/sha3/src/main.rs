#![no_std]
#![no_main]

use sha3::{Digest, Keccak256};
extern crate alloc;
use alloc::vec::Vec;

zkm_runtime::entrypoint!(main);

pub fn main() {
    let input: Vec<u8> = zkm_runtime::io::read();

    let mut hasher = Keccak256::new();
    hasher.update(input);
    let result = hasher.finalize();

    zkm_runtime::io::commit::<[u8; 32]>(&result.into());
}
