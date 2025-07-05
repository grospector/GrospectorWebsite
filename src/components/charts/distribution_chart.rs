use crate::types::bitcoin::{BitcoinDistribution, WealthRange};
use crate::utils::chart_theme::{format_bitcoin_amount, format_large_number, MempoolChartTheme};
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DistributionChartProps {
    pub distribution: BitcoinDistribution,
    pub user_amount: Option<f64>,
    pub user_percentile: Option<f64>,
}

#[function_component(DistributionChart)]
pub fn distribution_chart(props: &DistributionChartProps) -> Html {
    let canvas_ref = use_node_ref();

    // Effect to draw the chart when data changes
    {
        let canvas_ref = canvas_ref.clone();
        let distribution = props.distribution.clone();
        let user_amount = props.user_amount;
        let user_percentile = props.user_percentile;

        use_effect_with(
            (distribution.clone(), user_amount, user_percentile),
            move |_| {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    let _ = draw_distribution_chart(
                        canvas,
                        &distribution,
                        user_amount,
                        user_percentile,
                    );
                }
                || ()
            },
        );
    }

    html! {
        <div class="bg-gradient-to-br from-white to-gray-50 dark:from-gray-800 dark:to-gray-900 rounded-xl shadow-xl p-8 border border-gray-200 dark:border-gray-700 transform hover:shadow-2xl transition-all duration-300">
            // Enhanced Header Section
            <div class="mb-8">
                <div class="flex items-center mb-4">
                    <div class="text-3xl mr-3">{"📊"}</div>
                    <div>
                        <h3 class="text-3xl font-bold text-gray-900 dark:text-white">{"Bitcoin Distribution Chart"}</h3>
                        <p class="text-lg text-gray-600 dark:text-gray-300">{"Visualizing global Bitcoin wealth distribution"}</p>
                    </div>
                </div>
                
                // Key Statistics Cards
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
                    <div class="bg-gradient-to-br from-blue-500 to-blue-600 rounded-xl p-4 text-white transform hover:scale-105 transition-transform duration-300 shadow-lg">
                        <div class="flex items-center justify-between mb-2">
                            <div class="text-2xl">{"🏦"}</div>
                            <div class="text-xs opacity-75 bg-white/20 px-2 py-1 rounded-full">{"ADDRESSES"}</div>
                        </div>
                        <div class="text-2xl font-bold mb-1">{format!("{}M", (props.distribution.total_addresses as f64 / 1_000_000.0).round())}</div>
                        <div class="text-sm opacity-90">{"Total Bitcoin Addresses"}</div>
                    </div>
                    
                    <div class="bg-gradient-to-br from-orange-500 to-orange-600 rounded-xl p-4 text-white transform hover:scale-105 transition-transform duration-300 shadow-lg">
                        <div class="flex items-center justify-between mb-2">
                            <div class="text-2xl">{"₿"}</div>
                            <div class="text-xs opacity-75 bg-white/20 px-2 py-1 rounded-full">{"SUPPLY"}</div>
                        </div>
                        <div class="text-2xl font-bold mb-1">{format!("{:.1}M", props.distribution.total_supply / 1_000_000.0)}</div>
                        <div class="text-sm opacity-90">{"Bitcoin in Circulation"}</div>
                    </div>
                    
                    <div class="bg-gradient-to-br from-purple-500 to-purple-600 rounded-xl p-4 text-white transform hover:scale-105 transition-transform duration-300 shadow-lg">
                        <div class="flex items-center justify-between mb-2">
                            <div class="text-2xl">{"📈"}</div>
                            <div class="text-xs opacity-75 bg-white/20 px-2 py-1 rounded-full">{"CONCENTRATION"}</div>
                        </div>
                        <div class="text-2xl font-bold mb-1">{"High"}</div>
                        <div class="text-sm opacity-90">{"Wealth Distribution"}</div>
                    </div>
                </div>
            </div>

            // Chart Container with Enhanced Styling
            <div class="relative bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700 shadow-lg">
                <canvas
                    ref={canvas_ref}
                    width="800"
                    height="400"
                    class="w-full h-auto rounded-lg"
                    style="max-width: 100%; height: auto;"
                />
                
                // Enhanced Legend with Modern Design
                <div class="mt-6 bg-gray-50 dark:bg-gray-900 rounded-xl p-4">
                    <h4 class="text-lg font-semibold mb-3 text-gray-900 dark:text-white flex items-center">
                        <div class="text-xl mr-2">{"🎨"}</div>
                        {"Chart Legend"}
                    </h4>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <div class="flex items-center p-3 bg-white dark:bg-gray-800 rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200">
                            <div class="w-6 h-6 rounded-lg mr-3" style="background: linear-gradient(135deg, #F7931A, #FF8C42);"></div>
                            <div>
                                <div class="font-semibold text-gray-900 dark:text-white">{"Address Distribution"}</div>
                                <div class="text-sm text-gray-600 dark:text-gray-300">{"Bitcoin holdings by range"}</div>
                            </div>
                        </div>
                        
                        if props.user_amount.is_some() {
                            <div class="flex items-center p-3 bg-white dark:bg-gray-800 rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200">
                                <div class="w-6 h-6 rounded-lg mr-3 relative" style="background: linear-gradient(135deg, #F7931A, #FF8C42);">
                                    <div class="absolute inset-0 border-2 border-white rounded-lg"></div>
                                </div>
                                <div>
                                    <div class="font-semibold text-gray-900 dark:text-white">{"Your Position"}</div>
                                    <div class="text-sm text-gray-600 dark:text-gray-300">{"Your ranking on the chart"}</div>
                                </div>
                            </div>
                        }
                        
                        <div class="flex items-center p-3 bg-white dark:bg-gray-800 rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200">
                            <div class="w-6 h-6 rounded-lg mr-3" style="background: linear-gradient(135deg, #6B7280, #9CA3AF);"></div>
                            <div>
                                <div class="font-semibold text-gray-900 dark:text-white">{"Key Percentiles"}</div>
                                <div class="text-sm text-gray-600 dark:text-gray-300">{"Important wealth thresholds"}</div>
                            </div>
                        </div>
                    </div>
                </div>
                
                // Insights Section
                if props.user_amount.is_some() && props.user_percentile.is_some() {
                    <div class="mt-6 bg-gradient-to-r from-green-50 to-emerald-50 dark:from-green-900/20 dark:to-emerald-900/20 rounded-xl p-4 border border-green-200 dark:border-green-800">
                        <div class="flex items-center mb-2">
                            <div class="text-2xl mr-2">{"💡"}</div>
                            <h4 class="text-lg font-semibold text-gray-900 dark:text-white">{"Chart Insights"}</h4>
                        </div>
                        <p class="text-gray-700 dark:text-gray-300">
                            {"Your position is marked on the chart above. The distribution shows how Bitcoin wealth is concentrated among different address ranges, with most addresses holding smaller amounts while fewer addresses hold larger amounts."}
                        </p>
                    </div>
                }
            </div>
        </div>
    }
}

