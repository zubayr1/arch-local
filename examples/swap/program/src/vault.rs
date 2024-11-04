use borsh::{BorshDeserialize, BorshSerialize};
use arch_program::pubkey::Pubkey;
use std::collections::HashMap;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Vault {
    pub owner: Pubkey,
    pub token_amounts: HashMap<Pubkey, u64>,
}

impl Vault {
    pub fn new(owner: Pubkey) -> Self {
        Vault {
            owner,
            token_amounts: HashMap::new(),
        }
    }
}





