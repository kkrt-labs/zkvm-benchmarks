use revm::{
    db::CacheState,
    interpreter::CreateScheme,
    primitives::{
        calc_excess_blob_gas, keccak256, BlockEnv, Bytecode, CfgEnv, Env, SpecId, TransactTo,
        TxEnv, B256, U256,
    },
    Evm,
    // context::BlockEnv, // note: context は　private なので、revm::primitives::BlockEnv を使うように変更する。
};
// extern crate libc;

mod models;
use models::*;

mod utils;

use utils::recover_address;

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

const INPUT: &[u8] = include_bytes!("");

// pub fn main() {
//     ethereum_test();
// }

fn ethereum_test() {
    let input: Vec<u8> = INPUT.to_vec();
    let suite = read_suite(&input);

    assert!(execute_test_suite(suite).is_ok());
}

fn read_suite(s: &Vec<u8>) -> TestSuite {
    let suite: TestUnit = serde_json::from_slice(s).map_err(|e| e).unwrap();
    let mut btm = BTreeMap::new();
    btm.insert("test".to_string(), suite);
    TestSuite(btm)
}

pub fn execute_test_suite(suite: TestSuite) -> Result<(), String> {
    // let s = std::fs::read_to_string(path).unwrap();
    // let path = path.to_string_lossy().into_owned();
    // let suite: TestSuite = serde_json::from_str(&s).map_err(|e| TestError {
    //     name: "Unknown".to_string(),
    //     path: path.clone(),
    //     kind: e.into(),
    // })?;

    for (name, unit) in suite.0 {
        // Create database and insert cache
        let mut cache_state = CacheState::new(false);
        for (address, info) in unit.pre {
            let code_hash = keccak256(&info.code);
            let bytecode = Bytecode::new_raw_checked(info.code.clone())
                .unwrap_or(Bytecode::new_legacy(info.code));
            let acc_info = revm::primitives::AccountInfo {
                balance: info.balance,
                code_hash,
                code: Some(bytecode),
                nonce: info.nonce,
            };
            cache_state.insert_account_with_storage(address, acc_info, info.storage);
        }

        let mut cfg = CfgEnv::default();
        let mut block = BlockEnv::default();
        let mut tx = TxEnv::default();
        // For mainnet
        cfg.chain_id = 1;

        // Block env
        block.number = unit
            .env
            .current_number
            .try_into()
            .unwrap_or(U256::from(u64::MAX));
        block.coinbase = unit.env.current_coinbase;
        block.timestamp = unit
            .env
            .current_timestamp
            .try_into()
            .unwrap_or(U256::from(u64::MAX));
        block.gas_limit = unit
            .env
            .current_gas_limit
            .try_into()
            .unwrap_or(U256::from(u64::MAX));
        block.basefee = unit
            .env
            .current_base_fee
            .unwrap_or_default()
            .try_into()
            .unwrap_or(U256::from(u64::MAX));
        block.difficulty = unit.env.current_difficulty;
        // After the Merge prevrandao replaces mix_hash field in block and replaced difficulty opcode in EVM.
        block.prevrandao = unit.env.current_random;

        // Tx env
        tx.caller = if let Some(address) = unit.transaction.sender {
            address
        } else {
            recover_address(unit.transaction.secret_key.as_slice())
                .ok_or_else(|| String::from(format!("name: {} kind:{}", name.clone(), unit.transaction.secret_key)))?
        };
        tx.gas_price = unit
            .transaction
            .gas_price
            .or(unit.transaction.max_fee_per_gas)
            .unwrap_or_default()
            .try_into()
            .unwrap_or(U256::from(u128::MAX));
        tx.gas_priority_fee = unit
            .transaction
            .max_priority_fee_per_gas
            .map(|b| b);
        // EIP-4844
        tx.blob_hashes = unit.transaction.blob_versioned_hashes.clone();
        tx.max_fee_per_blob_gas = Some(unit
            .transaction
            .max_fee_per_blob_gas
            .map(|b| b)
            .unwrap_or(U256::from(u128::MAX)));

        // Post and execution
        for (spec_name, tests) in unit.post {
            // Constantinople was immediately extended by Petersburg.
            // There isn't any production Constantinople transaction
            // so we don't support it and skip right to Petersburg.
            if spec_name == SpecName::Constantinople {
                continue;
            }

            // cfg.spec = spec_name.to_spec_id(); // Note: spec filedがなくなって直接実行時に指定するようになったっぽい。

            // EIP-4844
            if let Some(current_excess_blob_gas) = unit.env.current_excess_blob_gas {
                block.set_blob_excess_gas_and_price(
                    current_excess_blob_gas.to(),
                    cfg.spec.is_enabled_in(SpecId::PRAGUE),
                );
            } else if let (Some(parent_blob_gas_used), Some(parent_excess_blob_gas)) = (
                unit.env.parent_blob_gas_used,
                unit.env.parent_excess_blob_gas,
            ) {
                block.set_blob_excess_gas_and_price(
                    calc_excess_blob_gas(
                        parent_blob_gas_used.to(),
                        parent_excess_blob_gas.to(),
                        unit.env
                            .parent_target_blobs_per_block
                            .map(|i| i.to())
                            .unwrap_or(TARGET_BLOB_GAS_PER_BLOCK_CANCUN),
                    ),
                    cfg.spec.is_enabled_in(SpecId::PRAGUE),
                );
            }

            if cfg.spec.is_enabled_in(SpecId::MERGE) && block.prevrandao.is_none() {
                // If spec is merge and prevrandao is not set, set it to default
                block.prevrandao = Some(B256::default());
            }

            for (index, test) in tests.into_iter().enumerate() {
                let Some(tx_type) = unit.transaction.tx_type(test.indexes.data) else {
                    if test.expect_exception.is_some() {
                        continue;
                    } else {
                        panic!("Invalid transaction type without expected exception");
                    }
                };

                tx.tx_type = tx_type as u8;

                tx.gas_limit = unit.transaction.gas_limit[test.indexes.gas].saturating_to();

                tx.data = unit
                    .transaction
                    .data
                    .get(test.indexes.data)
                    .unwrap()
                    .clone();

                tx.nonce = u64::try_from(unit.transaction.nonce).unwrap();
                tx.value = unit.transaction.value[test.indexes.value];

                tx.access_list = unit
                    .transaction
                    .access_lists
                    .get(test.indexes.data)
                    .cloned()
                    .flatten()
                    .unwrap_or_default();

                tx.authorization_list = unit
                    .transaction
                    .authorization_list
                    .clone()
                    .map(|auth_list| auth_list.into_iter().map(Into::into).collect::<Vec<_>>())
                    .unwrap_or_default();

                let to = match unit.transaction.to {
                    Some(add) => TxKind::Call(add),
                    None => TxKind::Create,
                };
                tx.kind = to;

                let mut cache = cache_state.clone();
                cache.set_state_clear_flag(cfg.spec.is_enabled_in(SpecId::SPURIOUS_DRAGON));
                let mut state = database::State::builder()
                    .with_cached_prestate(cache)
                    .with_bundle_update()
                    .build();
                let mut evm = Context::mainnet()
                    .with_block(&block)
                    .with_tx(&tx)
                    .with_cfg(&cfg)
                    .with_db(&mut state)
                    .build_mainnet();

                // Do the deed
                let (e, exec_result) = if trace {
                    let mut evm = Context::mainnet()
                        .with_block(&block)
                        .with_tx(&tx)
                        .with_cfg(&cfg)
                        .with_db(&mut state)
                        .build_mainnet_with_inspector(
                            TracerEip3155::buffered(stderr()).without_summary(),
                        );

                    let timer = Instant::now();
                    let res = evm.inspect_commit_previous();
                    *elapsed.lock().unwrap() += timer.elapsed();

                    let spec = cfg.spec();
                    let db = &mut evm.data.ctx.journaled_state.database;
                    // Dump state and traces if test failed
                    let output = check_evm_execution(
                        &test,
                        unit.out.as_ref(),
                        &name,
                        &res,
                        db,
                        spec,
                        print_json_outcome,
                    );
                    let Err(e) = output else {
                        continue;
                    };
                    (e, res)
                } else {
                    let timer = Instant::now();
                    let res = evm.transact_commit_previous();
                    *elapsed.lock().unwrap() += timer.elapsed();

                    let spec = cfg.spec();
                    let db = evm.data.ctx.journaled_state.database;
                    // Dump state and traces if test failed
                    let output = check_evm_execution(
                        &test,
                        unit.out.as_ref(),
                        &name,
                        &res,
                        db,
                        spec,
                        print_json_outcome,
                    );
                    let Err(e) = output else {
                        continue;
                    };
                    (e, res)
                };

                // Print only once or if we are already in trace mode, just return error
                // If trace is true that print_json_outcome will be also true.
                static FAILED: AtomicBool = AtomicBool::new(false);
                if print_json_outcome || FAILED.swap(true, Ordering::SeqCst) {
                    return Err(TestError {
                        name: name.clone(),
                        path: path.clone(),
                        kind: e,
                    });
                }

                // Re-build to run with tracing
                let mut cache = cache_state.clone();
                cache.set_state_clear_flag(cfg.spec.is_enabled_in(SpecId::SPURIOUS_DRAGON));
                let mut state = database::State::builder()
                    .with_cached_prestate(cache)
                    .with_bundle_update()
                    .build();

                println!("\nTraces:");

                let mut evm = Context::mainnet()
                    .with_db(&mut state)
                    .with_block(&block)
                    .with_tx(&tx)
                    .with_cfg(&cfg)
                    .build_mainnet_with_inspector(
                        TracerEip3155::buffered(stderr()).without_summary(),
                    );

                let _ = evm.inspect_commit_previous();

                println!("\nExecution result: {exec_result:#?}");
                println!("\nExpected exception: {:?}", test.expect_exception);
                println!("\nState before: {cache_state:#?}");
                println!(
                    "\nState after: {:#?}",
                    evm.data.ctx.journaled_state.database.cache
                );
                println!("\nSpecification: {:?}", cfg.spec);
                println!("\nTx: {tx:#?}");
                println!("Block: {block:#?}");
                println!("Cfg: {cfg:#?}");
                println!("\nTest name: {name:?} (index: {index}, path: {path:?}) failed:\n{e}");

                return Err(TestError {
                    path: path.clone(),
                    name: name.clone(),
                    kind: e,
                });
            }
        }
    }
    Ok(())
}

