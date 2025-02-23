#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use k256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    EncodedPoint,
};

#[jolt::provable(stack_size = 1000_000, memory_size = 10000000, max_input_size = 10000000)]
pub fn ecdsa_verify(
    encoded_verifying_key: EncodedPoint,
    message: &[u8],
    signature: Signature
) -> bool {
    let verifying_key: VerifyingKey = VerifyingKey::from_encoded_point(&encoded_verifying_key).unwrap();

    verifying_key.verify(message, &signature).is_ok()
}
