use plotters::prelude::*;
use web_sys::window;

/// Mempool.space inspired chart theme with Bitcoin orange accents
#[derive(Clone, Debug)]
pub struct MempoolChartTheme {
    // Background colors
    pub background: RGBColor,
    pub card_background: RGBColor,
    #[allow(dead_code)]
    pub tertiary_background: RGBColor,

    // Text colors
    pub text_primary: RGBColor,
    pub text_secondary: RGBColor,
    pub text_muted: RGBColor,

    // Border and grid colors
    pub border_primary: RGBColor,
    pub border_secondary: RGBColor,
    pub grid_color: RGBColor,

    // Bitcoin brand colors
    pub bitcoin_orange: RGBColor,
    #[allow(dead_code)]
    pub bitcoin_orange_hover: RGBColor,
    #[allow(dead_code)]
    pub bitcoin_orange_muted: RGBColor,

    // Chart specific colors
    #[allow(dead_code)]
    pub chart_primary: RGBColor,
    pub chart_secondary: RGBColor,
    pub chart_accent: RGBColor,
    #[allow(dead_code)]
    pub chart_success: RGBColor,
    pub chart_warning: RGBColor,
    #[allow(dead_code)]
    pub chart_error: RGBColor,
}

impl MempoolChartTheme {
    /// Create a new theme based on current CSS custom properties
    pub fn new() -> Self {
        if is_dark_theme() {
            Self::dark_theme()
        } else {
            Self::light_theme()
        }
    }

    /// Beautiful modern dark theme perfectly matching our CSS
    pub fn dark_theme() -> Self {
        Self {
            // Perfect match with CSS --bg-primary, --bg-secondary, --bg-tertiary
            background: RGBColor(15, 23, 42),        // #0f172a - matches CSS --bg-primary
            card_background: RGBColor(30, 41, 59),   // #1e293b - matches CSS --bg-secondary  
            tertiary_background: RGBColor(51, 65, 85), // #334155 - matches CSS --bg-tertiary

            // Exact match with CSS text colors for consistency
            text_primary: RGBColor(248, 250, 252),   // #f8fafc - matches CSS --text-primary
            text_secondary: RGBColor(226, 232, 240), // #e2e8f0 - matches CSS --text-secondary
            text_muted: RGBColor(148, 163, 184),     // #94a3b8 - matches CSS --text-muted

            // Borders matching CSS border system for consistency  
            border_primary: RGBColor(148, 163, 184), // #94a3b8 - matches CSS border opacity
            border_secondary: RGBColor(71, 85, 105), // #475569 - slightly more visible
            grid_color: RGBColor(100, 116, 139),     // #64748b - subtle grid lines

            // Bitcoin orange exactly matching CSS
            bitcoin_orange: RGBColor(247, 147, 26), // #F7931A - perfect match
            bitcoin_orange_hover: RGBColor(255, 159, 46), // #ff9f2e - hover state
            bitcoin_orange_muted: RGBColor(247, 147, 26), // #F7931A

            // Chart colors optimized for dark theme readability
            chart_primary: RGBColor(247, 147, 26), // Bitcoin orange - primary
            chart_secondary: RGBColor(59, 130, 246), // #3b82f6 - matches CSS info color
            chart_accent: RGBColor(16, 185, 129),  // #10b981 - matches CSS success
            chart_success: RGBColor(16, 185, 129), // #10b981 - success green
            chart_warning: RGBColor(245, 158, 11), // #f59e0b - warning orange
            chart_error: RGBColor(239, 68, 68),    // #ef4444 - error red
        }
    }

