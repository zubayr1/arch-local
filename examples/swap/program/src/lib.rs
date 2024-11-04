pub mod curve;
pub mod instruction;
pub mod operations;
pub mod reward;
pub mod state;
pub mod vault_operations;
pub mod trading_routes;
pub mod limit_order;
pub mod dca;
use trading_routes::TradingRoute;
use limit_order::LimitOrderBook;

pub use curve::*;
pub use instruction::LiquidityInstruction;
pub use operations::*;
pub use reward::RewardParams;
pub use state::LiquidityParams;

use arch_program::{
    account::AccountInfo,
    entrypoint,
    program::next_account_info,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

mod curve;
mod operations;
mod reward;
mod state;

use crate::curve::calculate_swap_amount;
use crate::operations::{add_liquidity, remove_liquidity, swap_tokens};
pub use crate::reward::RewardParams;
pub use crate::state::LiquidityParams;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    if accounts.len() != 1 {
        return Err(ProgramError::Custom(501));
    }

    let account_iter = &mut accounts.iter();
    let liquidity_account = next_account_info(account_iter)?;

    let instruction = LiquidityInstruction::deserialize(&mut &instruction_data[..])
        .map_err(|_| ProgramError::Custom(502))?;

    match instruction {

        LiquidityInstruction::AddLiquidity {
            token_address,
            amount,
        } => {
            add_liquidity(
                &liquidity_account,
                &mut liquidity_params,
                token_address,
                amount,
            )?;
        }
        LiquidityInstruction::RemoveLiquidity {
            token_address,
            amount,
        } => {
            remove_liquidity(
                &liquidity_account,
                &mut liquidity_params,
                token_address,
                amount,
            )?;
        }
        LiquidityInstruction::SwapTokens {
            token_in_address,
            token_out_address,
            amount,
        } => {
            swap_tokens(
                &liquidity_account,
                &mut liquidity_params,
                token_in_address,
                token_out_address,
                amount,
            )?;
        }
        LiquidityInstruction::StakeTokens { stake_amount } => {
            let accounts_iter = &mut accounts.iter();
            let staking_account = next_account_info(accounts_iter)?;

            let mut reward_params = RewardParams::try_from_slice(&staking_account.data.borrow())
                .map_err(|_| ProgramError::InvalidAccountData)?;

            stake_tokens(staking_account, &mut reward_params, stake_amount)?;

            reward_params
                .serialize(&mut &mut staking_account.data.borrow_mut()[..])
                .map_err(|_| ProgramError::InvalidAccountData)?;
        }
        LiquidityInstruction::UnstakeTokens { unstake_amount } => {
            let accounts_iter = &mut accounts.iter();
            let staking_account = next_account_info(accounts_iter)?;

            let mut reward_params = RewardParams::try_from_slice(&staking_account.data.borrow())
                .map_err(|_| ProgramError::InvalidAccountData)?;

            unstake_tokens(staking_account, &mut reward_params, unstake_amount)?;

            reward_params
                .serialize(&mut &mut staking_account.data.borrow_mut()[..])
                .map_err(|_| ProgramError::InvalidAccountData)?;
        }
        LiquidityInstruction::ClaimRewards => {
            let accounts_iter = &mut accounts.iter();
            let staking_account = next_account_info(accounts_iter)?;

            let mut reward_params = RewardParams::try_from_slice(&staking_account.data.borrow())
                .map_err(|_| ProgramError::InvalidAccountData)?;

            let reward_amount = claim_rewards(staking_account, &mut reward_params)?;

            reward_params
                .serialize(&mut &mut staking_account.data.borrow_mut()[..])
                .map_err(|_| ProgramError::InvalidAccountData)?;
        }
        LiquidityInstruction::OptimizedTradingRoute => {
            let accounts_iter = &mut accounts.iter();
            let staking_account = next_account_info(accounts_iter)?;

            let mut reward_params = RewardParams::try_from_slice(&staking_account.data.borrow())
                .map_err(|_| ProgramError::InvalidAccountData)?;
        LiquidityInstruction::AddLiquidity { token_a_amount, token_b_amount } => {
            let mut liquidity_params = LiquidityParams::deserialize(&mut &liquidity_account.data.borrow()[..])
                .map_err(|_| ProgramError::Custom(503))?;
            add_liquidity(liquidity_account, &mut liquidity_params, token_a_amount, token_b_amount)
        },
        LiquidityInstruction::RemoveLiquidity { token_a_amount, token_b_amount, current_time } => {
            let mut liquidity_params = LiquidityParams::deserialize(&mut &liquidity_account.data.borrow()[..])
                .map_err(|_| ProgramError::Custom(503))?;
            remove_liquidity(liquidity_account, &mut liquidity_params, token_a_amount, token_b_amount, current_time)
        },
        LiquidityInstruction::SwapTokens { token_a_amount, min_token_b_amount } => {
            let mut liquidity_params = LiquidityParams::deserialize(&mut &liquidity_account.data.borrow()[..])
                .map_err(|_| ProgramError::Custom(503))?;
            swap_tokens(liquidity_account, &mut liquidity_params, token_a_amount, min_token_b_amount)
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum LiquidityInstruction {
    AddLiquidity {
        token_a_amount: u64,
        token_b_amount: u64,
    },
    RemoveLiquidity {
        token_a_amount: u64,
        token_b_amount: u64,
        current_time: u64,
    },
    SwapTokens {
        token_a_amount: u64,
        min_token_b_amount: u64,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_account() -> (Pubkey, AccountInfo) {
        let pubkey = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let mut data = vec![0u8; 1000];
        let account = AccountInfo::new(
            &pubkey,
            &mut data,
            &owner,
            true,
            true,
            false,
        );
        (pubkey, account)
    }

    #[test]
    fn test_process_instruction() {
        let (_, account) = create_test_account();
        let program_id = Pubkey::new_unique();
        
        let instruction = LiquidityInstruction::AddLiquidity {
            token_a_amount: 100,
            token_b_amount: 100,
        };
        
        let instruction_data = borsh::to_vec(&instruction).unwrap();
        
        let result = process_instruction(
            &program_id,
            &[account],
            &instruction_data,
        );
        
        assert!(result.is_ok());
    }
}