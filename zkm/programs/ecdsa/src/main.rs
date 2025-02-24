#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;

use k256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    EncodedPoint,
};

zkm_runtime::entrypoint!(main);

pub fn main() {
    let (encoded_point, message, signature): (EncodedPoint, Vec<u8>, Signature) = zkm_runtime::io::read();

    let is_ok = ecdsa_verify(encoded_point, message, signature);

    zkm_runtime::io::commit(&is_ok);
}

pub fn ecdsa_verify(
    encoded_verifying_key: EncodedPoint,
    message: Vec<u8>,
    signature: Signature
) -> bool {
    let verifying_key: VerifyingKey = VerifyingKey::from_encoded_point(&encoded_verifying_key).unwrap();

    verifying_key.verify(&message, &signature).is_ok()
}