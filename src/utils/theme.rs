use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::window;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn to_string(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "light" => Some(Theme::Light),
            "dark" => Some(Theme::Dark),
            _ => None,
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}

pub struct ThemeManager;

impl ThemeManager {
    const STORAGE_KEY: &'static str = "bitcoin-wealth-theme";

    /// Get the user's preferred theme from localStorage
    pub fn get_stored_theme() -> Option<Theme> {
        let window = window()?;
        let local_storage = window.local_storage().ok()??;
        let theme_str = local_storage.get_item(Self::STORAGE_KEY).ok()??;
        Theme::from_string(&theme_str)
    }

    /// Save the user's theme preference to localStorage
    pub fn save_theme(theme: Theme) -> Result<(), JsValue> {
        let window = window().ok_or("No window available")?;
        let local_storage = window
            .local_storage()
            .map_err(|_| "Failed to get localStorage")?
            .ok_or("localStorage not available")?;

        local_storage
            .set_item(Self::STORAGE_KEY, theme.to_string())
            .map_err(|_| "Failed to save theme to localStorage")?;

        Ok(())
    }

    /// Detect system theme preference
    pub fn get_system_theme() -> Theme {
        let _window = match window() {
            Some(w) => w,
            None => return Theme::Light,
        };

        // For now, default to light theme
        // TODO: Implement proper media query detection when needed
        Theme::Light
    }

    /// Get the initial theme (stored preference > system preference > light)
    pub fn get_initial_theme() -> Theme {
        Self::get_stored_theme().unwrap_or_else(|| Self::get_system_theme())
    }

    /// Apply theme to document with optimized performance and no hanging
    pub fn apply_theme(theme: Theme) -> Result<(), JsValue> {
        let window = window().ok_or("No window available")?;
        let document = window.document().ok_or("No document available")?;
        let html_element = document
            .document_element()
            .ok_or("No document element available")?;

        // Apply theme using data-theme attribute and class attribute
        match theme {
            Theme::Dark => {
                // Use setAttribute with data-theme for better CSS targeting
                html_element
                    .set_attribute("data-theme", "dark")
                    .map_err(|_| "Failed to set data-theme attribute")?;

                // Also set class to "dark" for Tailwind compatibility
                html_element
                    .set_attribute("class", "dark")
                    .map_err(|_| "Failed to set dark class")?;
            }
            Theme::Light => {
                // Set data-theme to light
                html_element
                    .set_attribute("data-theme", "light")
                    .map_err(|_| "Failed to set data-theme attribute")?;

                // Remove class attribute for light theme
                html_element
                    .remove_attribute("class")
                    .map_err(|_| "Failed to remove class attribute")?;
            }
        }

        web_sys::console::log_1(&format!("✅ Theme applied quickly: {}", theme.to_string()).into());

        Ok(())
    }

    /// Apply theme with limited retry to prevent infinite loops
    pub fn apply_theme_with_retry(theme: Theme) -> Result<(), JsValue> {
        // First attempt - this should usually succeed
        if let Ok(()) = Self::apply_theme(theme) {
            return Ok(());
        }

        web_sys::console::log_1(&"⚠️ Theme application failed, trying once more...".into());

        // Only one retry attempt to prevent hanging
        match Self::apply_theme(theme) {
            Ok(()) => {
                web_sys::console::log_1(&"✅ Theme applied on retry".into());
                Ok(())
            }
            Err(e) => {
                web_sys::console::log_1(
                    &format!("❌ Theme application failed after retry: {:?}", e).into(),
                );
                // Don't propagate error to prevent app hanging
                Ok(())
            }
        }
    }

}
