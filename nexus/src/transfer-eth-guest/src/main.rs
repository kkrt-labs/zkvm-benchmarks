#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::{read_private_input, write_output};

#[nexus_rt::main]
fn main() {
    let n = read_private_input::<usize>().unwrap();
    let z = revm_utils::transfer_eth_n_times(n);
    write_output::<bool>(&z)
}