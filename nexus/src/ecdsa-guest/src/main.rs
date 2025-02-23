#![cfg_attr(target_arch = "riscv32", no_std, no_main)]
use k256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    elliptic_curve::sec1::EncodedPoint,
    Secp256k1,
};
use nexus_rt::{read_private_input, write_output};

extern crate alloc;
use alloc::vec::Vec;

#[nexus_rt::main]
fn main() {
    let (encoded_point, message, signature) = read_private_input::<(EncodedPoint<Secp256k1>, Vec<u8>, Signature)>().unwrap();
    let is_ok = ecdsa_verify(encoded_point, &message, signature);
    write_output::<bool>(&is_ok)
}

pub fn ecdsa_verify(encoded_point: EncodedPoint<Secp256k1>, message: &[u8], signature: Signature) -> bool {
    let verifying_key: VerifyingKey = VerifyingKey::from_encoded_point(&encoded_point).unwrap();

    verifying_key.verify(&message, &signature).is_ok()
}
