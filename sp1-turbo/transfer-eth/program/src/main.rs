#![no_main]
sp1_zkvm::entrypoint!(main);


// use ethblock_lib::{trace_block, PublicValuesStruct};

use alloy_sol_types::sol;
use alloy_sol_types::SolType;
use revm_utils::transfer_eth_n_times;

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        bool result;
    }
}

fn main() {

    let num = sp1_zkvm::io::read::<usize>();

    let b = transfer_eth_n_times(num);
    assert!(b);

    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct { result: b });

    sp1_zkvm::io::commit_slice(&bytes);
}