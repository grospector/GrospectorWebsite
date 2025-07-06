use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Bitcoin wealth categories based on holdings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WealthCategory {
    Dust,      // < 0.001 BTC
    Shrimp,    // 0.001 - 0.01 BTC
    Crab,      // 0.01 - 0.1 BTC  
    Fish,      // 0.1 - 1 BTC
    Dolphin,   // 1 - 10 BTC
    Shark,     // 10 - 100 BTC
    Whale,     // 100 - 1000 BTC
    Humpback,  // 1000+ BTC
}

/// Represents a Bitcoin wealth distribution range
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WealthRange {
    pub min_btc: f64,
    pub max_btc: f64,
    pub address_count: u64,
    pub total_btc: f64,
    pub percentage_of_addresses: f64,
    pub percentage_of_supply: f64,
}

/// Complete Bitcoin distribution data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitcoinDistribution {
    pub ranges: Vec<WealthRange>,
    pub total_addresses: u64,
    pub total_supply: f64,
    pub timestamp: u64,
    pub data_source: String,
}

/// User's percentile calculation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PercentileResult {
    pub user_bitcoin_amount: f64,
    pub percentile: f64,
    pub rank: u64,
    pub addresses_below: u64,
    pub addresses_above: u64,
    pub wealth_category: WealthCategory,
    pub comparison_metrics: HashMap<String, f64>,
}

/// Comprehensive Bitcoin network statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitcoinStats {
    pub total_supply: f64,
    pub circulating_supply: f64,
    pub total_addresses: u64,
    pub active_addresses: u64,
    pub mean_amount: f64,
    pub median_amount: f64,
    pub gini_coefficient: f64,
    pub top_1_percent_wealth: f64,
    pub top_5_percent_wealth: f64,
    pub top_10_percent_wealth: f64,
    pub concentration_ratios: HashMap<String, f64>,
}

/// Wealth inequality metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WealthInequalityMetrics {
    pub gini_coefficient: f64,
    pub hhi_index: f64,
    pub concentration_ratios: HashMap<String, f64>,
    pub percentile_thresholds: Vec<(f64, f64)>, // (percentile, btc_amount)
}

/// Statistics derived from Bitcoin distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitcoinStatistics {
    pub median_balance: f64,
    pub mean_balance: f64,
    pub gini_coefficient: f64,
    pub top_1_percent_threshold: f64,
    pub top_5_percent_threshold: f64,
    pub top_10_percent_threshold: f64,
    pub concentration_ratio: f64,
}

impl BitcoinDistribution {
    /// Calculate statistics from the distribution data
    #[allow(dead_code)]
    pub fn calculate_statistics(&self) -> BitcoinStatistics {
        let mut cumulative_addresses = 0u64;
        let mut cumulative_btc = 0.0;
        
        // Calculate thresholds
        let top_1_percent_address_count = self.total_addresses / 100;
        let top_5_percent_address_count = self.total_addresses * 5 / 100;
        let top_10_percent_address_count = self.total_addresses * 10 / 100;
        
        let mut top_1_percent_threshold = 0.0;
        let mut top_5_percent_threshold = 0.0;
        let mut top_10_percent_threshold = 0.0;
        
        // Process ranges from highest to lowest
        for range in self.ranges.iter().rev() {
            cumulative_addresses += range.address_count;
            cumulative_btc += range.total_btc;
            
            if cumulative_addresses >= top_1_percent_address_count && top_1_percent_threshold == 0.0 {
                top_1_percent_threshold = range.min_btc;
            }
            if cumulative_addresses >= top_5_percent_address_count && top_5_percent_threshold == 0.0 {
                top_5_percent_threshold = range.min_btc;
            }
            if cumulative_addresses >= top_10_percent_address_count && top_10_percent_threshold == 0.0 {
                top_10_percent_threshold = range.min_btc;
            }
        }
        
        // Calculate median (simplified - would need more precise calculation)
        let median_balance = self.estimate_median();
        let mean_balance = self.total_supply / self.total_addresses as f64;
        
        // Simplified Gini coefficient calculation
        let gini_coefficient = self.calculate_gini_coefficient();
        
        // Top 1% concentration ratio
        let concentration_ratio = cumulative_btc / self.total_supply;
        
        BitcoinStatistics {
            median_balance,
            mean_balance,
            gini_coefficient,
            top_1_percent_threshold,
            top_5_percent_threshold,
            top_10_percent_threshold,
            concentration_ratio,
        }
    }
    
    #[allow(dead_code)]
    fn estimate_median(&self) -> f64 {
        let target_addresses = self.total_addresses / 2;
        let mut cumulative_addresses = 0u64;
        
        for range in &self.ranges {
            cumulative_addresses += range.address_count;
            if cumulative_addresses >= target_addresses {
                // Estimate median within this range
                return (range.min_btc + range.max_btc) / 2.0;
            }
        }
        
        0.0
    }
    
    #[allow(dead_code)]
    fn calculate_gini_coefficient(&self) -> f64 {
        // Simplified Gini coefficient calculation
        // In a real implementation, this would be more sophisticated
        let mut total_area = 0.0;
        let mut _cumulative_addresses = 0.0;
        let mut cumulative_wealth = 0.0;
        
        for range in &self.ranges {
            let address_proportion = range.address_count as f64 / self.total_addresses as f64;
            let wealth_proportion = range.total_btc / self.total_supply;
            
            total_area += address_proportion * (cumulative_wealth + wealth_proportion / 2.0);
            
            _cumulative_addresses += address_proportion;
            cumulative_wealth += wealth_proportion;
        }
        
        // Gini = 1 - 2 * area_under_lorenz_curve
        (1.0 - 2.0 * total_area).max(0.0).min(1.0)
    }
}

