use crate::types::bitcoin::{BitcoinDistribution, WealthRange};
use crate::utils::validators::validate_bitcoin_amount;
use std::collections::HashMap;
use web_sys::console;

pub struct DataProcessor {
    #[allow(dead_code)]
    cache: HashMap<String, BitcoinDistribution>,
}

impl DataProcessor {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    
    /// Process raw Bitcoin distribution data
    #[allow(dead_code)]
    pub fn process_raw_data(&self, _raw_data: &str) -> Result<BitcoinDistribution, String> {
        // This can be extended to parse different data formats
        // For now, we rely on the API service to provide structured data
        Err("Direct raw data processing not implemented - use API service".to_string())
    }
    
    /// Validate Bitcoin distribution data
    pub fn validate_distribution(&self, distribution: &BitcoinDistribution) -> Result<(), String> {
        if distribution.ranges.is_empty() {
            return Err("Distribution must have at least one range".to_string());
        }
        
        if distribution.total_addresses == 0 {
            return Err("Total addresses cannot be zero".to_string());
        }
        
        if distribution.total_supply <= 0.0 {
            return Err("Total supply must be positive".to_string());
        }
        
        if distribution.total_supply > 21_000_000.0 {
            return Err("Total supply cannot exceed 21 million BTC".to_string());
        }
        
        // Validate each range
        for (i, range) in distribution.ranges.iter().enumerate() {
            if range.min_btc < 0.0 {
                return Err(format!("Range {} has negative minimum", i));
            }
            
            if range.max_btc <= range.min_btc && range.max_btc != f64::INFINITY {
                return Err(format!("Range {} has invalid bounds", i));
            }
            
            if range.address_count == 0 {
                return Err(format!("Range {} has zero addresses", i));
            }
            
            if range.total_btc < 0.0 {
                return Err(format!("Range {} has negative Bitcoin amount", i));
            }
            
            if range.percentage_of_addresses < 0.0 || range.percentage_of_addresses > 100.0 {
                return Err(format!("Range {} has invalid address percentage", i));
            }
            
            if range.percentage_of_supply < 0.0 || range.percentage_of_supply > 100.0 {
                return Err(format!("Range {} has invalid supply percentage", i));
            }
        }
        
        // Validate range continuity
        self.validate_range_continuity(&distribution.ranges)?;
        
        // Validate totals
        self.validate_totals(distribution)?;
        
        console::log_1(&"✅ Distribution validation passed".into());
        Ok(())
    }
    
    /// Validate that ranges are continuous and non-overlapping
    fn validate_range_continuity(&self, ranges: &[WealthRange]) -> Result<(), String> {
        let mut sorted_ranges = ranges.to_vec();
        sorted_ranges.sort_by(|a, b| a.min_btc.partial_cmp(&b.min_btc).unwrap());
        
        for i in 0..sorted_ranges.len() - 1 {
            let current = &sorted_ranges[i];
            let next = &sorted_ranges[i + 1];
            
            if current.max_btc != next.min_btc && current.max_btc != f64::INFINITY {
                console::log_1(&format!("⚠️  Range discontinuity detected between {} and {}", 
                    current.max_btc, next.min_btc).into());
                // This is a warning, not an error for now
            }
        }
        
        Ok(())
    }
    
