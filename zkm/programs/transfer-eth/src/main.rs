#![no_main]

use guests::ethtransfer;
extern crate alloc;

zkm_zkvm::entrypoint!(main);

pub fn main() {
    let input: usize = zkm_zkvm::io::read();
    let result = ethtransfer::ethtransfer(input);
    zkm_zkvm::io::commit::<bool>(&result);
}
