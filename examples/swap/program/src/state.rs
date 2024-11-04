use borsh::{BorshDeserialize, BorshSerialize};
use arch_program::pubkey::Pubkey;
use std::collections::HashMap;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct LiquidityParams {
    pub token_amounts: HashMap<Pubkey, u64>,
    pub yield_accumulated: u64,
    pub last_yield_update_time: u64,
}

impl LiquidityParams {
    pub fn new() -> Self {
        LiquidityParams {
            token_amounts: HashMap::new(),
            yield_accumulated: 0,
            last_yield_update_time: 0,
        }
    }

    // Method to get the current liquidity amount
    pub fn get_liquidity_amount(&self) -> u64 {
        self.token_amounts.values().sum()
    }

    pub fn get_token_details(&self) -> (u64, u64, u64) {
        (self.token_amounts.values().sum(), self.token_amounts.values().sum(), self.token_amounts.values().sum())
    }
}

pub struct LimitOrder {
    pub owner: Pubkey,
    pub token_pair: (Pubkey, Pubkey),
    pub amount: u64,
    pub price: u64,
    pub order_type: OrderType,
    pub status: OrderStatus,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub enum OrderStatus {
    Open,
    Executed,
    Cancelled,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Vault {
    pub owner: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}
