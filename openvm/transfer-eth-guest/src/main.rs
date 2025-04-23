#![cfg_attr(not(feature = "std"), no_main)]
#![cfg_attr(not(feature = "std"), no_std)]

use guests::ethtransfer;
use openvm::io::{read, reveal};

openvm::entry!(main);

pub fn main() {
    let n: usize = read();
    let b = ethtransfer::ethtransfer(n);
    reveal(b as u32, 0);
}
