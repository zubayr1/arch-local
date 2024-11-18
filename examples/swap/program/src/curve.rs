use std::num::Wrapping;

pub fn calculate_swap_amount(token_a_amount: u64, token_b_amount: u64, input_amount: u64) -> u64 {
    let rate_adjustment = calculate_rate_adjustment(token_a_amount, token_b_amount);
    let k = Wrapping(token_a_amount) * Wrapping(token_b_amount);
    let input_amount_wrapping = Wrapping(input_amount) * Wrapping(rate_adjustment);

    let output_amount = 
        Wrapping(token_b_amount) - (k / (Wrapping(token_a_amount) + input_amount_wrapping));
    
    output_amount.0
}

fn calculate_rate_adjustment(token_a_amount: u64, token_b_amount: u64) -> u64 {
    if token_a_amount > token_b_amount {
        let excess = token_a_amount.saturating_sub(token_b_amount);
        1 + (excess / token_b_amount).min(10)  // Limit rate adjustment to avoid excessive values
    } else if token_a_amount < token_b_amount {
        let shortage = token_b_amount.saturating_sub(token_a_amount);
        1 + (shortage / token_a_amount).min(10) // Limit rate adjustment similarly
    } else {
        1
    }
}

