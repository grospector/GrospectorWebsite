use serde::{Deserialize, Serialize};

/// API response wrapper
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: u64,
}

/// BitInfoCharts API response structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitInfoChartsResponse {
    pub addresses: Vec<AddressRange>,
    pub total_addresses: u64,
    pub total_supply: f64,
    pub last_updated: u64,
}

/// Address range from BitInfoCharts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressRange {
    pub range: String,
    pub addresses: u64,
    pub btc: f64,
    pub percentage: f64,
}

/// Generic API error structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
    pub details: Option<String>,
}

impl<T> ApiResponse<T> {
    #[allow(dead_code)]
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: js_sys::Date::now() as u64,
        }
    }
    
    #[allow(dead_code)]
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            timestamp: js_sys::Date::now() as u64,
        }
    }
}

/// Request configuration for API calls
#[derive(Debug, Clone)]
pub struct ApiConfig {
    #[allow(dead_code)]
    pub base_url: String,
    #[allow(dead_code)]
    pub timeout_ms: u32,
    #[allow(dead_code)]
    pub retry_count: u32,
    #[allow(dead_code)]
    pub api_key: Option<String>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            base_url: "https://bitinfocharts.com/api".to_string(),
            timeout_ms: 10000,
            retry_count: 3,
            api_key: None,
        }
    }
}
