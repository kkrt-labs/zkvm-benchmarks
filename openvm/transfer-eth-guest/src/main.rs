#![cfg_attr(not(feature = "std"), no_main)]
#![cfg_attr(not(feature = "std"), no_std)]

use openvm::io::{read, reveal};

openvm::entry!(main);

pub fn main() {
    let n: usize = read();
    let b = revm_utils::transfer_eth_n_times(n);
    reveal(b as u32, 0);
}
