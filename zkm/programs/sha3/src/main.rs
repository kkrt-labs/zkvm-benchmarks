#![no_main]

use sha3::{Digest, Keccak256};
extern crate alloc;

zkm_zkvm::entrypoint!(main);

pub fn main() {
    let input: Vec<u8> = zkm_zkvm::io::read();

    let mut hasher = Keccak256::new();
    hasher.update(input);
    let result = hasher.finalize();

    zkm_zkvm::io::commit::<[u8; 32]>(&result.into());
}