fn draw_distribution_chart(
    canvas: HtmlCanvasElement,
    distribution: &BitcoinDistribution,
    user_amount: Option<f64>,
    user_percentile: Option<f64>,
) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
    let root = backend.into_drawing_area();

    // Get mempool.space inspired theme
    let theme = MempoolChartTheme::new();
    root.fill(&theme.background)?;

    // Filter out ranges with very few addresses for better visualization
    let mut filtered_ranges: Vec<&WealthRange> = distribution
        .ranges
        .iter()
        .filter(|range| range.address_count > 100 && range.max_btc < 1000000.0) // Filter extreme outliers
        .collect();

    // Sort by min_btc for proper ordering
    filtered_ranges.sort_by(|a, b| a.min_btc.partial_cmp(&b.min_btc).unwrap());

    if filtered_ranges.is_empty() {
        return Ok(());
    }

    // Calculate chart bounds
    let min_btc = filtered_ranges.first().unwrap().min_btc.max(0.0001);
    let max_btc = filtered_ranges.last().unwrap().max_btc.min(10000.0);
    let max_addresses = filtered_ranges
        .iter()
        .map(|r| r.address_count)
        .max()
        .unwrap_or(1);

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Bitcoin Distribution by Address Holdings",
            theme.create_text_style(20),
        )
        .margin(15)
        .x_label_area_size(60)
        .y_label_area_size(80)
        .build_cartesian_2d((min_btc..max_btc).log_scale(), 0u64..max_addresses)?;

    chart
        .configure_mesh()
        .x_desc("Bitcoin Amount (BTC)")
        .y_desc("Number of Addresses")
        .x_label_formatter(&|x| format_bitcoin_amount(*x).replace(" BTC", ""))
        .y_label_formatter(&|y| format_large_number(*y as f64))
        .label_style(theme.create_secondary_text_style(12))
        .axis_style(&theme.grid_color)
        .draw()?;

    // Create gradient effect for distribution bars
    let gradient_colors = theme.get_gradient_colors();

    // Draw distribution bars with Bitcoin orange gradient
    chart
        .draw_series(filtered_ranges.iter().enumerate().map(|(i, range)| {
            let x_start = range.min_btc.max(min_btc);
            let x_end = range.max_btc.min(max_btc);
            let height = range.address_count;

            // Create gradient effect based on position
            let color_index = (i * gradient_colors.len()) / filtered_ranges.len().max(1);
            let color = gradient_colors
                .get(color_index)
                .unwrap_or(&theme.bitcoin_orange);

            Rectangle::new([(x_start, 0), (x_end, height)], color.mix(0.8).filled())
        }))?
        .label("Address Distribution")
        .legend(|(x, y)| Rectangle::new([(x, y), (x + 12, y + 8)], theme.bitcoin_orange.filled()));

    // Draw user position if provided with prominent Bitcoin orange
    if let Some(amount) = user_amount {
        if amount >= min_btc && amount <= max_btc {
            // Draw user position line with Bitcoin orange
            chart
                .draw_series(std::iter::once(PathElement::new(
                    vec![(amount, 0), (amount, max_addresses)],
                    theme.bitcoin_orange.stroke_width(4),
                )))?
                .label("Your Position")
                .legend(|(x, y)| {
                    PathElement::new(
                        vec![(x, y), (x + 15, y)],
                        theme.bitcoin_orange.stroke_width(3),
                    )
                });

            // Add user percentile label with better positioning
            if let Some(percentile) = user_percentile {
                let label_text = format!("You: {:.2}%", percentile);
                let label_y = max_addresses as u64 * 85 / 100; // Position at 85% height

                chart.draw_series(std::iter::once(Text::new(
                    label_text,
                    (amount * 1.05, label_y),
                    theme.create_text_style(14),
                )))?;
            }
        }
    }

    // Draw key percentile lines with subtle styling
    let percentiles = vec![50.0, 75.0, 90.0, 95.0, 99.0];
    for percentile in percentiles {
        if let Some(btc_amount) = calculate_percentile_amount(distribution, percentile) {
            if btc_amount >= min_btc && btc_amount <= max_btc {
                // Draw subtle percentile line
                chart.draw_series(std::iter::once(PathElement::new(
                    vec![(btc_amount, 0), (btc_amount, max_addresses / 3)],
                    theme.border_secondary.stroke_width(1),
                )))?;

                // Add percentile label
                chart.draw_series(std::iter::once(Text::new(
                    format!("{}%", percentile),
                    (btc_amount * 1.02, max_addresses / 3 + max_addresses / 15),
                    theme.create_muted_text_style(10),
                )))?;
            }
        }
    }

    // Configure legend with better styling
    chart
        .configure_series_labels()
        .background_style(&theme.card_background.mix(0.9))
        .border_style(&theme.border_primary)
        .label_font(theme.create_secondary_text_style(12))
        .draw()?;

    root.present()?;
    Ok(())
}

fn calculate_percentile_amount(
    distribution: &BitcoinDistribution,
    target_percentile: f64,
) -> Option<f64> {
    let mut cumulative_addresses = 0u64;
    let target_count = (distribution.total_addresses as f64 * target_percentile / 100.0) as u64;

    for range in &distribution.ranges {
        cumulative_addresses += range.address_count;
        if cumulative_addresses >= target_count {
            // Linear interpolation within the range
            let range_position = (target_count - (cumulative_addresses - range.address_count))
                as f64
                / range.address_count as f64;
            return Some(range.min_btc + (range.max_btc - range.min_btc) * range_position);
        }
    }

    None
}
