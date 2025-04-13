use revm::{
    context::TxEnv,
    primitives::{address, alloy_primitives::U160, Address, TxKind, U256},
    state::AccountInfo,
    Context, ExecuteCommitEvm, MainBuilder, MainContext,
};
use revm_database::{CacheDB, EmptyDB};

pub fn transfer_eth_n_times(n: usize) -> bool {
    /* Initialize REVM */
    let transfer_fee = U256::from(100_000_000_000_000_000u128);
    let transfer_amount = transfer_fee * U256::from(100_000_000);
    let initial_tokens = U256::from(n) * (transfer_amount + transfer_fee);

    let base_account = address!("0x0000000000000000000000000000000000000000");
    let mut base_account_info = AccountInfo::default();
    base_account_info.balance = initial_tokens;

    let mut cache_db = CacheDB::new(EmptyDB::default());
    cache_db.insert_account_info(base_account, base_account_info);

    let mut evm = Context::mainnet().with_db(cache_db).build_mainnet();

    /* Transfer */
    let mut is_all_success = true;
    for i in 0..n {
        let to = Address::from(U160::from(100_000_000_000_000_000u64 + i as u64));

        let mut tx = TxEnv::default();
        tx.caller = base_account;
        tx.kind = TxKind::Call(to);
        tx.value = transfer_amount;
        tx.nonce = i as u64;
        let res = evm.transact_commit(tx);
        is_all_success &= res.is_ok();
    }

    is_all_success
}

// https://medium.com/@solidquant/revm-is-all-you-need-e01b5b0421e4
// https://medium.com/@solidquant/why-i-think-everyone-should-use-revm-rust-evm-art-of-mempool-watching-part-2-c67308d5cf03
