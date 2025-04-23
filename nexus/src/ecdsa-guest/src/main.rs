#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use guests::ecdsa;
extern crate alloc;
use alloc::vec::Vec;

#[nexus_rt::main]
fn main(input: ecdsa::EecsaVerifyInput) -> bool {
    let result = ecdsa::ecdsa_verify(input);
    result
}
