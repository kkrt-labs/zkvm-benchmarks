#![no_main]

use guests::ecdsa;
extern crate alloc;

zkm_zkvm::entrypoint!(main);

pub fn main() {
    let input: ecdsa::EcdsaVerifyInput = zkm_zkvm::io::read();
    let result = ecdsa::ecdsa_verify(input);
    zkm_zkvm::io::commit::<bool>(&result);
}
