use std::convert::Infallible;

use revm::{
    primitives::{Address, B256, U256},
    state::{AccountInfo, Bytecode},
    Database, DatabaseRef,
};

pub struct WrapperDB {}

impl WrapperDB {
    pub fn new() -> Self {
        Self {}
    }
}

impl Database for WrapperDB {
    #[doc = " The database error type."]
    type Error = Infallible;

    #[doc = " Gets basic account information."]
    fn basic(&mut self, _address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        todo!()
    }

    #[doc = " Gets account code by its hash."]
    fn code_by_hash(&mut self, _code_hash: B256) -> Result<Bytecode, Self::Error> {
        todo!()
    }

    #[doc = " Gets storage value of address at index."]
    fn storage(&mut self, _address: Address, _index: U256) -> Result<U256, Self::Error> {
        todo!()
    }

    #[doc = " Gets block hash by block number."]
    fn block_hash(&mut self, _number: u64) -> Result<B256, Self::Error> {
        todo!()
    }
}

impl DatabaseRef for WrapperDB {
    #[doc = " The database error type."]
    type Error = Infallible;

    #[doc = " Gets basic account information."]
    fn basic_ref(&self, _address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        todo!()
    }

    #[doc = " Gets account code by its hash."]
    fn code_by_hash_ref(&self, _code_hash: B256) -> Result<Bytecode, Self::Error> {
        todo!()
    }

    #[doc = " Gets storage value of address at index."]
    fn storage_ref(&self, _address: Address, _index: U256) -> Result<U256, Self::Error> {
        todo!()
    }

    #[doc = " Gets block hash by block number."]
    fn block_hash_ref(&self, _number: u64) -> Result<B256, Self::Error> {
        todo!()
    }
}
