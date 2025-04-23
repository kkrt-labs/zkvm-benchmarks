#![cfg_attr(not(feature = "std"), no_main)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use guests::ecdsa;

use openvm::io::read;

// ANCHOR: main
openvm::entry!(main);

pub fn main() {
    let input: ecdsa::EcdsaVerifyInput = read();
    let is_ok = ecdsa::ecdsa_verify(input);
    if is_ok != true {
        panic!();
    }
}
// ANCHOR_END: main
