use crate::types::bitcoin::{BitcoinDistribution, WealthRange};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_sys::console;

// BitInfoCharts API response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitInfoChartsData {
    pub addresses: Vec<AddressData>,
    pub total_addresses: u64,
    pub total_supply: f64,
    pub last_updated: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressData {
    pub range: String,
    pub addresses: u64,
    pub btc: f64,
    pub percentage: f64,
}

// Alternative API: Bitcoin Rich List
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichListEntry {
    pub rank: u32,
    pub address: String,
    pub balance: f64,
    pub percentage: f64,
}

pub struct BitcoinApiService {
    client: Client,
    base_urls: HashMap<String, String>,
}

impl BitcoinApiService {
    pub fn new() -> Self {
        let client = Client::new();
        let mut base_urls = HashMap::new();
        
        // Primary APIs
        base_urls.insert("bitinfocharts".to_string(), "https://bitinfocharts.com/api".to_string());
        base_urls.insert("blockchain_info".to_string(), "https://api.blockchain.info".to_string());
        base_urls.insert("coingecko".to_string(), "https://api.coingecko.com/api/v3".to_string());
        
        Self {
            client,
            base_urls,
        }
    }

    /// Fetch Bitcoin distribution data from multiple sources
    pub async fn fetch_bitcoin_distribution(&self) -> Result<BitcoinDistribution, String> {
        console::log_1(&"🔍 Fetching Bitcoin distribution data...".into());
        
        // Try different data sources in order of preference
        match self.fetch_from_blockchain_info().await {
            Ok(distribution) => {
                console::log_1(&"✅ Successfully fetched from blockchain.info".into());
                Ok(distribution)
            }
            Err(e) => {
                console::log_1(&format!("❌ blockchain.info failed: {}", e).into());
                
                // Fallback to mock data for development
                console::log_1(&"🔄 Using mock data for development".into());
                Ok(self.generate_mock_distribution())
            }
        }
    }

    /// Fetch data from blockchain.info API
    async fn fetch_from_blockchain_info(&self) -> Result<BitcoinDistribution, String> {
        let url = format!("{}/stats", self.base_urls["blockchain_info"]);
        
        let response = self.client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (compatible; BitcoinWealthComparison/1.0)")
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        let stats: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parsing error: {}", e))?;

        // Extract basic stats and create distribution
        let total_bitcoins = stats["total_bitcoins"].as_f64().unwrap_or(21_000_000.0);
        let estimated_addresses = stats["n_unique_addresses"].as_u64().unwrap_or(1_000_000);
        
        Ok(self.create_distribution_from_stats(total_bitcoins, estimated_addresses))
    }

    /// Create Bitcoin distribution from basic stats
    fn create_distribution_from_stats(&self, total_supply: f64, total_addresses: u64) -> BitcoinDistribution {
        // Create realistic distribution based on known Bitcoin wealth patterns
        let ranges = vec![
            WealthRange {
                min_btc: 0.0,
                max_btc: 0.001,
                address_count: (total_addresses as f64 * 0.40) as u64,
                total_btc: total_supply * 0.001,
                percentage_of_addresses: 40.0,
                percentage_of_supply: 0.1,
            },
            WealthRange {
                min_btc: 0.001,
                max_btc: 0.01,
                address_count: (total_addresses as f64 * 0.25) as u64,
                total_btc: total_supply * 0.002,
                percentage_of_addresses: 25.0,
                percentage_of_supply: 0.2,
            },
            WealthRange {
                min_btc: 0.01,
                max_btc: 0.1,
                address_count: (total_addresses as f64 * 0.20) as u64,
                total_btc: total_supply * 0.005,
                percentage_of_addresses: 20.0,
                percentage_of_supply: 0.5,
            },
            WealthRange {
                min_btc: 0.1,
                max_btc: 1.0,
                address_count: (total_addresses as f64 * 0.10) as u64,
                total_btc: total_supply * 0.015,
                percentage_of_addresses: 10.0,
                percentage_of_supply: 1.5,
            },
            WealthRange {
                min_btc: 1.0,
                max_btc: 10.0,
                address_count: (total_addresses as f64 * 0.035) as u64,
                total_btc: total_supply * 0.05,
                percentage_of_addresses: 3.5,
                percentage_of_supply: 5.0,
            },
            WealthRange {
                min_btc: 10.0,
                max_btc: 100.0,
                address_count: (total_addresses as f64 * 0.012) as u64,
                total_btc: total_supply * 0.12,
                percentage_of_addresses: 1.2,
                percentage_of_supply: 12.0,
            },
            WealthRange {
                min_btc: 100.0,
                max_btc: 1000.0,
                address_count: (total_addresses as f64 * 0.002) as u64,
                total_btc: total_supply * 0.20,
                percentage_of_addresses: 0.2,
                percentage_of_supply: 20.0,
            },
            WealthRange {
                min_btc: 1000.0,
                max_btc: 10000.0,
                address_count: (total_addresses as f64 * 0.001) as u64,
                total_btc: total_supply * 0.25,
                percentage_of_addresses: 0.1,
                percentage_of_supply: 25.0,
            },
            WealthRange {
                min_btc: 10000.0,
                max_btc: f64::INFINITY,
                address_count: (total_addresses as f64 * 0.0001) as u64,
                total_btc: total_supply * 0.357,
                percentage_of_addresses: 0.01,
                percentage_of_supply: 35.7,
            },
        ];

        BitcoinDistribution {
            ranges,
            total_addresses,
            total_supply,
            timestamp: js_sys::Date::now() as u64,
            data_source: "blockchain.info + estimated distribution".to_string(),
        }
    }

