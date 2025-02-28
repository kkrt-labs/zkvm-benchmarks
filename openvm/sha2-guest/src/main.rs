#![cfg_attr(not(feature = "std"), no_main)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// ANCHOR: imports
use alloc::vec::Vec;
use core::hint::black_box;

use hex::FromHex;
use openvm_sha256_guest::sha256;
// ANCHOR_END: imports

use openvm::io::{read, reveal};

// ANCHOR: main
openvm::entry!(main);

pub fn main() {
    let input: Vec<u8> = read();
    let output = sha256(&black_box(input));
}
// ANCHOR_END: main