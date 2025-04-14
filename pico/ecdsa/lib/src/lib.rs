use k256::{
    Secp256k1,
    ecdsa::{Signature, VerifyingKey, signature::Verifier},
    elliptic_curve::sec1::EncodedPoint,
};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct EcdsaData {
    pub encoded_point: EncodedPoint<Secp256k1>,
    pub message: Vec<u8>,
    pub signature: Signature,
}

pub fn ecdsa_verify(
    encoded_point: EncodedPoint<Secp256k1>,
    message: &[u8],
    signature: Signature,
) -> bool {
    let verifying_key: VerifyingKey = VerifyingKey::from_encoded_point(&encoded_point).unwrap();

    verifying_key.verify(message, &signature).is_ok()
}

/// Loads an ELF file from the specified path.
pub fn load_elf(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|err| {
        panic!("Failed to load ELF file from {}: {}", path, err);
    })
}
