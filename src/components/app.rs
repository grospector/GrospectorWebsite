use gloo_timers::future::TimeoutFuture;
use stylist::yew::styled_component;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

use crate::components::charts::comparison_chart::ComparisonChart;
use crate::components::charts::distribution_chart::DistributionChart;
use crate::components::charts::statistics_chart::StatisticsChart;
use crate::components::ui::footer::Footer;
use crate::components::ui::header::Header;
use crate::components::ui::loading_spinner::{LoadingSpinner, SpinnerSize};
use crate::services::bitcoin_api::BitcoinApiService;
use crate::services::data_processor::DataProcessor;
use crate::services::percentile_calculator::PercentileCalculator;
use crate::types::bitcoin::{BitcoinDistribution, PercentileResult};
use crate::utils::formatters::{
    format_large_number, format_number_with_commas, format_rank,
};
use crate::utils::theme::{Theme, ThemeManager};

#[derive(Clone, PartialEq)]
pub enum AppState {
    Loading,
    Ready {
        distribution: BitcoinDistribution,
        bitcoin_price: f64,
        network_stats: std::collections::HashMap<String, f64>,
    },
    Error {
        message: String,
    },
}

impl Default for AppState {
    fn default() -> Self {
        AppState::Loading
    }
}

#[derive(Clone, PartialEq)]
pub struct AppData {
    pub state: AppState,
    pub user_input: String,
    pub calculation_result: Option<PercentileResult>,
    pub is_calculating: bool,
    pub wealth_analysis: Option<std::collections::HashMap<String, f64>>,
    pub percentile_thresholds: Option<Vec<(f64, f64)>>,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            state: AppState::default(),
            user_input: String::new(),
            calculation_result: None,
            is_calculating: false,
            wealth_analysis: None,
            percentile_thresholds: None,
        }
    }
}

