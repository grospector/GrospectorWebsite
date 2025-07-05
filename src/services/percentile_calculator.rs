use crate::types::bitcoin::{BitcoinDistribution, PercentileResult, WealthCategory};
use crate::services::data_processor::DataProcessor;
use crate::utils::validators::validate_bitcoin_amount;
use crate::utils::formatters::{format_large_number, format_rank};
use web_sys::console;

pub struct PercentileCalculator {
    data_processor: DataProcessor,
}

impl PercentileCalculator {
    pub fn new() -> Self {
        Self {
            data_processor: DataProcessor::new(),
        }
    }
    
    /// Calculate detailed percentile information for a user's Bitcoin amount
    pub fn calculate_user_percentile(
        &self,
        user_amount: f64,
        distribution: &BitcoinDistribution,
    ) -> Result<PercentileResult, String> {
        // Validate input
        validate_bitcoin_amount(user_amount)?;
        
        // Validate distribution
        self.data_processor.validate_distribution(distribution)?;
        
        console::log_1(&format!("🧮 Calculating percentile for {} BTC", user_amount).into());
        
        // Find the range containing the user's amount
        let containing_range = self.data_processor.find_range_for_amount(user_amount, distribution);
        
        if containing_range.is_none() {
            return Err("User amount does not fit in any distribution range".to_string());
        }
        
        let range = containing_range.unwrap();
        
        // Calculate percentile within the range
        let percentile = self.calculate_percentile_in_range(user_amount, range, distribution);
        
        // Calculate rank (how many people have less Bitcoin)
        let addresses_below = self.calculate_addresses_below(user_amount, distribution);
        let rank = distribution.total_addresses - addresses_below;
        
        // Calculate addresses above
        let addresses_above = addresses_below;
        
        // Determine wealth category
        let wealth_category = self.determine_wealth_category(user_amount);
        
        // Calculate comparative metrics
        let comparison_metrics = self.calculate_comparison_metrics(user_amount, distribution);
        
        console::log_1(&format!("✅ Percentile calculation complete: {:.2}%", percentile).into());
        
        Ok(PercentileResult {
            user_bitcoin_amount: user_amount,
            percentile,
            rank,
            addresses_below,
            addresses_above,
            wealth_category,
            comparison_metrics,
        })
    }
    
    /// Calculate percentile within a specific range
    fn calculate_percentile_in_range(
        &self,
        user_amount: f64,
        range: &crate::types::bitcoin::WealthRange,
        distribution: &BitcoinDistribution,
    ) -> f64 {
        // Calculate how many addresses are below this range
        let addresses_below_range = distribution.ranges.iter()
            .filter(|r| r.max_btc <= range.min_btc)
            .map(|r| r.address_count)
            .sum::<u64>();
        
        // Calculate position within the range
        let position_in_range = if range.max_btc == f64::INFINITY {
            // For the highest range, assume exponential distribution
            self.calculate_position_in_infinite_range(user_amount, range)
        } else {
            // Linear interpolation within the range
            (user_amount - range.min_btc) / (range.max_btc - range.min_btc)
        };
        
        let addresses_below_in_range = (range.address_count as f64 * position_in_range) as u64;
        let total_addresses_below = addresses_below_range + addresses_below_in_range;
        
        // Calculate percentile
        (total_addresses_below as f64 / distribution.total_addresses as f64) * 100.0
    }
    
    /// Calculate position within an infinite range (for whale category)
    fn calculate_position_in_infinite_range(&self, user_amount: f64, range: &crate::types::bitcoin::WealthRange) -> f64 {
        // Use logarithmic scale for infinite range
        // Assume the range follows a power law distribution
        let log_min = range.min_btc.ln();
        let log_user = user_amount.ln();
        
        // Estimate the "effective maximum" based on the distribution
        let effective_max = range.min_btc * 100.0; // Assume 100x as effective maximum
        let log_max = effective_max.ln();
        
        ((log_user - log_min) / (log_max - log_min)).min(0.99) // Cap at 99%
    }
    
    /// Calculate how many addresses have less Bitcoin
    fn calculate_addresses_below(&self, user_amount: f64, distribution: &BitcoinDistribution) -> u64 {
        let mut addresses_below = 0u64;
        
        for range in &distribution.ranges {
            if range.max_btc <= user_amount {
                // Entire range is below user amount
                addresses_below += range.address_count;
            } else if range.min_btc < user_amount && user_amount < range.max_btc {
                // User amount is within this range
                let position = if range.max_btc == f64::INFINITY {
                    self.calculate_position_in_infinite_range(user_amount, range)
                } else {
                    (user_amount - range.min_btc) / (range.max_btc - range.min_btc)
                };
                addresses_below += (range.address_count as f64 * position) as u64;
            }
            // If range.min_btc >= user_amount, skip (entire range is above)
        }
        
        addresses_below
    }
    