    /// Validate that totals are consistent
    fn validate_totals(&self, distribution: &BitcoinDistribution) -> Result<(), String> {
        let total_addresses: u64 = distribution.ranges.iter()
            .map(|r| r.address_count)
            .sum();
            
        let total_btc: f64 = distribution.ranges.iter()
            .map(|r| r.total_btc)
            .sum();
            
        let total_address_percentage: f64 = distribution.ranges.iter()
            .map(|r| r.percentage_of_addresses)
            .sum();
            
        let total_supply_percentage: f64 = distribution.ranges.iter()
            .map(|r| r.percentage_of_supply)
            .sum();
        
        // Allow for small rounding errors
        const TOLERANCE: f64 = 0.01;
        
        if (total_addresses as f64 - distribution.total_addresses as f64).abs() > distribution.total_addresses as f64 * TOLERANCE {
            console::log_1(&format!("⚠️  Address count mismatch: {} vs {}", 
                total_addresses, distribution.total_addresses).into());
        }
        
        if (total_btc - distribution.total_supply).abs() > distribution.total_supply * TOLERANCE {
            console::log_1(&format!("⚠️  Bitcoin amount mismatch: {} vs {}", 
                total_btc, distribution.total_supply).into());
        }
        
        if (total_address_percentage - 100.0).abs() > TOLERANCE {
            console::log_1(&format!("⚠️  Address percentage sum: {}%", total_address_percentage).into());
        }
        
        if (total_supply_percentage - 100.0).abs() > TOLERANCE {
            console::log_1(&format!("⚠️  Supply percentage sum: {}%", total_supply_percentage).into());
        }
        
        Ok(())
    }
    
    /// Sort ranges by minimum Bitcoin amount
    pub fn sort_ranges(&self, ranges: &mut Vec<WealthRange>) {
        ranges.sort_by(|a, b| a.min_btc.partial_cmp(&b.min_btc).unwrap());
    }
    
    /// Merge overlapping ranges
    #[allow(dead_code)]
    pub fn merge_ranges(&self, ranges: Vec<WealthRange>) -> Vec<WealthRange> {
        if ranges.is_empty() {
            return ranges;
        }
        
        let mut sorted_ranges = ranges;
        self.sort_ranges(&mut sorted_ranges);
        
        let mut merged: Vec<WealthRange> = Vec::new();
        let mut current = sorted_ranges[0].clone();
        
        for range in sorted_ranges.iter().skip(1) {
            if current.max_btc >= range.min_btc {
                // Merge overlapping ranges
                current.max_btc = current.max_btc.max(range.max_btc);
                current.address_count += range.address_count;
                current.total_btc += range.total_btc;
                current.percentage_of_addresses += range.percentage_of_addresses;
                current.percentage_of_supply += range.percentage_of_supply;
            } else {
                merged.push(current.clone());
                current = range.clone();
            }
        }
        
        merged.push(current);
        merged
    }
    
    /// Calculate cumulative distribution
    #[allow(dead_code)]
    pub fn calculate_cumulative_distribution(&self, distribution: &BitcoinDistribution) -> Vec<(f64, f64, f64)> {
        let mut sorted_ranges = distribution.ranges.clone();
        self.sort_ranges(&mut sorted_ranges);
        
        let mut cumulative: Vec<(f64, f64, f64)> = Vec::new();
        let mut cum_addresses = 0.0;
        let mut cum_btc = 0.0;
        
        for range in sorted_ranges {
            cum_addresses += range.percentage_of_addresses;
            cum_btc += range.percentage_of_supply;
            
            // (bitcoin_amount, cumulative_address_percentage, cumulative_supply_percentage)
            cumulative.push((range.max_btc, cum_addresses, cum_btc));
        }
        
        cumulative
    }
    