#[styled_component(App)]
pub fn app() -> Html {
    let app_data = use_state(|| AppData::default());

    // Theme state management
    let current_theme = use_state(|| ThemeManager::get_initial_theme());

    // Initialize services
    let api_service = use_state(|| BitcoinApiService::new());
    let _data_processor = use_state(|| DataProcessor::new());
    let percentile_calculator = use_state(|| PercentileCalculator::new());

    // Initialize theme on mount - only run once
    use_effect_with((), {
        let initial_theme = *current_theme;
        move |_| {
            console::log_1(&format!("🎨 Initializing theme: {:?}", initial_theme).into());

            // Use the improved theme application with retry logic
            if let Err(e) = ThemeManager::apply_theme_with_retry(initial_theme) {
                console::log_1(&format!("❌ Failed to apply theme with retry: {:?}", e).into());
            } else {
                console::log_1(&"✅ Theme initialization started".into());
            }
            || ()
        }
    });

    // Load Bitcoin distribution data on mount
    {
        let app_data_clone = app_data.clone();
        let api_service = api_service.clone();

        use_effect_with((), move |_| {
            let app_data = app_data_clone.clone();
            let api_service = api_service.clone();

            spawn_local(async move {
                console::log_1(&"🚀 Starting Bitcoin distribution data load...".into());

                // Set loading state
                app_data.set(AppData {
                    state: AppState::Loading,
                    ..(*app_data).clone()
                });

                // Add a small delay to show loading state
                TimeoutFuture::new(500).await;

                // Fetch Bitcoin distribution data
                match api_service.fetch_bitcoin_distribution().await {
                    Ok(distribution) => {
                        console::log_1(
                            &format!(
                                "✅ Distribution data loaded: {} ranges",
                                distribution.ranges.len()
                            )
                            .into(),
                        );

                        // Fetch additional data
                        let bitcoin_price =
                            api_service.fetch_bitcoin_price().await.unwrap_or(50000.0);
                        let network_stats =
                            api_service.fetch_network_stats().await.unwrap_or_default();

                        app_data.set(AppData {
                            state: AppState::Ready {
                                distribution,
                                bitcoin_price,
                                network_stats,
                            },
                            ..(*app_data).clone()
                        });
                    }
                    Err(e) => {
                        console::log_1(
                            &format!("❌ Failed to load distribution data: {}", e).into(),
                        );
                        app_data.set(AppData {
                            state: AppState::Error {
                                message: format!("Failed to load Bitcoin distribution data: {}", e),
                            },
                            ..(*app_data).clone()
                        });
                    }
                }
            });

            || ()
        });
    }

    // Handle user input for Bitcoin amount
    let on_input_change = {
        let app_data_clone = app_data.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = input.value();

            app_data_clone.set(AppData {
                user_input: value,
                calculation_result: None, // Clear previous result
                ..(*app_data_clone).clone()
            });
        })
    };

    // Handle theme changes
    let on_theme_change = {
        let current_theme = current_theme.clone();
        Callback::from(move |new_theme: Theme| {
            console::log_1(&format!("🎨 Changing theme to: {:?}", new_theme).into());

            // Update theme state
            current_theme.set(new_theme);

            // Apply theme to document
            if let Err(e) = ThemeManager::apply_theme(new_theme) {
                console::log_1(&format!("❌ Failed to apply theme: {:?}", e).into());
            } else {
                console::log_1(&"✅ Theme applied successfully".into());
            }

            // Save theme preference
            if let Err(e) = ThemeManager::save_theme(new_theme) {
                console::log_1(&format!("❌ Failed to save theme: {:?}", e).into());
            } else {
                console::log_1(&"✅ Theme saved successfully".into());
            }
        })
    };

    // Helper function to create preset button handlers
    let create_preset_handler = {
        let app_data_for_presets = app_data.clone();
        move |_app_data_param: UseStateHandle<AppData>, value: &str| {
            let app_data_clone = app_data_for_presets.clone();
            let value = value.to_string();
            Callback::from(move |_: MouseEvent| {
                app_data_clone.set(AppData {
                    user_input: value.clone(),
                    calculation_result: None, // Clear previous result
                    ..(*app_data_clone).clone()
                });
            })
        }
    };

    // Handle Bitcoin amount calculation
    let on_calculate = {
        let app_data_for_calc = app_data.clone();
        let percentile_calculator = percentile_calculator.clone();

        Callback::from(move |_| {
            let app_data = app_data_for_calc.clone();
            let percentile_calculator = percentile_calculator.clone();
            let user_input = app_data.user_input.clone();

            if let AppState::Ready { distribution, .. } = &app_data.state {
                let distribution = distribution.clone();

                spawn_local(async move {
                    // Set calculating state
                    app_data.set(AppData {
                        is_calculating: true,
                        ..(*app_data).clone()
                    });

                    // Parse user input
                    match user_input.parse::<f64>() {
                        Ok(amount) => {
                            console::log_1(
                                &format!("🧮 Calculating percentile for {} BTC", amount).into(),
                            );

                            // Add small delay for UX
                            TimeoutFuture::new(300).await;

                            // Calculate percentile and additional analysis
                            match percentile_calculator
                                .calculate_user_percentile(amount, &distribution)
                            {
                                Ok(result) => {
                                    console::log_1(
                                        &format!(
                                            "✅ Calculation complete: {:.2}%",
                                            result.percentile
                                        )
                                        .into(),
                                    );

                                    // Calculate wealth concentration analysis
                                    let wealth_analysis = percentile_calculator
                                        .calculate_wealth_concentration(&distribution)
                                        .unwrap_or_default();

                                    // Calculate percentile thresholds
                                    let percentile_thresholds = percentile_calculator
                                        .calculate_percentile_thresholds(&distribution)
                                        .unwrap_or_default();

                                    app_data.set(AppData {
                                        calculation_result: Some(result),
                                        wealth_analysis: Some(wealth_analysis),
                                        percentile_thresholds: Some(percentile_thresholds),
                                        is_calculating: false,
                                        ..(*app_data).clone()
                                    });
                                }
                                Err(e) => {
                                    console::log_1(&format!("❌ Calculation failed: {}", e).into());
                                    app_data.set(AppData {
                                        is_calculating: false,
                                        ..(*app_data).clone()
                                    });
                                }
                            }
                        }
                        Err(_) => {
                            console::log_1(&"❌ Invalid Bitcoin amount entered".into());
                            app_data.set(AppData {
                                is_calculating: false,
                                ..(*app_data).clone()
                            });
                        }
                    }
                });
            }
        })
    };

    let render_content = match &app_data.state {
        AppState::Loading => {
            html! {
                <div class="flex flex-col items-center justify-center min-h-96">
                    <LoadingSpinner size={SpinnerSize::Large} message={Some("Loading Bitcoin distribution data...".to_string())} />
                </div>
            }
        }
        AppState::Ready {
            distribution,
            bitcoin_price,
            network_stats: _,
        } => {
            html! {
                <div class="space-y-8">
                    // Welcome Section
                    <div class="bg-gradient-to-r from-blue-600 to-purple-600 rounded-lg p-8 text-white">
                        <h2 class="text-3xl font-bold mb-4">{"Welcome to Bitcoin Wealth Comparison"}</h2>
                        <p class="text-xl mb-6">
                            {"Discover where your Bitcoin holdings rank among global addresses. All calculations are performed locally in your browser."}
                        </p>
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mt-6">
                                                         <div class="bg-white/20 rounded-lg p-4">
                                 <div class="text-2xl font-bold">{format_large_number(distribution.total_addresses as f64)}</div>
                                 <div class="text-sm opacity-90">{"Total Addresses"}</div>
                             </div>
                             <div class="bg-white/20 rounded-lg p-4">
                                 <div class="text-2xl font-bold">{format!("{:.1}M", distribution.total_supply / 1_000_000.0)}</div>
                                 <div class="text-sm opacity-90">{"Total Supply (BTC)"}</div>
                             </div>
                             <div class="bg-white/20 rounded-lg p-4">
                                 <div class="text-2xl font-bold">{format!("${:.0}", bitcoin_price)}</div>
                                 <div class="text-sm opacity-90">{"Bitcoin Price"}</div>
                             </div>
                        </div>
                    </div>

                    // Enhanced Input Section with Smart Suggestions
                    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 hover:shadow-xl transition-shadow duration-300">
                        <div class="mb-6">
                            <h3 class="text-2xl font-bold mb-2 text-gray-900 dark:text-white">{"Find Your Bitcoin Rank"}</h3>
                            <p class="text-gray-600 dark:text-gray-300">{"Enter your Bitcoin amount to see how you compare globally"}</p>
                        </div>
                        
                        // Quick Amount Suggestions
                        <div class="mb-4">
                            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">{"Quick Select:"}</label>
                            <div class="flex flex-wrap gap-2">
                                <button
                                    onclick={create_preset_handler(app_data.clone(), "0.001")}
                                    class="px-3 py-1 text-sm bg-gray-100 dark:bg-gray-700 hover:bg-orange-100 dark:hover:bg-orange-900 text-gray-700 dark:text-gray-300 rounded-full transition-colors duration-200 hover:text-orange-600 dark:hover:text-orange-400"
                                >
                                    {"0.001 BTC"}
                                </button>
                                <button
                                    onclick={create_preset_handler(app_data.clone(), "0.01")}
                                    class="px-3 py-1 text-sm bg-gray-100 dark:bg-gray-700 hover:bg-orange-100 dark:hover:bg-orange-900 text-gray-700 dark:text-gray-300 rounded-full transition-colors duration-200 hover:text-orange-600 dark:hover:text-orange-400"
                                >
                                    {"0.01 BTC"}
                                </button>
                                <button
                                    onclick={create_preset_handler(app_data.clone(), "0.1")}
                                    class="px-3 py-1 text-sm bg-gray-100 dark:bg-gray-700 hover:bg-orange-100 dark:hover:bg-orange-900 text-gray-700 dark:text-gray-300 rounded-full transition-colors duration-200 hover:text-orange-600 dark:hover:text-orange-400"
                                >
                                    {"0.1 BTC"}
                                </button>
                                <button
                                    onclick={create_preset_handler(app_data.clone(), "1")}
                                    class="px-3 py-1 text-sm bg-gray-100 dark:bg-gray-700 hover:bg-orange-100 dark:hover:bg-orange-900 text-gray-700 dark:text-gray-300 rounded-full transition-colors duration-200 hover:text-orange-600 dark:hover:text-orange-400"
                                >
                                    {"1 BTC"}
                                </button>
                                <button
                                    onclick={create_preset_handler(app_data.clone(), "10")}
                                    class="px-3 py-1 text-sm bg-gray-100 dark:bg-gray-700 hover:bg-orange-100 dark:hover:bg-orange-900 text-gray-700 dark:text-gray-300 rounded-full transition-colors duration-200 hover:text-orange-600 dark:hover:text-orange-400"
                                >
                                    {"10 BTC"}
                                </button>
                                <button
                                    onclick={create_preset_handler(app_data.clone(), "100")}
                                    class="px-3 py-1 text-sm bg-gray-100 dark:bg-gray-700 hover:bg-orange-100 dark:hover:bg-orange-900 text-gray-700 dark:text-gray-300 rounded-full transition-colors duration-200 hover:text-orange-600 dark:hover:text-orange-400"
                                >
                                    {"100 BTC"}
                                </button>
                            </div>
                        </div>

                        // Main Input with Enhanced UX
                        <div class="space-y-4">
                            <div>
                                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">{"Your Bitcoin Amount"}</label>
                                <div class="relative">
                                    <input
                                        type="number"
                                        step="0.00000001"
                                        min="0"
                                        max="21000000"
                                        placeholder="0.00000000"
                                        value={app_data.user_input.clone()}
                                        oninput={on_input_change}
                                        onkeypress={{
                                            let on_calculate_clone = on_calculate.clone();
                                            let app_data_clone = app_data.clone();
                                            Callback::from(move |e: KeyboardEvent| {
                                                if e.key() == "Enter" && !app_data_clone.user_input.is_empty() && !app_data_clone.is_calculating {
                                                    e.prevent_default();
                                                    on_calculate_clone.emit(MouseEvent::new("click").unwrap());
                                                }
                                            })
                                        }}
                                        class="w-full p-4 pr-16 border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-transparent font-mono text-lg transition-all duration-200 hover:border-orange-300 dark:hover:border-orange-600"
                                    />
                                    <div class="absolute inset-y-0 right-0 flex items-center pr-4 pointer-events-none">
                                        <span class="text-gray-500 dark:text-gray-400 font-semibold">{"BTC"}</span>
                                    </div>
                                </div>
                                if !app_data.user_input.is_empty() {
                                    if let Ok(amount) = app_data.user_input.parse::<f64>() {
                                        <div class="mt-2 text-sm text-gray-600 dark:text-gray-400">
                                            {format!("≈ {} satoshis", format_number_with_commas(amount * 100_000_000.0))}
                                        </div>
                                    }
                                }
                            </div>
                            
                            <button
                                onclick={on_calculate}
                                disabled={app_data.user_input.is_empty() || app_data.is_calculating}
                                class="w-full px-6 py-4 bg-gradient-to-r from-orange-500 to-orange-600 hover:from-orange-600 hover:to-orange-700 disabled:opacity-50 disabled:cursor-not-allowed text-white rounded-lg font-semibold text-lg transition-all duration-300 transform hover:scale-105 hover:shadow-lg disabled:hover:scale-100 disabled:hover:shadow-none"
                            >
                                if app_data.is_calculating {
                                    <div class="flex items-center justify-center">
                                        <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                        </svg>
                                        {"Calculating Your Rank..."}
                                    </div>
                                } else {
                                    {"🚀 Calculate My Bitcoin Rank"}
                                }
                            </button>
                        </div>
                    </div>

                    // Enhanced Results Section with Better UX
                    if let Some(result) = &app_data.calculation_result {
                        <div class="bg-gradient-to-br from-white to-gray-50 dark:from-gray-800 dark:to-gray-900 rounded-xl shadow-xl p-8 border border-gray-200 dark:border-gray-700 animate-slide-in">
                            // Results Header with Celebration
                            <div class="text-center mb-8">
                                <div class="text-4xl mb-2">{"🎉"}</div>
                                <h3 class="text-3xl font-bold mb-2 text-gray-900 dark:text-white">{"Your Bitcoin Rank"}</h3>
                                <p class="text-lg text-gray-600 dark:text-gray-300">{"Here's how you compare to Bitcoin holders worldwide"}</p>
                            </div>
                            
                            // Main Stats Grid with Enhanced Design
                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                                <div class="bg-gradient-to-br from-blue-500 to-blue-600 rounded-xl p-6 text-white transform hover:scale-105 transition-transform duration-300 shadow-lg">
                                    <div class="flex items-center justify-between mb-2">
                                        <div class="text-3xl">{"📊"}</div>
                                        <div class="text-xs opacity-75 bg-white/20 px-2 py-1 rounded-full">{"PERCENTILE"}</div>
                                    </div>
                                    <div class="text-3xl font-bold mb-1">{format!("{:.2}%", result.percentile)}</div>
                                    <div class="text-sm opacity-90">{"Global Percentile"}</div>
                                </div>
                                
                                <div class="bg-gradient-to-br from-green-500 to-green-600 rounded-xl p-6 text-white transform hover:scale-105 transition-transform duration-300 shadow-lg">
                                    <div class="flex items-center justify-between mb-2">
                                        <div class="text-3xl">{"🏆"}</div>
                                        <div class="text-xs opacity-75 bg-white/20 px-2 py-1 rounded-full">{"RANK"}</div>
                                    </div>
                                    <div class="text-3xl font-bold mb-1">{format_rank(result.rank)}</div>
                                    <div class="text-sm opacity-90">{"Global Ranking"}</div>
                                </div>
                                
                                <div class="bg-gradient-to-br from-purple-500 to-purple-600 rounded-xl p-6 text-white transform hover:scale-105 transition-transform duration-300 shadow-lg">
                                    <div class="flex items-center justify-between mb-2">
                                        <div class="text-3xl">{result.wealth_category.emoji()}</div>
                                        <div class="text-xs opacity-75 bg-white/20 px-2 py-1 rounded-full">{"CATEGORY"}</div>
                                    </div>
                                    <div class="text-2xl font-bold mb-1">{result.wealth_category.as_str()}</div>
                                    <div class="text-sm opacity-90">{"Wealth Tier"}</div>
                                </div>
                                
                                <div class="bg-gradient-to-br from-orange-500 to-orange-600 rounded-xl p-6 text-white transform hover:scale-105 transition-transform duration-300 shadow-lg">
                                    <div class="flex items-center justify-between mb-2">
                                        <div class="text-3xl">{"₿"}</div>
                                        <div class="text-xs opacity-75 bg-white/20 px-2 py-1 rounded-full">{"HOLDINGS"}</div>
                                    </div>
                                    <div class="text-2xl font-bold mb-1">{format!("{:.4}", result.user_bitcoin_amount)}</div>
                                    <div class="text-sm opacity-90">{"Bitcoin Amount"}</div>
                                </div>
                            </div>

                            // Detailed Insights Section
                            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                // Comparison Insights
                                <div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700">
                                    <div class="flex items-center mb-4">
                                        <div class="text-2xl mr-3">{"📈"}</div>
                                        <h4 class="text-lg font-semibold text-gray-900 dark:text-white">{"Comparison Insights"}</h4>
                                    </div>
                                    <div class="space-y-3">
                                        <div class="flex items-center justify-between p-3 bg-green-50 dark:bg-green-900/20 rounded-lg">
                                            <span class="text-sm text-gray-700 dark:text-gray-300">{"Addresses below you"}</span>
                                            <span class="font-semibold text-green-600 dark:text-green-400">{format_large_number(result.addresses_below as f64)}</span>
                                        </div>
                                        <div class="flex items-center justify-between p-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
                                            <span class="text-sm text-gray-700 dark:text-gray-300">{"Addresses above you"}</span>
                                            <span class="font-semibold text-blue-600 dark:text-blue-400">{format_large_number(result.addresses_above as f64)}</span>
                                        </div>
                                        if let Some(supply_share) = result.comparison_metrics.get("supply_share_percent") {
                                            <div class="flex items-center justify-between p-3 bg-orange-50 dark:bg-orange-900/20 rounded-lg">
                                                <span class="text-sm text-gray-700 dark:text-gray-300">{"% of total supply"}</span>
                                                <span class="font-semibold text-orange-600 dark:text-orange-400">{format!("{:.6}%", supply_share)}</span>
                                            </div>
                                        }
                                    </div>
                                </div>

                                // Achievement Section
                                <div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700">
                                    <div class="flex items-center mb-4">
                                        <div class="text-2xl mr-3">{"🎯"}</div>
                                        <h4 class="text-lg font-semibold text-gray-900 dark:text-white">{"Your Achievement"}</h4>
                                    </div>
                                    <div class="text-center">
                                        <div class="text-6xl mb-3">
                                            {result.wealth_category.emoji()}
                                        </div>
                                        <div class="text-lg font-semibold text-gray-900 dark:text-white mb-2">
                                            {format!("Bitcoin {}!", result.wealth_category.as_str())}
                                        </div>
                                        <div class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                                            {result.wealth_category.btc_range()}
                                        </div>
                                        <p class="text-sm text-gray-600 dark:text-gray-400">
                                            {result.wealth_category.description()}
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Wealth Analysis Section (if available)
                        if let Some(wealth_analysis) = &app_data.wealth_analysis {
                            <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6 border border-gray-200 dark:border-gray-700">
                                <div class="flex items-center mb-6">
                                    <div class="text-2xl mr-3">{"📊"}</div>
                                    <h3 class="text-xl font-bold text-gray-900 dark:text-white">{"Wealth Concentration Analysis"}</h3>
                                </div>
                                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                                    if let Some(gini) = wealth_analysis.get("gini_coefficient") {
                                        <div class="bg-gradient-to-br from-indigo-50 to-indigo-100 dark:from-indigo-900/20 dark:to-indigo-800/20 rounded-lg p-4 border border-indigo-200 dark:border-indigo-700">
                                            <div class="text-sm font-medium text-indigo-600 dark:text-indigo-400 mb-1">{"Gini Coefficient"}</div>
                                            <div class="text-2xl font-bold text-indigo-900 dark:text-indigo-100">{format!("{:.3}", gini)}</div>
                                            <div class="text-xs text-indigo-600 dark:text-indigo-400">{"Wealth inequality (0=equal, 1=unequal)"}</div>
                                        </div>
                                    }
                                    if let Some(top_1) = wealth_analysis.get("top_1percent_wealth") {
                                        <div class="bg-gradient-to-br from-red-50 to-red-100 dark:from-red-900/20 dark:to-red-800/20 rounded-lg p-4 border border-red-200 dark:border-red-700">
                                            <div class="text-sm font-medium text-red-600 dark:text-red-400 mb-1">{"Top 1% Wealth Share"}</div>
                                            <div class="text-2xl font-bold text-red-900 dark:text-red-100">{format!("{:.1}%", top_1)}</div>
                                            <div class="text-xs text-red-600 dark:text-red-400">{"Held by top 1% of addresses"}</div>
                                        </div>
                                    }
                                    if let Some(top_10) = wealth_analysis.get("top_10percent_wealth") {
                                        <div class="bg-gradient-to-br from-amber-50 to-amber-100 dark:from-amber-900/20 dark:to-amber-800/20 rounded-lg p-4 border border-amber-200 dark:border-amber-700">
                                            <div class="text-sm font-medium text-amber-600 dark:text-amber-400 mb-1">{"Top 10% Wealth Share"}</div>
                                            <div class="text-2xl font-bold text-amber-900 dark:text-amber-100">{format!("{:.1}%", top_10)}</div>
                                            <div class="text-xs text-amber-600 dark:text-amber-400">{"Held by top 10% of addresses"}</div>
                                        </div>
                                    }
                                    if let Some(hhi) = wealth_analysis.get("hhi_index") {
                                        <div class="bg-gradient-to-br from-emerald-50 to-emerald-100 dark:from-emerald-900/20 dark:to-emerald-800/20 rounded-lg p-4 border border-emerald-200 dark:border-emerald-700">
                                            <div class="text-sm font-medium text-emerald-600 dark:text-emerald-400 mb-1">{"HHI Index"}</div>
                                            <div class="text-2xl font-bold text-emerald-900 dark:text-emerald-100">{format!("{:.0}", hhi)}</div>
                                            <div class="text-xs text-emerald-600 dark:text-emerald-400">{"Market concentration index"}</div>
                                        </div>
                                    }
                                </div>
                            </div>
                        }

                        // Percentile Thresholds Section (if available)
                        if let Some(thresholds) = &app_data.percentile_thresholds {
                            <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6 border border-gray-200 dark:border-gray-700">
                                <div class="flex items-center mb-6">
                                    <div class="text-2xl mr-3">{"🎯"}</div>
                                    <h3 class="text-xl font-bold text-gray-900 dark:text-white">{"Bitcoin Wealth Percentile Thresholds"}</h3>
                                </div>
                                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                                    {for thresholds.iter().map(|(percentile, amount)| {
                                        // Get the appropriate wealth category based on the amount threshold
                                        use crate::types::bitcoin::WealthCategory;
                                        let wealth_category = WealthCategory::from_btc_amount(*amount);
                                        
                                        // Enhanced high-contrast color system for dark mode visibility with systematic emoji
                                        let (bg_color, text_primary, text_secondary, border_color) = match *percentile {
                                            p if p >= 99.0 => (
                                                "from-orange-50 to-orange-100 dark:from-gray-800 dark:to-gray-700",
                                                "text-orange-900 dark:text-orange-300",
                                                "text-orange-700 dark:text-orange-400", 
                                                "border-orange-200 dark:border-orange-600"
                                            ),
                                            p if p >= 95.0 => (
                                                "from-orange-50 to-yellow-50 dark:from-gray-800 dark:to-gray-700",
                                                "text-orange-900 dark:text-yellow-300",
                                                "text-orange-700 dark:text-yellow-400",
                                                "border-orange-200 dark:border-yellow-600"
                                            ),
                                            p if p >= 90.0 => (
                                                "from-yellow-50 to-amber-50 dark:from-gray-800 dark:to-gray-700",
                                                "text-amber-900 dark:text-amber-300", 
                                                "text-amber-700 dark:text-amber-400",
                                                "border-amber-200 dark:border-amber-600"
                                            ),
                                            p if p >= 75.0 => (
                                                "from-green-50 to-emerald-50 dark:from-gray-800 dark:to-gray-700",
                                                "text-emerald-900 dark:text-emerald-300",
                                                "text-emerald-700 dark:text-emerald-400", 
                                                "border-emerald-200 dark:border-emerald-600"
                                            ),
                                            p if p >= 50.0 => (
                                                "from-blue-50 to-cyan-50 dark:from-gray-800 dark:to-gray-700",
                                                "text-cyan-900 dark:text-cyan-300",
                                                "text-cyan-700 dark:text-cyan-400",
                                                "border-cyan-200 dark:border-cyan-600"
                                            ),
                                            _ => (
                                                "from-slate-50 to-gray-50 dark:from-gray-800 dark:to-gray-700",
                                                "text-slate-900 dark:text-slate-200",
                                                "text-slate-700 dark:text-slate-400", 
                                                "border-slate-200 dark:border-slate-600"
                                            ),
                                        };

                                        let amount_text = if *amount >= 1.0 {
                                            format!("{:.2} BTC", amount)
                                        } else if *amount >= 0.001 {
                                            format!("{:.4} BTC", amount)
                                        } else {
                                            format!("{:.8} BTC", amount)
                                        };

                                        html! {
                                            <div class={format!("bg-gradient-to-br {} rounded-xl p-5 border-2 {} transform hover:scale-105 transition-all duration-300 shadow-lg hover:shadow-xl", bg_color, border_color)}>
                                                <div class="flex items-center justify-between mb-3">
                                                    <div class="text-2xl">{wealth_category.emoji()}</div>
                                                    <div class={format!("text-xs font-semibold {} px-3 py-1 rounded-full bg-white/20 dark:bg-black/20", text_secondary)}>
                                                        {format!("{:.1}%", percentile)}
                                                    </div>
                                                </div>
                                                <div class={format!("text-2xl font-black {} mb-2 tracking-tight", text_primary)}>
                                                    {amount_text}
                                                </div>
                                                <div class={format!("text-sm font-medium {} mb-1", text_secondary)}>
                                                    {format!("{:.1}th Percentile", percentile)}
                                                </div>
                                                <div class={format!("text-xs {} opacity-90", text_secondary)}>
                                                    {"Minimum threshold"}
                                                </div>
                                            </div>
                                        }
                                    })}
                                </div>
                                <div class="mt-4 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
                                    <p class="text-sm text-gray-600 dark:text-gray-300">
                                        {"These thresholds show the minimum Bitcoin amount needed to reach each percentile rank among all Bitcoin addresses globally."}
                                    </p>
                                </div>
                            </div>
                        }

                        // Chart visualizations for user results
                        <ComparisonChart
                            user_result={result.clone()}
                            distribution={distribution.clone()}
                        />
                    }

                    // Main Distribution Chart (always visible when data is loaded)
                    <DistributionChart
                        distribution={distribution.clone()}
                        user_amount={app_data.calculation_result.as_ref().map(|r| r.user_bitcoin_amount)}
                        user_percentile={app_data.calculation_result.as_ref().map(|r| r.percentile)}
                    />

                    // Network Statistics Section - Temporarily commented out due to syntax issues
                    // TODO: Fix the HTML macro syntax for the statistics section
                    /*
                    {
                        let stats = distribution.calculate_statistics();
                        html! {
                            <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6 border border-gray-200 dark:border-gray-700">
                                <div class="flex items-center mb-6">
                                    <div class="text-2xl mr-3">{"📈"}</div>
                                    <h3 class="text-xl font-bold text-gray-900 dark:text-white">{"Network Statistics"}</h3>
                                </div>
                                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                                    // Statistics cards would go here
                                </div>
                            </div>
                        }
                    }
                    */

                    // Statistics Chart
                    <StatisticsChart
                        distribution={distribution.clone()}
                        bitcoin_price={*bitcoin_price}
                    />

                    // Distribution Overview
                    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
                        <h3 class="text-xl font-bold mb-4 text-gray-900 dark:text-white">{"Bitcoin Distribution Overview"}</h3>
                        <div class="space-y-3">
                            {for distribution.ranges.iter().take(8).map(|range| {
                                let max_display = if range.max_btc == f64::INFINITY {
                                    "∞".to_string()
                                } else {
                                    format!("{:.3}", range.max_btc)
                                };

                                html! {
                                    <div class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
                                        <div class="flex-1">
                                            <div class="font-semibold text-gray-900 dark:text-white">
                                                {format!("{:.3} - {} BTC", range.min_btc, max_display)}
                                            </div>
                                            <div class="text-sm text-gray-600 dark:text-gray-300">
                                                {format!("{} addresses ({:.1}%)", format_large_number(range.address_count as f64), range.percentage_of_addresses)}
                                            </div>
                                        </div>
                                        <div class="text-right">
                                            <div class="font-semibold text-gray-900 dark:text-white">
                                                {format!("{:.1}%", range.percentage_of_supply)}
                                            </div>
                                            <div class="text-sm text-gray-600 dark:text-gray-300">{"of supply"}</div>
                                        </div>
                                    </div>
                                }
                            })}
                        </div>
                    </div>
                </div>
            }
        }
        AppState::Error { message } => {
            html! {
                <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-6">
                    <div class="flex items-center">
                        <div class="mr-3 text-red-600 dark:text-red-400">
                            <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"></path>
                            </svg>
                        </div>
                        <div>
                            <h3 class="text-lg font-semibold text-red-800 dark:text-red-200">{"Error Loading Data"}</h3>
                            <p class="mt-1 text-red-700 dark:text-red-300">{message}</p>
                        </div>
                    </div>
                </div>
            }
        }
    };

    html! {
        <div class="min-h-screen bg-gray-50 dark:bg-gray-900 flex flex-col">
            <Header
                current_theme={*current_theme}
                on_theme_change={on_theme_change}
            />
            <main class="flex-1 container mx-auto px-4 py-8">
                <div class="max-w-6xl mx-auto">
                    {render_content}
                </div>
            </main>
            <Footer />
        </div>
    }
}