    /// Determine wealth category based on Bitcoin amount
    fn determine_wealth_category(&self, amount: f64) -> WealthCategory {
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
    
    /// Calculate comparative metrics
    fn calculate_comparison_metrics(&self, user_amount: f64, distribution: &BitcoinDistribution) -> std::collections::HashMap<String, f64> {
        let mut metrics = std::collections::HashMap::new();
        
        // Calculate statistics
        let stats = self.data_processor.calculate_statistics(distribution);
        
        // Compare to median
        let median = stats.get("median_amount").unwrap_or(&0.0);
        metrics.insert("vs_median_ratio".to_string(), if *median > 0.0 { user_amount / median } else { 0.0 });
        
        // Compare to mean
        let mean = stats.get("mean_amount").unwrap_or(&0.0);
        metrics.insert("vs_mean_ratio".to_string(), if *mean > 0.0 { user_amount / mean } else { 0.0 });
        
        // Share of total supply
        let supply_share = (user_amount / distribution.total_supply) * 100.0;
        metrics.insert("supply_share_percent".to_string(), supply_share);
        
        // Equivalent addresses (how many median holders equal your amount)
        let equivalent_addresses = if *median > 0.0 { user_amount / median } else { 0.0 };
        metrics.insert("equivalent_median_addresses".to_string(), equivalent_addresses);
        
        // Time to accumulate at different rates (in years)
        let daily_rates = [0.001, 0.01, 0.1]; // BTC per day
        for rate in daily_rates {
            let days = user_amount / rate;
            let years = days / 365.0;
            metrics.insert(format!("years_to_accumulate_at_{}_btc_per_day", rate), years);
        }
        
        // Dollar value estimates (using approximate price)
        let btc_price = 50000.0; // Approximate BTC price in USD
        let dollar_value = user_amount * btc_price;
        metrics.insert("estimated_usd_value".to_string(), dollar_value);
        
        metrics
    }
    
    /// Calculate percentile thresholds for common percentiles
    pub fn calculate_percentile_thresholds(
        &self,
        distribution: &BitcoinDistribution,
    ) -> Result<Vec<(f64, f64)>, String> {
        self.data_processor.validate_distribution(distribution)?;
        
        let percentiles = vec![1.0, 5.0, 10.0, 25.0, 50.0, 75.0, 90.0, 95.0, 99.0, 99.9];
        let mut thresholds = Vec::new();
        
        for percentile in percentiles {
            let amount = self.calculate_amount_at_percentile(percentile, distribution);
            thresholds.push((percentile, amount));
        }
        
        Ok(thresholds)
    }
    
    /// Calculate the Bitcoin amount at a specific percentile
    fn calculate_amount_at_percentile(&self, percentile: f64, distribution: &BitcoinDistribution) -> f64 {
        let mut sorted_ranges = distribution.ranges.clone();
        self.data_processor.sort_ranges(&mut sorted_ranges);
        
        let mut cumulative_addresses = 0.0;
        
        for range in sorted_ranges {
            let new_cumulative = cumulative_addresses + range.percentage_of_addresses;
            
            if new_cumulative >= percentile {
                // Interpolate within the range
                let remaining = percentile - cumulative_addresses;
                let position = remaining / range.percentage_of_addresses;
                
                if range.max_btc == f64::INFINITY {
                    // For infinite range, use logarithmic interpolation
                    let log_min = range.min_btc.ln();
                    let effective_max = range.min_btc * 100.0; // Assume 100x span
                    let log_max = effective_max.ln();
                    let log_amount = log_min + position * (log_max - log_min);
                    return log_amount.exp();
                } else {
                    return range.min_btc + position * (range.max_btc - range.min_btc);
                }
            }
            
            cumulative_addresses = new_cumulative;
        }
        
        // If we get here, return the maximum available
        distribution.ranges.iter()
            .map(|r| if r.max_btc == f64::INFINITY { r.min_btc * 100.0 } else { r.max_btc })
            .fold(0.0, |a, b| a.max(b))
    }
    
    /// Calculate wealth concentration analysis
    pub fn calculate_wealth_concentration(&self, distribution: &BitcoinDistribution) -> Result<std::collections::HashMap<String, f64>, String> {
        self.data_processor.validate_distribution(distribution)?;
        
        let mut concentration = std::collections::HashMap::new();
        
        // Calculate Gini coefficient
        let stats = self.data_processor.calculate_statistics(distribution);
        if let Some(gini) = stats.get("gini_coefficient") {
            concentration.insert("gini_coefficient".to_string(), *gini);
        }
        
        // Calculate concentration ratios
        let concentration_levels = vec![0.1, 0.5, 1.0, 5.0, 10.0, 25.0];
        for level in concentration_levels {
            let key = format!("top_{}percent_wealth", level);
            let wealth_share = self.calculate_top_percent_wealth(level, distribution);
            concentration.insert(key, wealth_share);
        }
        
        // Calculate Herfindahl-Hirschman Index (HHI) for concentration
        let hhi = self.calculate_hhi(distribution);
        concentration.insert("hhi_index".to_string(), hhi);
        
        Ok(concentration)
    }
    
    /// Calculate what percentage of total wealth is held by top X% of addresses
    fn calculate_top_percent_wealth(&self, top_percent: f64, distribution: &BitcoinDistribution) -> f64 {
        let mut sorted_ranges = distribution.ranges.clone();
        // Sort by wealth (highest first)
        sorted_ranges.sort_by(|a, b| b.min_btc.partial_cmp(&a.min_btc).unwrap());
        
        let mut cumulative_addresses = 0.0;
        let mut cumulative_wealth = 0.0;
        
        for range in sorted_ranges {
            let new_cumulative_addresses = cumulative_addresses + range.percentage_of_addresses;
            
            if new_cumulative_addresses <= top_percent {
                cumulative_wealth += range.percentage_of_supply;
                cumulative_addresses = new_cumulative_addresses;
            } else {
                // Partial range
                let remaining_addresses = top_percent - cumulative_addresses;
                let partial_wealth = range.percentage_of_supply * (remaining_addresses / range.percentage_of_addresses);
                cumulative_wealth += partial_wealth;
                break;
            }
        }
        
        cumulative_wealth
    }
    
    /// Calculate Herfindahl-Hirschman Index for wealth concentration
    fn calculate_hhi(&self, distribution: &BitcoinDistribution) -> f64 {
        distribution.ranges.iter()
            .map(|range| {
                let market_share = range.percentage_of_supply / 100.0;
                market_share * market_share
            })
            .sum::<f64>() * 10000.0 // Scale to 0-10000 range
    }
    
    /// Generate a wealth report for a user
    #[allow(dead_code)]
    pub fn generate_wealth_report(&self, user_amount: f64, distribution: &BitcoinDistribution) -> Result<String, String> {
        let result = self.calculate_user_percentile(user_amount, distribution)?;
        let thresholds = self.calculate_percentile_thresholds(distribution)?;
        
        let mut report = String::new();
        
        report.push_str(&format!("🏆 Bitcoin Wealth Report\n"));
        report.push_str(&format!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"));
        report.push_str(&format!("Your Bitcoin: {:.8} BTC\n", user_amount));
        report.push_str(&format!("Percentile: {:.2}%\n", result.percentile));
        report.push_str(&format!("Rank: {} out of {}\n", format_rank(result.rank), format_large_number(distribution.total_addresses as f64)));
        report.push_str(&format!("Category: {:?}\n", result.wealth_category));
        report.push_str(&format!("\n"));
        
        report.push_str(&format!("📊 Comparison:\n"));
        report.push_str(&format!("• {} addresses have less Bitcoin than you\n", format_large_number(result.addresses_below as f64)));
        report.push_str(&format!("• {} addresses have more Bitcoin than you\n", format_large_number(result.addresses_above as f64)));
        
        if let Some(vs_median) = result.comparison_metrics.get("vs_median_ratio") {
            report.push_str(&format!("• You have {:.1}x the median amount\n", vs_median));
        }
        
        if let Some(supply_share) = result.comparison_metrics.get("supply_share_percent") {
            report.push_str(&format!("• You own {:.6}% of total Bitcoin supply\n", supply_share));
        }
        
        report.push_str(&format!("\n"));
        report.push_str(&format!("🎯 Key Thresholds:\n"));
        
        for (percentile, amount) in thresholds {
            report.push_str(&format!("• {:.1}th percentile: {:.8} BTC\n", percentile, amount));
        }
        
        Ok(report)
    }
}

impl Default for PercentileCalculator {
    fn default() -> Self {
        Self::new()
    }
}
