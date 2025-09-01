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
    // Reveal first 4 bytes of hash as u32
    let first_u32 = u32::from_le_bytes([
        output[0],
        output[1],
        output[2],
        output[3],
    ]);
    reveal_u32(first_u32, 0);
}
// ANCHOR_END: main
