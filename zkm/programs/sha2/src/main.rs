#![no_main]

use guests::sha2;
extern crate alloc;

zkm_zkvm::entrypoint!(main);

pub fn main() {
    let input: Vec<u8> = zkm_zkvm::io::read();
    let result = sha2::sha2(&input);
    zkm_zkvm::io::commit::<[u8; 32]>(&result.into());
}
