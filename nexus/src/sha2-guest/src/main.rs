#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use guests::sha2;
extern crate alloc;
use alloc::vec::Vec;

#[nexus_rt::main]
fn main(input: Vec<u8>) -> [u8; 32] {
    let result = sha2::sha2(&input);
    result
}
