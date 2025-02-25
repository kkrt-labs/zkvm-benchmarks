
use alloy_sol_types::sol;

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        bool result;
    }
}


pub fn trace_block(num_txs: usize) -> bool {
    ethblock_utils::trace_ethblock(num_txs)
}
