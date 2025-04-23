#![no_main]
sp1_zkvm::entrypoint!(main);

use guests::ethtransfer;

fn main() {
    let num = sp1_zkvm::io::read::<usize>();

    let b = ethtransfer::ethtransfer(num);
    assert!(b);

    sp1_zkvm::io::commit(&b);
}
