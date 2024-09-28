use std::num::Wrapping;

pub fn calculate_swap_amount(token_a_amount: u64, token_b_amount: u64, input_amount: u64) -> u64 {
    let k = Wrapping(token_a_amount) * Wrapping(token_b_amount);
    let input_amount_wrapping = Wrapping(input_amount);

    // Simple x * y = k calculation
    let output_amount =
        Wrapping(token_b_amount) - (k / (Wrapping(token_a_amount) + input_amount_wrapping));
    output_amount.0
}