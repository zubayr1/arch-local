use std::num::Wrapping;

pub fn calculate_swap_amount(token_a_amount: u64, token_b_amount: u64, input_amount: u64) -> u64 {
    let rate_adjustment = calculate_rate_adjustment(token_a_amount, token_b_amount);
    let k = Wrapping(token_a_amount) * Wrapping(token_b_amount);
    let input_amount_wrapping = Wrapping(input_amount) * Wrapping(rate_adjustment);

    let output_amount = 
        Wrapping(token_b_amount) - (k / (Wrapping(token_a_amount) + input_amount_wrapping));
    
    // Debug statements for tracking values
    println!("Token A Reserve: {}, Token B Reserve: {}", token_a_amount, token_b_amount);
    println!("Input Amount: {}, Rate Adjustment: {}", input_amount, rate_adjustment);
    println!("Constant Product (k): {}", k.0);
    println!("Output Amount: {}", output_amount.0);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_swap_amount() {
        // Test case 1: Balanced pool with small swap impact
        let token_a_reserve = 2000;
        let token_b_reserve = 2000;
        let input_amount = 100;

        let output = calculate_swap_amount(token_a_reserve, token_b_reserve, input_amount);
        assert_eq!(output, 96);

        // Test case 2: Unbalanced pool favoring token_a
        let token_a_reserve = 3000;
        let token_b_reserve = 1000;
        let input_amount = 100;

        let output = calculate_swap_amount(token_a_reserve, token_b_reserve, input_amount);
        assert_eq!(output, 91);

        // Test case 3: Large swap impact on a balanced pool
        let token_a_reserve = 1000;
        let token_b_reserve = 1000;
        let input_amount = 500;

        let output = calculate_swap_amount(token_a_reserve, token_b_reserve, input_amount);
        assert_eq!(output, 334);

        // Test case 4: Minimal swap impact on a large pool
        let token_a_reserve = 10000;
        let token_b_reserve = 10000;
        let input_amount = 10;

        let output = calculate_swap_amount(token_a_reserve, token_b_reserve, input_amount);
        assert_eq!(output, 10);

        // Test case 5: Unbalanced pool favoring token_b
        let token_a_reserve = 1000;
        let token_b_reserve = 3000;
        let input_amount = 100;

        let output = calculate_swap_amount(token_a_reserve, token_b_reserve, input_amount);
        assert_eq!(output, 114);
    }
}
