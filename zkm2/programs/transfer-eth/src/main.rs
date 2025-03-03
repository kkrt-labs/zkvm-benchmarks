#![no_std]
#![no_main]
zkm2_zkvm::entrypoint!(main);

pub fn main() {
    let n = zkm2_zkvm::io::read::<usize>();
    let b: bool = revm_utils::transfer_eth_n_times(n);
    zkm2_zkvm::io::commit(&b);
}
