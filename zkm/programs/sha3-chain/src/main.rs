#![no_std]
#![no_main]

use sha3::{Digest, Keccak256};

zkm_runtime::entrypoint!(main);

pub fn main() {
    let input: [u8; 32] = zkm_runtime::io::read();
    let num_iters: u32 = zkm_runtime::io::read();
    let mut hash = input;
    for _ in 0..num_iters {
        let mut hasher = Keccak256::new();
        hasher.update(input);
        let res = &hasher.finalize();
        hash = Into::<[u8; 32]>::into (*res);
    }

    zkm_runtime::io::commit::<[u8; 32]>(&hash.into());
}
