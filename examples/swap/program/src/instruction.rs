use borsh::{BorshDeserialize, BorshSerialize};
use arch_program::pubkey::Pubkey;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub enum LiquidityInstruction {
    AddLiquidity {
        token_address: Pubkey,
        amount: u64,
    },
    RemoveLiquidity {
        token_address: Pubkey,
        amount: u64,
    },
    SwapTokens {
        token_in_address: Pubkey,
        token_out_address: Pubkey,
        amount: u64,
    },
    StakeTokens {
        stake_amount: u64,
    },
    UnstakeTokens {
        unstake_amount: u64,
    },
    ClaimRewards,
}
