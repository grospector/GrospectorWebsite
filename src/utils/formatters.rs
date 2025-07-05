// Utility functions for formatting numbers and data
// Only keeping functions that are actually used to eliminate warnings

/// Format numbers with commas for better readability
pub fn format_number_with_commas(number: f64) -> String {
    let mut result = String::new();
    let number_str = format!("{:.0}", number);
    let chars: Vec<char> = number_str.chars().collect();
    
    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*ch);
    }
    
    result
}

/// Format large numbers with appropriate suffixes (K, M, B)
pub fn format_large_number(number: f64) -> String {
    if number.abs() >= 1_000_000_000.0 {
        format!("{:.1}B", number / 1_000_000_000.0)
    } else if number.abs() >= 1_000_000.0 {
        format!("{:.1}M", number / 1_000_000.0)
    } else if number.abs() >= 1_000.0 {
        format!("{:.1}K", number / 1_000.0)
    } else {
        format!("{:.0}", number)
    }
}

/// Format rank numbers with appropriate suffix (1st, 2nd, 3rd, etc.)
pub fn format_rank(rank: u64) -> String {
    let suffix = match rank % 10 {
        1 if rank % 100 != 11 => "st",
        2 if rank % 100 != 12 => "nd", 
        3 if rank % 100 != 13 => "rd",
        _ => "th",
    };
    
    format!("{}{}", format_number_with_commas(rank as f64), suffix)
}

// All other unused formatter functions have been removed to eliminate warnings