    /// Light theme with professional styling
    pub fn light_theme() -> Self {
        Self {
            // Clean light backgrounds
            background: RGBColor(255, 255, 255),      // #ffffff
            card_background: RGBColor(249, 250, 251), // #f9fafb
            tertiary_background: RGBColor(243, 244, 246), // #f3f4f6

            // Dark text for contrast
            text_primary: RGBColor(17, 24, 39),   // #111827
            text_secondary: RGBColor(55, 65, 81), // #374151
            text_muted: RGBColor(107, 114, 128),  // #6b7280

            // Light borders and grids
            border_primary: RGBColor(229, 231, 235), // #e5e7eb
            border_secondary: RGBColor(209, 213, 219), // #d1d5db
            grid_color: RGBColor(229, 231, 235),     // #e5e7eb

            // Bitcoin orange brand colors
            bitcoin_orange: RGBColor(247, 147, 26), // #F7931A
            bitcoin_orange_hover: RGBColor(230, 133, 9), // #e68509
            bitcoin_orange_muted: RGBColor(247, 147, 26), // #F7931A with alpha

            // Chart data colors
            chart_primary: RGBColor(247, 147, 26), // Bitcoin orange
            chart_secondary: RGBColor(59, 130, 246), // Blue
            chart_accent: RGBColor(16, 185, 129),  // Green
            chart_success: RGBColor(16, 185, 129), // #10b981
            chart_warning: RGBColor(245, 158, 11), // #f59e0b
            chart_error: RGBColor(239, 68, 68),    // #ef4444
        }
    }

    /// Get gradient colors for enhanced visual effects
    pub fn get_gradient_colors(&self) -> Vec<RGBColor> {
        vec![
            self.bitcoin_orange,
            RGBColor(255, 179, 86),  // Lighter orange
            RGBColor(255, 203, 134), // Even lighter orange
        ]
    }

    /// Get wealth category colors (for distribution visualization)
    pub fn get_wealth_colors(&self) -> Vec<(String, RGBColor)> {
        vec![
            ("Shrimp".to_string(), self.chart_accent),       // Green
            ("Crab".to_string(), self.chart_warning),        // Yellow/Orange
            ("Fish".to_string(), self.chart_secondary),      // Blue
            ("Dolphin".to_string(), RGBColor(139, 92, 246)), // Purple
            ("Shark".to_string(), RGBColor(236, 72, 153)),   // Pink
            ("Whale".to_string(), self.bitcoin_orange),      // Bitcoin Orange
        ]
    }

    /// Create a styled text element with proper font
    pub fn create_text_style(&self, size: i32) -> TextStyle<'static> {
        ("Inter", size).into_font().color(&self.text_primary)
    }

    /// Create a muted text style
    pub fn create_muted_text_style(&self, size: i32) -> TextStyle<'static> {
        ("Inter", size).into_font().color(&self.text_muted)
    }

    /// Create a secondary text style
    pub fn create_secondary_text_style(&self, size: i32) -> TextStyle<'static> {
        ("Inter", size).into_font().color(&self.text_secondary)
    }
}

/// Check if dark theme is currently active by reading CSS custom properties
pub fn is_dark_theme() -> bool {
    let window = match window() {
        Some(w) => w,
        None => return false,
    };

    let document = match window.document() {
        Some(d) => d,
        None => return false,
    };

    let html_element = match document.document_element() {
        Some(e) => e,
        None => return false,
    };

    // Check data-theme attribute first (most reliable)
    if let Some(theme) = html_element.get_attribute("data-theme") {
        return theme == "dark";
    }

    // Check class attribute as fallback
    if let Some(class) = html_element.get_attribute("class") {
        return class.contains("dark");
    }

    false
}

/// Format large numbers in a human-readable way
pub fn format_large_number(value: f64) -> String {
    if value >= 1_000_000_000.0 {
        format!("{:.1}B", value / 1_000_000_000.0)
    } else if value >= 1_000_000.0 {
        format!("{:.1}M", value / 1_000_000.0)
    } else if value >= 1_000.0 {
        format!("{:.1}K", value / 1_000.0)
    } else if value >= 1.0 {
        format!("{:.0}", value)
    } else if value >= 0.001 {
        format!("{:.4}", value)
    } else {
        format!("{:.8}", value)
    }
}

/// Format Bitcoin amounts with appropriate precision
pub fn format_bitcoin_amount(btc: f64) -> String {
    if btc >= 1.0 {
        format!("{:.2} BTC", btc)
    } else if btc >= 0.001 {
        format!("{:.4} BTC", btc)
    } else if btc >= 0.000001 {
        format!("{:.6} BTC", btc)
    } else {
        format!("{:.8} BTC", btc)
    }
}

/// Format percentile values
pub fn format_percentile(percentile: f64) -> String {
    if percentile >= 99.0 {
        format!("{:.2}%", percentile)
    } else if percentile >= 90.0 {
        format!("{:.1}%", percentile)
    } else {
        format!("{:.0}%", percentile)
    }
}
