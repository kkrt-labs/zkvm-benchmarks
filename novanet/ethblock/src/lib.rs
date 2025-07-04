use std::cell::RefCell;

// use ethblock_utils::trace_ethblock;
use getrandom::register_custom_getrandom;
use rand::{rngs::SmallRng, RngCore, SeedableRng};

/* wasm32-unknown-unknown でrandを使用する */
thread_local! {
    pub static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::seed_from_u64(123456789));
}
fn custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    RNG.with(|rng| rng.borrow_mut().fill_bytes(buf));
    Ok(())
}
register_custom_getrandom!(custom_getrandom);

#[no_mangle]
pub fn ethblock(_num_txs: usize) -> bool {
    // trace_ethblock(num_txs)
    true
}
