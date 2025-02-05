#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::{read_private_input, write_output};
use sha2::{Digest, Sha256};
extern crate alloc;
use alloc::vec::Vec;

#[nexus_rt::main]
fn main() {
    let input = read_private_input::<Vec<u8>>().unwrap();
    let z = sha2(input);
    write_output::<[u8; 32]>(&z)
}

fn sha2(input: Vec<u8>) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().into()
}
