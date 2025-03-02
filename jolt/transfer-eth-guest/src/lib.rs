#![cfg_attr(feature = "guest", no_std)]
#![no_main]

#[jolt::provable]
fn transfer_eth_n_times(n: usize) -> bool {
    revm_utils::transfer_eth_n_times(n)
}