    /// Generate mock distribution for development
    fn generate_mock_distribution(&self) -> BitcoinDistribution {
        let ranges = vec![
            WealthRange {
                min_btc: 0.0,
                max_btc: 0.001,
                address_count: 25_000_000,
                total_btc: 2_500.0,
                percentage_of_addresses: 62.5,
                percentage_of_supply: 0.012,
            },
            WealthRange {
                min_btc: 0.001,
                max_btc: 0.01,
                address_count: 8_000_000,
                total_btc: 40_000.0,
                percentage_of_addresses: 20.0,
                percentage_of_supply: 0.19,
            },
            WealthRange {
                min_btc: 0.01,
                max_btc: 0.1,
                address_count: 4_000_000,
                total_btc: 200_000.0,
                percentage_of_addresses: 10.0,
                percentage_of_supply: 0.95,
            },
            WealthRange {
                min_btc: 0.1,
                max_btc: 1.0,
                address_count: 2_000_000,
                total_btc: 1_000_000.0,
                percentage_of_addresses: 5.0,
                percentage_of_supply: 4.76,
            },
            WealthRange {
                min_btc: 1.0,
                max_btc: 10.0,
                address_count: 800_000,
                total_btc: 4_000_000.0,
                percentage_of_addresses: 2.0,
                percentage_of_supply: 19.05,
            },
            WealthRange {
                min_btc: 10.0,
                max_btc: 100.0,
                address_count: 150_000,
                total_btc: 7_500_000.0,
                percentage_of_addresses: 0.375,
                percentage_of_supply: 35.71,
            },
            WealthRange {
                min_btc: 100.0,
                max_btc: 1000.0,
                address_count: 40_000,
                total_btc: 4_000_000.0,
                percentage_of_addresses: 0.1,
                percentage_of_supply: 19.05,
            },
            WealthRange {
                min_btc: 1000.0,
                max_btc: 10000.0,
                address_count: 2_000,
                total_btc: 2_000_000.0,
                percentage_of_addresses: 0.005,
                percentage_of_supply: 9.52,
            },
            WealthRange {
                min_btc: 10000.0,
                max_btc: f64::INFINITY,
                address_count: 100,
                total_btc: 2_257_500.0,
                percentage_of_addresses: 0.0025,
                percentage_of_supply: 10.75,
            },
        ];

        BitcoinDistribution {
            ranges,
            total_addresses: 40_000_000,
            total_supply: 21_000_000.0,
            timestamp: js_sys::Date::now() as u64,
            data_source: "mock_data_for_development".to_string(),
        }
    }

    /// Get the latest Bitcoin price (for context)
    pub async fn fetch_bitcoin_price(&self) -> Result<f64, String> {
        let url = format!("{}/simple/price?ids=bitcoin&vs_currencies=usd", self.base_urls["coingecko"]);
        
        let response = self.client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (compatible; BitcoinWealthComparison/1.0)")
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if !response.status().is_success() {
            return Ok(50000.0); // Fallback price
        }

        let price_data: serde_json::Value = response
            .json()
            .await
            .map_err(|_| "Failed to parse price data".to_string())?;

        Ok(price_data["bitcoin"]["usd"].as_f64().unwrap_or(50000.0))
    }

    /// Get Bitcoin network statistics
    pub async fn fetch_network_stats(&self) -> Result<HashMap<String, f64>, String> {
        let mut stats = HashMap::new();
        
        // Mock network stats for development
        stats.insert("total_supply".to_string(), 19_500_000.0);
        stats.insert("circulating_supply".to_string(), 19_500_000.0);
        stats.insert("total_addresses".to_string(), 40_000_000.0);
        stats.insert("active_addresses".to_string(), 1_000_000.0);
        stats.insert("hash_rate".to_string(), 400_000_000.0);
        stats.insert("difficulty".to_string(), 48_000_000_000_000.0);
        
        Ok(stats)
    }
}

impl Default for BitcoinApiService {
    fn default() -> Self {
        Self::new()
    }
}