// fn execute_test_suite(suite: TestSuite) -> Result<(), String> {
//     for (_name, unit) in suite.0 {
//         // Create database and insert cache
//         let mut cache_state = CacheState::new(false);
//         for (address, info) in unit.pre {
//             let acc_info = revm::primitives::AccountInfo {
//                 balance: info.balance,
//                 // code_hash: B256::from(zkm_runtime::io::keccak(&info.code)),
//                 code_hash: keccak256(&info.code),
//                 code: Some(Bytecode::new_raw(info.code)),
//                 nonce: info.nonce,
//             };
//             cache_state.insert_account_with_storage(address, acc_info, info.storage);
//         }

//         let mut env = Env::default();
//         // for mainnet
//         env.cfg.chain_id = 1;
//         // env.cfg.spec_id is set down the road

//         // block env
//         env.block.number = unit.env.current_number;
//         env.block.coinbase = unit.env.current_coinbase;
//         env.block.timestamp = unit.env.current_timestamp;
//         env.block.gas_limit = unit.env.current_gas_limit;
//         env.block.basefee = unit.env.current_base_fee.unwrap_or_default();
//         env.block.difficulty = unit.env.current_difficulty;
//         // after the Merge prevrandao replaces mix_hash field in block and replaced difficulty opcode in EVM.
//         env.block.prevrandao = unit.env.current_random;
//         // EIP-4844
//         if let (Some(parent_blob_gas_used), Some(parent_excess_blob_gas)), Some(parent_target_blob_gas_per_block) = (
//             unit.env.parent_blob_gas_used,
//             unit.env.parent_excess_blob_gas,
//             unit.env.parent_target_blob_gas_per_block,
//         ) {
//             env.block
//                 .set_blob_excess_gas_and_price(calc_excess_blob_gas(
//                     parent_blob_gas_used.to(),
//                     parent_excess_blob_gas.to(),
//                 ));
//         }

