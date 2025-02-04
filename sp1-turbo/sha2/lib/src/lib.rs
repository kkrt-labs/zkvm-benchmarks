use alloy_sol_types::sol;

extern crate alloc;
use sha2::{Digest, Sha256};

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        uint8[] input;
        uint8[32] result;
    }
}

/// Compute the n'th fibonacci number (wrapping around on overflows), using normal Rust code.
pub fn sha2(input: Vec<u8>) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();

    result.into()
}
