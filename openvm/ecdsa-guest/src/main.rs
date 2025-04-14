#![cfg_attr(not(feature = "std"), no_main)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// ANCHOR: imports
use alloc::vec::Vec;
use k256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    EncodedPoint,
};
use serde::{Deserialize, Serialize};
// ANCHOR_END: imports

use openvm::io::read;

// ANCHOR: main
openvm::entry!(main);

#[derive(Serialize, Deserialize)]
pub struct SomeStruct {
    pub encoded_verifying_key: EncodedPoint,
    pub message: Vec<u8>,
    pub signature: Signature,
}

pub fn main() {
    let input: SomeStruct = read();
    let verifying_key: VerifyingKey =
        VerifyingKey::from_encoded_point(&input.encoded_verifying_key).unwrap();
    let is_ok = verifying_key
        .verify(&input.message, &input.signature)
        .is_ok();
    if is_ok != true {
        panic!();
    }
}
// ANCHOR_END: main
