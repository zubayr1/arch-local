use arch_program::{
    account::AccountInfo, account::AccountMeta, instruction::Instruction, pubkey::Pubkey,
    system_instruction::SystemInstruction, utxo::UtxoMeta,
};
use bitcoincore_rpc::{Auth, Client};
use borsh::{BorshDeserialize, BorshSerialize};
use common::constants::*;
use common::helper::*;
use common::models::*;
use std::fs;
use std::str::FromStr;
use std::thread;

use env_logger;
use log::{debug, error, info, warn};

#[path = "../program/src/curve.rs"]
pub mod curve;
#[path = "../program/src/operations.rs"]
pub mod operations;
#[path = "../program/src/reward.rs"]
pub mod reward;
#[path = "../program/src/state.rs"]
pub mod state;

use crate::curve::calculate_swap_amount;
use crate::operations::add_liquidity;
use crate::operations::claim_rewards;
use crate::operations::remove_liquidity;
use crate::operations::unstake_tokens;
pub use crate::reward::RewardParams;
pub use crate::state::LiquidityParams;

// Define the `create_mock_account` function
pub fn create_mock_account<'a>(
    key: &'a Pubkey,
    data: &'a mut [u8],
    owner: &'a Pubkey,
    utxo: &'a UtxoMeta,
) -> AccountInfo<'a> {
    AccountInfo::new(
        key, data, owner, utxo, true,  // is_signer
        true,  // is_writable
        false, // is_executable
    )
}

fn setup() {
    env_logger::init();
}



#[cfg(test)]
mod tests;
