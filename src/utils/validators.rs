// Input validation utilities
// Only keeping functions that are actually used to eliminate warnings

/// Validate Bitcoin amount for calculations
pub fn validate_bitcoin_amount(amount: f64) -> Result<(), String> {
    if amount < 0.0 {
        return Err("Bitcoin amount cannot be negative".to_string());
    }
    
    if amount > 21_000_000.0 {
        return Err("Bitcoin amount cannot exceed total supply".to_string());
    }
    
    if amount.is_nan() || amount.is_infinite() {
        return Err("Invalid Bitcoin amount".to_string());
    }
    
    Ok(())
}

// All other unused validator functions have been removed to eliminate warnings