/// Calculate user's percentile based on Bitcoin distribution
#[allow(dead_code)]
pub fn calculate_percentile(
    user_amount: f64,
    distribution: &BitcoinDistribution,
) -> PercentileResult {
    let mut addresses_below = 0u64;
    let mut found_range = false;
    
    for range in &distribution.ranges {
        if user_amount >= range.min_btc && user_amount < range.max_btc {
            // User falls within this range
            // Estimate position within the range
            let range_progress = (user_amount - range.min_btc) / (range.max_btc - range.min_btc);
            addresses_below += (range.address_count as f64 * range_progress) as u64;
            found_range = true;
            break;
        } else if user_amount >= range.max_btc {
            addresses_below += range.address_count;
        }
    }
    
    if !found_range && user_amount > 0.0 {
        // User amount is higher than all ranges
        addresses_below = distribution.total_addresses;
    }
    
    let addresses_above = distribution.total_addresses - addresses_below;
    let percentile = (addresses_below as f64 / distribution.total_addresses as f64) * 100.0;
    let rank = addresses_above + 1;
    
    let wealth_category = WealthCategory::from_btc_amount(user_amount);
    
    PercentileResult {
        user_bitcoin_amount: user_amount,
        percentile,
        rank,
        addresses_below,
        addresses_above,
        wealth_category,
        comparison_metrics: HashMap::new(),
    }
}

impl WealthCategory {
    #[allow(dead_code)]
    pub fn from_btc_amount(amount: f64) -> Self {
        match amount {
            x if x < 0.001 => WealthCategory::Dust,
            x if x < 0.01 => WealthCategory::Shrimp,
            x if x < 0.1 => WealthCategory::Crab,
            x if x < 1.0 => WealthCategory::Fish,
            x if x < 10.0 => WealthCategory::Dolphin,
            x if x < 100.0 => WealthCategory::Shark,
            x if x < 1000.0 => WealthCategory::Whale,
            _ => WealthCategory::Humpback,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            WealthCategory::Dust => "Dust",
            WealthCategory::Shrimp => "Shrimp",
            WealthCategory::Crab => "Crab",
            WealthCategory::Fish => "Fish",
            WealthCategory::Dolphin => "Dolphin",
            WealthCategory::Shark => "Shark",
            WealthCategory::Whale => "Whale",
            WealthCategory::Humpback => "Humpback",
        }
    }
    
    /// Get the emoji representation for each wealth category
    pub fn emoji(&self) -> &'static str {
        match self {
            WealthCategory::Dust => "🟫",        // Brown square representing dust particles
            WealthCategory::Shrimp => "🦐",      // Shrimp - small sea creature
            WealthCategory::Crab => "🦀",        // Crab - medium sea creature
            WealthCategory::Fish => "🐟",        // Fish - swimming creature
            WealthCategory::Dolphin => "🐬",     // Dolphin - intelligent marine mammal
            WealthCategory::Shark => "🦈",       // Shark - powerful predator
            WealthCategory::Whale => "🐋",       // Whale - large marine creature
            WealthCategory::Humpback => "🐳",    // Special whale for ultimate tier
        }
    }
    
    /// Get a descriptive explanation of the wealth category
    pub fn description(&self) -> &'static str {
        match self {
            WealthCategory::Dust => "Minimal Bitcoin holdings - every satoshi counts!",
            WealthCategory::Shrimp => "Small but steady - you're building your Bitcoin stack!",
            WealthCategory::Crab => "Solid foundation - you're moving sideways and up!",
            WealthCategory::Fish => "Swimming with purpose - great Bitcoin accumulation!",
            WealthCategory::Dolphin => "Intelligent holder - significant Bitcoin wealth!",
            WealthCategory::Shark => "Apex Bitcoin holder - you're in the elite tier!",
            WealthCategory::Whale => "Bitcoin whale - massive holdings that move markets!",
            WealthCategory::Humpback => "Ultimate Bitcoin titan - legendary wealth status!",
        }
    }
    
    /// Get the Bitcoin range for this category as a formatted string
    pub fn btc_range(&self) -> &'static str {
        match self {
            WealthCategory::Dust => "< 0.001 BTC",
            WealthCategory::Shrimp => "0.001 - 0.01 BTC",
            WealthCategory::Crab => "0.01 - 0.1 BTC",
            WealthCategory::Fish => "0.1 - 1 BTC",
            WealthCategory::Dolphin => "1 - 10 BTC",
            WealthCategory::Shark => "10 - 100 BTC",
            WealthCategory::Whale => "100 - 1000 BTC",
            WealthCategory::Humpback => "1000+ BTC",
        }
    }
    
    #[allow(dead_code)]
    pub fn color_class(&self) -> &'static str {
        match self {
            WealthCategory::Dust => "text-gray-500",
            WealthCategory::Shrimp => "text-blue-500",
            WealthCategory::Crab => "text-green-500",
            WealthCategory::Fish => "text-yellow-500",
            WealthCategory::Dolphin => "text-orange-500",
            WealthCategory::Shark => "text-orange-500",
            WealthCategory::Whale => "text-orange-500",
            WealthCategory::Humpback => "text-red-500",
        }
    }
}