//         // tx env
//         env.tx.caller = match unit.transaction.sender {
//             Some(address) => address,
//             _ => recover_address(unit.transaction.secret_key.as_slice())
//                 .ok_or_else(|| String::new())?,
//         };
//         env.tx.gas_price = unit
//             .transaction
//             .gas_price
//             .or(unit.transaction.max_fee_per_gas)
//             .unwrap_or_default();
//         env.tx.gas_priority_fee = unit.transaction.max_priority_fee_per_gas;
//         // EIP-4844
//         env.tx.blob_hashes = unit.transaction.blob_versioned_hashes;
//         env.tx.max_fee_per_blob_gas = unit.transaction.max_fee_per_blob_gas;

//         // post and execution
//         for (spec_name, tests) in unit.post {
//             if matches!(
//                 spec_name,
//                 SpecName::ByzantiumToConstantinopleAt5
//                     | SpecName::Constantinople
//                     | SpecName::Unknown
//             ) {
//                 continue;
//             }

//             let spec_id = spec_name.to_spec_id();

//             for (_index, test) in tests.into_iter().enumerate() {
//                 env.tx.gas_limit = unit.transaction.gas_limit[test.indexes.gas].saturating_to();

//                 env.tx.data = unit
//                     .transaction
//                     .data
//                     .get(test.indexes.data)
//                     .unwrap()
//                     .clone();
//                 env.tx.value = unit.transaction.value[test.indexes.value];

