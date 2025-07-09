#![cfg_attr(not(feature = "std"), no_main)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// ANCHOR: imports
use alloc::vec::Vec;
use core::hint::black_box;

use openvm_sha256_guest::sha256;
use openvm::io::{read, reveal_u32};
// ANCHOR_END: imports

// ANCHOR: main
openvm::entry!(main);

pub fn main() {
    let input: Vec<u8> = read();
    let output = sha256(&black_box(input));
    reveal_u32(output[0], 0);
}
// ANCHOR_END: main
