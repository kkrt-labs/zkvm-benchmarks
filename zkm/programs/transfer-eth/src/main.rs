#![no_std]
#![no_main]

zkm_runtime::entrypoint!(main);

pub fn main() {
    let n = zkm_runtime::io::read::<usize>();
    let b: bool = revm_utils::transfer_eth_n_times(n);
    zkm_runtime::io::commit(&b);
}