//                 env.tx.access_list = unit
//                     .transaction
//                     .access_lists
//                     .get(test.indexes.data)
//                     .and_then(Option::as_deref)
//                     .unwrap_or_default()
//                     .iter()
//                     .map(|item| {
//                         (
//                             item.address,
//                             item.storage_keys
//                                 .iter()
//                                 .map(|key| U256::from_be_bytes(key.0))
//                                 .collect::<Vec<_>>(),
//                         )
//                     })
//                     .collect();

//                 let to = match unit.transaction.to {
//                     Some(add) => TransactTo::Call(add),
//                     None => TransactTo::Create(CreateScheme::Create),
//                 };
//                 env.tx.transact_to = to;

//                 let mut cache = cache_state.clone();
//                 cache.set_state_clear_flag(SpecId::enabled(
//                     spec_id,
//                     revm::primitives::SpecId::SPURIOUS_DRAGON,
//                 ));
//                 let mut state = revm::db::State::builder()
//                     .with_cached_prestate(cache)
//                     .with_bundle_update()
//                     .build();
//                 let mut evm = Evm::builder()
//                     .with_db(&mut state)
//                     .modify_env(|e| *e = env.clone())
//                     .spec_id(spec_id)
//                     .build();

//                 // do the deed
//                 //let timer = Instant::now();
//                 let mut check = || {
//                     let exec_result = evm.transact_commit();

//                     match (&test.expect_exception, &exec_result) {
//                         // do nothing
//                         (None, Ok(_)) => (),
//                         // return okay, exception is expected.
//                         (Some(_), Err(_e)) => {
//                             return Ok(());
//                         }
//                         _ => {
//                             let s = exec_result.clone().err().map(|e| e.to_string()).unwrap();
//                             return Err(s);
//                         }
//                     }
//                     Ok(())
//                 };

//                 let Err(e) = check() else { continue };

//                 return Err(e);
//             }
//         }
//     }
//     Ok(())
// }
