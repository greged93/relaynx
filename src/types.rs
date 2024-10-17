use serde::Serialize;
use starknet::core::types::Felt;

#[derive(Debug, Serialize)]
pub struct Account {
    pub(crate) address: Felt,
    pub(crate) balance: Felt,
}

impl Account {
    pub fn new(address: Felt, balance: Felt) -> Self {
        Self { address, balance }
    }
}
