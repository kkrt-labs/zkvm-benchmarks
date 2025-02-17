use alloy_sol_types::sol;
use anyhow::{Result, anyhow};
extern crate alloc;


sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        bool result;
    }
}

// mod sp1_reth_primitives;
mod copy;

use revm::{Context, InMemoryDB};
use revm::primitives::{Address, ExecutionResult, Output, TxKind, U256};
// use revm::{context_interface::result::{ExecutionResult, Output}, primitives::ruint::aliases::U256, Context, ExecuteCommitEvm, MainBuilder, MainContext};
use alloy_sol_types::SolCall;
use alloy_sol_types::SolValue;
// use sp1_reth_primitives::SP1RethInput;

const INPUT: &[u8] = include_bytes!("");

/*
sp1-rethにあるimpl InMemoryDBHelper for InMemoryDBと同じものを実装する。
 */
pub fn execuse_transfer_uniswap_v2(
    from: Address,
    to: Address,
    amount: U256,
    token: Address,
    // cache_db: &mut InMemoryDB,
) -> Result<()> {
    sol! {
        function transfer(address to, uint amount) external returns (bool);
    }

    let input: SP1RethInput = bincode::deserialize(INPUT).expect("unable to deserialize input");
    let cache_db = InMemoryDB::initalize(input);

    let encoded = transferCall { to, amount }.abi_encode();

    let mut evm = Context::mainnet()
        .with_db(cache_db)
        .modify_tx_chained(|tx| {
            tx.caller = from;
            tx.kind = TxKind::Call(token);
            tx.data = encoded.into();
            tx.value = U256::from(0);
        })
        .build_mainnet();

    let ref_tx = evm.transact_commit_previous().unwrap();
    let success: bool = match ref_tx {
        ExecutionResult::Success {
            output: Output::Call(value),
            ..
        } => <bool>::abi_decode(&value, false)?,
        result => return Err(anyhow!("'transfer' execution failed: {result:?}")),
    };

    if !success {
        return Err(anyhow!("'transfer' failed"));
    }

    Ok(())
}
