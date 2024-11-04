use crate::vault::Vault;
use crate::operations::{calculate_swap_amount};
use crate::errors::SwapError;
use arch_program::{account::AccountInfo, program_error::ProgramError};
use arch_program::pubkey::Pubkey;
use log::{info, error}; // Use the log crate for structured logging

pub fn add_liquidity_to_vault(
    vault: &mut Vault,
    token_address: Pubkey,
    amount: u64,
) -> Result<(), ProgramError> {
    *vault.token_amounts.entry(token_address).or_insert(0) += amount;
    Ok(())
}

pub fn remove_liquidity_from_vault(
    vault: &mut Vault,
    token_address: Pubkey,
    amount: u64,
) -> Result<(), ProgramError> {
    let token_amount = vault.token_amounts.entry(token_address).or_insert(0);
    if *token_amount < amount {
        return Err(ProgramError::Custom(507)); // Insufficient liquidity
    }
    *token_amount -= amount;
    Ok(())
}

pub fn swap_tokens_in_vault(
    vault: &mut Vault,
    token_in_address: Pubkey,
    token_out_address: Pubkey,
    amount: u64,
) -> Result<u64, ProgramError> {
    let token_in_amount = vault.token_amounts.entry(token_in_address).or_insert(0);
    let token_out_amount = vault.token_amounts.entry(token_out_address).or_insert(0);

    let swap_amount = calculate_swap_amount(*token_in_amount, *token_out_amount, amount);
    if swap_amount == 0 {
        return Err(SwapError::SlippageError.into());
    }

    *token_in_amount += amount;
    *token_out_amount -= swap_amount;

    Ok(swap_amount)
}

pub fn log_swap_status(vault: &Vault) {
    info!("Current Vault Status: Token A: {}, Token B: {}", vault.token_a_amount, vault.token_b_amount);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::Vault;

    #[test]
    fn test_add_liquidity() {
        let mut vault = Vault::new(Pubkey::new_unique(), Pubkey::new_unique(), Pubkey::new_unique());
        assert_eq!(vault.token_a_amount, 0);
        assert_eq!(vault.token_b_amount, 0);

        add_liquidity_to_vault(&mut vault, 100, 200).unwrap();
        assert_eq!(vault.token_a_amount, 100);
        assert_eq!(vault.token_b_amount, 200);
    }

    #[test]
    fn test_remove_liquidity() {
        let mut vault = Vault::new(Pubkey::new_unique(), Pubkey::new_unique(), Pubkey::new_unique());
        add_liquidity_to_vault(&mut vault, 300, 300).unwrap();
        remove_liquidity_from_vault(&mut vault, 100, 100).unwrap();
        assert_eq!(vault.token_a_amount, 200);
        assert_eq!(vault.token_b_amount, 200);
    }

    #[test]
    fn test_swap_tokens() {
        let mut vault = Vault::new(Pubkey::new_unique(), Pubkey::new_unique(), Pubkey::new_unique());
        add_liquidity_to_vault(&mut vault, 500, 500).unwrap();
        let output = swap_tokens_in_vault(&mut vault, 100, 90, true).unwrap();
        assert!(output >= 90); // Check if the output meets the minimum output amount
        assert_eq!(vault.token_a_amount, 400); // 500 - 100
        assert_eq!(vault.token_b_amount, 500 + output); // 500 + output from the swap
    }

    #[test]
    fn test_insufficient_liquidity_removal() {
        let mut vault = Vault::new(Pubkey::new_unique(), Pubkey::new_unique(), Pubkey::new_unique());
        add_liquidity_to_vault(&mut vault, 100, 100).unwrap();
        let result = remove_liquidity_from_vault(&mut vault, 200, 200);
        assert!(result.is_err()); // Expect an error due to insufficient liquidity
    }

    #[test]
    fn test_invalid_swap_amount() {
        let mut vault = Vault::new(Pubkey::new_unique(), Pubkey::new_unique(), Pubkey::new_unique());
        add_liquidity_to_vault(&mut vault, 500, 500).unwrap();
        let result = swap_tokens_in_vault(&mut vault, 100, 600, true); // Expecting more than available
        assert!(result.is_err()); // Expect an error due to slippage or invalid output
    }
}