    /// Find the range that contains a specific Bitcoin amount
    pub fn find_range_for_amount<'a>(&self, amount: f64, distribution: &'a BitcoinDistribution) -> Option<&'a WealthRange> {
        // Validate input
        if validate_bitcoin_amount(amount).is_err() {
            return None;
        }
        
        distribution.ranges.iter()
            .find(|range| amount >= range.min_btc && amount < range.max_btc)
    }
    
    /// Calculate statistics for the distribution
    pub fn calculate_statistics(&self, distribution: &BitcoinDistribution) -> HashMap<String, f64> {
        let mut stats = HashMap::new();
        
        // Basic statistics
        stats.insert("total_addresses".to_string(), distribution.total_addresses as f64);
        stats.insert("total_supply".to_string(), distribution.total_supply);
        stats.insert("total_ranges".to_string(), distribution.ranges.len() as f64);
        
        // Calculate weighted averages
        let mut total_weighted_amount = 0.0;
        let mut total_addresses = 0.0;
        
        for range in &distribution.ranges {
            let avg_amount = (range.min_btc + range.max_btc) / 2.0;
            total_weighted_amount += avg_amount * range.address_count as f64;
            total_addresses += range.address_count as f64;
        }
        
        let mean_amount = if total_addresses > 0.0 {
            total_weighted_amount / total_addresses
        } else {
            0.0
        };
        
        stats.insert("mean_amount".to_string(), mean_amount);
        
        // Calculate Gini coefficient (wealth inequality measure)
        let gini = self.calculate_gini_coefficient(distribution);
        stats.insert("gini_coefficient".to_string(), gini);
        
        // Calculate concentration ratios
        let top_1_percent = self.calculate_concentration_ratio(distribution, 1.0);
        let top_5_percent = self.calculate_concentration_ratio(distribution, 5.0);
        let top_10_percent = self.calculate_concentration_ratio(distribution, 10.0);
        
        stats.insert("top_1_percent_wealth".to_string(), top_1_percent);
        stats.insert("top_5_percent_wealth".to_string(), top_5_percent);
        stats.insert("top_10_percent_wealth".to_string(), top_10_percent);
        
        // Calculate median (50th percentile)
        let median = self.calculate_percentile_amount(distribution, 50.0);
        stats.insert("median_amount".to_string(), median);
        
        stats
    }
    
    /// Calculate Gini coefficient for wealth inequality
    fn calculate_gini_coefficient(&self, distribution: &BitcoinDistribution) -> f64 {
        let mut sorted_ranges = distribution.ranges.clone();
        self.sort_ranges(&mut sorted_ranges);
        
        let mut total_area = 0.0;
        let mut _cumulative_addresses = 0.0;
        let mut cumulative_wealth = 0.0;
        
        for range in sorted_ranges {
            let address_proportion = range.percentage_of_addresses / 100.0;
            let wealth_proportion = range.percentage_of_supply / 100.0;
            
            // Calculate area under Lorenz curve
            total_area += address_proportion * (cumulative_wealth + wealth_proportion / 2.0);
            
            _cumulative_addresses += address_proportion;
            cumulative_wealth += wealth_proportion;
        }
        
        // Gini coefficient = 1 - 2 * (area under Lorenz curve)
        let gini = 1.0 - 2.0 * total_area;
        gini.max(0.0).min(1.0) // Clamp to [0, 1]
    }
    
    /// Calculate concentration ratio (what percentage of wealth is held by top X% of addresses)
    fn calculate_concentration_ratio(&self, distribution: &BitcoinDistribution, top_percent: f64) -> f64 {
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
    
    /// Calculate the Bitcoin amount at a specific percentile
    fn calculate_percentile_amount(&self, distribution: &BitcoinDistribution, percentile: f64) -> f64 {
        let mut sorted_ranges = distribution.ranges.clone();
        self.sort_ranges(&mut sorted_ranges);
        
        let mut cumulative_addresses = 0.0;
        
        for range in sorted_ranges {
            let new_cumulative = cumulative_addresses + range.percentage_of_addresses;
            
            if new_cumulative >= percentile {
                // Interpolate within the range
                let remaining = percentile - cumulative_addresses;
                let position = remaining / range.percentage_of_addresses;
                return range.min_btc + position * (range.max_btc - range.min_btc);
            }
            
            cumulative_addresses = new_cumulative;
        }
        
        // If we get here, return the maximum
        distribution.ranges.iter()
            .map(|r| r.max_btc)
            .fold(0.0, |a, b| a.max(b))
    }
    
    /// Cache distribution data
    #[allow(dead_code)]
    pub fn cache_distribution(&mut self, key: String, distribution: BitcoinDistribution) {
        self.cache.insert(key, distribution);
    }
    
    /// Get cached distribution
    #[allow(dead_code)]
    pub fn get_cached_distribution(&self, key: &str) -> Option<&BitcoinDistribution> {
        self.cache.get(key)
    }
    
    /// Clear cache
    #[allow(dead_code)]
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for DataProcessor {
    fn default() -> Self {
        Self::new()
    }
}
