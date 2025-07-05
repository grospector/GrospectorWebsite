use crate::types::bitcoin::BitcoinDistribution;
use crate::utils::chart_theme::{format_large_number, MempoolChartTheme};
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StatisticsChartProps {
    pub distribution: BitcoinDistribution,
    pub bitcoin_price: f64,
}

#[function_component(StatisticsChart)]
pub fn statistics_chart(props: &StatisticsChartProps) -> Html {
    let supply_canvas_ref = use_node_ref();
    let address_canvas_ref = use_node_ref();

    // Effect to draw the supply concentration chart
    {
        let supply_canvas_ref = supply_canvas_ref.clone();
        let distribution = props.distribution.clone();

        use_effect_with(distribution.clone(), move |_| {
            if let Some(canvas) = supply_canvas_ref.cast::<HtmlCanvasElement>() {
                let _ = draw_supply_concentration_chart(canvas, &distribution);
            }
            || ()
        });
    }

    // Effect to draw the address distribution chart
    {
        let address_canvas_ref = address_canvas_ref.clone();
        let distribution = props.distribution.clone();

        use_effect_with(distribution.clone(), move |_| {
            if let Some(canvas) = address_canvas_ref.cast::<HtmlCanvasElement>() {
                let _ = draw_address_distribution_chart(canvas, &distribution);
            }
            || ()
        });
    }

    html! {
        <div class="bg-gradient-to-br from-white to-gray-50 dark:from-gray-800 dark:to-gray-900 rounded-xl shadow-xl p-8 border border-gray-200 dark:border-gray-700 transform hover:shadow-2xl transition-all duration-300">
            // Simplified Header Section - Cards removed to avoid duplication with main app
            <div class="mb-8">
                <div class="flex items-center mb-4">
                    <div class="text-3xl mr-3">{"🍰"}</div>
                    <div>
                        <h3 class="text-3xl font-bold text-gray-900 dark:text-white">{"Bitcoin Supply Distribution"}</h3>
                        <p class="text-lg text-gray-600 dark:text-gray-300">{"How Bitcoin wealth is distributed across address ranges"}</p>
                    </div>
                </div>
            </div>

            // Charts Grid - Separated for Better UX with Individual Canvas Elements
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                // Supply Concentration Chart
                <div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700 shadow-lg hover:shadow-xl transition-shadow duration-300">
                    <div class="flex items-center mb-4">
                        <div class="text-2xl mr-3">{"📊"}</div>
                        <div>
                            <h4 class="text-xl font-bold text-gray-900 dark:text-white">{"Supply Concentration"}</h4>
                            <p class="text-sm text-gray-600 dark:text-gray-300">{"How Bitcoin is distributed among holders"}</p>
                        </div>
                    </div>
                    <div class="h-64 relative bg-gray-50 dark:bg-gray-700 rounded-lg overflow-hidden">
                        <canvas
                            ref={supply_canvas_ref.clone()}
                            width="400"
                            height="300"
                            class="w-full h-full object-contain"
                            id="supply-chart"
                        />
                    </div>
                    // Supply Concentration Statistics
                    <div class="mt-4 grid grid-cols-2 gap-3">
                        <div class="bg-gradient-to-r from-orange-50 to-orange-100 dark:from-orange-900/20 dark:to-orange-800/20 rounded-lg p-3 text-center border border-orange-200 dark:border-orange-700">
                            <div class="text-lg font-bold text-orange-600 dark:text-orange-400">{"~40%"}</div>
                            <div class="text-xs text-orange-700 dark:text-orange-300">{"Top 1% holds"}</div>
                        </div>
                        <div class="bg-gradient-to-r from-blue-50 to-blue-100 dark:from-blue-900/20 dark:to-blue-800/20 rounded-lg p-3 text-center border border-blue-200 dark:border-blue-700">
                            <div class="text-lg font-bold text-blue-600 dark:text-blue-400">{"~85%"}</div>
                            <div class="text-xs text-blue-700 dark:text-blue-300">{"Top 10% holds"}</div>
                        </div>
                    </div>
                </div>

                // Address Distribution Chart
                <div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700 shadow-lg hover:shadow-xl transition-shadow duration-300">
                    <div class="flex items-center mb-4">
                        <div class="text-2xl mr-3">{"📈"}</div>
                        <div>
                            <h4 class="text-xl font-bold text-gray-900 dark:text-white">{"Top Address Ranges"}</h4>
                            <p class="text-sm text-gray-600 dark:text-gray-300">{"Most common Bitcoin holding amounts"}</p>
                        </div>
                    </div>
                    <div class="h-64 relative bg-gray-50 dark:bg-gray-700 rounded-lg overflow-hidden">
                        <canvas
                            ref={address_canvas_ref.clone()}
                            width="400"
                            height="300"
                            class="w-full h-full object-contain"
                            id="address-chart"
                        />
                    </div>
                    // Address Range Quick Stats
                    <div class="mt-4 grid grid-cols-3 gap-2">
                        <div class="bg-gradient-to-r from-green-50 to-green-100 dark:from-green-900/20 dark:to-green-800/20 rounded-lg p-2 text-center border border-green-200 dark:border-green-700">
                            <div class="text-sm font-bold text-green-600 dark:text-green-400">{"<0.001"}</div>
                            <div class="text-xs text-green-700 dark:text-green-300">{"Most common"}</div>
                        </div>
                        <div class="bg-gradient-to-r from-purple-50 to-purple-100 dark:from-purple-900/20 dark:to-purple-800/20 rounded-lg p-2 text-center border border-purple-200 dark:border-purple-700">
                            <div class="text-sm font-bold text-purple-600 dark:text-purple-400">{"0.1-1"}</div>
                            <div class="text-xs text-purple-700 dark:text-purple-300">{"Growing"}</div>
                        </div>
                        <div class="bg-gradient-to-r from-indigo-50 to-indigo-100 dark:from-indigo-900/20 dark:to-indigo-800/20 rounded-lg p-2 text-center border border-indigo-200 dark:border-indigo-700">
                            <div class="text-sm font-bold text-indigo-600 dark:text-indigo-400">{"1000+"}</div>
                            <div class="text-xs text-indigo-700 dark:text-indigo-300">{"Whales"}</div>
                        </div>
                    </div>
                </div>
            </div>

            // Distribution Insights Section
            <div class="mt-8 grid grid-cols-1 md:grid-cols-2 gap-6">
                <div class="bg-gradient-to-r from-purple-50 to-indigo-50 dark:from-purple-900/20 dark:to-indigo-900/20 rounded-xl p-6 border border-purple-200 dark:border-purple-800">
                    <div class="flex items-center mb-4">
                        <div class="text-2xl mr-3">{"🎯"}</div>
                        <h4 class="text-lg font-semibold text-gray-900 dark:text-white">{"Wealth Concentration"}</h4>
                    </div>
                    <div class="space-y-3">
                        <div class="flex items-center justify-between p-3 bg-white dark:bg-gray-800 rounded-lg">
                            <span class="text-sm text-gray-700 dark:text-gray-300">{"Top 1% of addresses"}</span>
                            <span class="font-semibold text-purple-600 dark:text-purple-400">{"~40%+ of supply"}</span>
                        </div>
                        <div class="flex items-center justify-between p-3 bg-white dark:bg-gray-800 rounded-lg">
                            <span class="text-sm text-gray-700 dark:text-gray-300">{"Top 10% of addresses"}</span>
                            <span class="font-semibold text-purple-600 dark:text-purple-400">{"~85%+ of supply"}</span>
                        </div>
                        <div class="flex items-center justify-between p-3 bg-white dark:bg-gray-800 rounded-lg">
                            <span class="text-sm text-gray-700 dark:text-gray-300">{"Bottom 50% of addresses"}</span>
                            <span class="font-semibold text-purple-600 dark:text-purple-400">{"<1% of supply"}</span>
                        </div>
                    </div>
                </div>

                <div class="bg-gradient-to-r from-green-50 to-emerald-50 dark:from-green-900/20 dark:to-emerald-900/20 rounded-xl p-6 border border-green-200 dark:border-green-800">
                    <div class="flex items-center mb-4">
                        <div class="text-2xl mr-3">{"📋"}</div>
                        <h4 class="text-lg font-semibold text-gray-900 dark:text-white">{"Distribution Facts"}</h4>
                    </div>
                    <div class="space-y-3">
                        <div class="flex items-start p-3 bg-white dark:bg-gray-800 rounded-lg">
                            <div class="text-green-500 mr-2 mt-0.5">{"✓"}</div>
                            <span class="text-sm text-gray-700 dark:text-gray-300">{"Highly concentrated wealth distribution"}</span>
                        </div>
                        <div class="flex items-start p-3 bg-white dark:bg-gray-800 rounded-lg">
                            <div class="text-green-500 mr-2 mt-0.5">{"✓"}</div>
                            <span class="text-sm text-gray-700 dark:text-gray-300">{"Long-tail distribution pattern"}</span>
                        </div>
                        <div class="flex items-start p-3 bg-white dark:bg-gray-800 rounded-lg">
                            <div class="text-green-500 mr-2 mt-0.5">{"✓"}</div>
                            <span class="text-sm text-gray-700 dark:text-gray-300">{"Most addresses hold small amounts"}</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Draw the supply concentration chart as an enhanced pie chart
fn draw_supply_concentration_chart(
    canvas: HtmlCanvasElement,
    distribution: &BitcoinDistribution,
) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
    let root = backend.into_drawing_area();
    let theme = MempoolChartTheme::new();
    
    root.fill(&theme.background)?;

    // Calculate dynamic supply concentration based on actual data
    let mut supply_ranges = Vec::new();
    
    // Calculate supply by percentiles more accurately
    let mut ranges_sorted = distribution.ranges.clone();
    ranges_sorted.sort_by(|a, b| b.min_btc.partial_cmp(&a.min_btc).unwrap());
    let mut top_1_percent = 0.0;
    let mut top_10_percent = 0.0;
    let mut top_50_percent = 0.0;
    let mut remaining = 0.0;
    
    let mut cumulative_addresses = 0u64;
    
    for range in &ranges_sorted {
        let range_supply = ((range.max_btc + range.min_btc) / 2.0) * range.address_count as f64;
        cumulative_addresses += range.address_count;
        
        let address_percentile = (cumulative_addresses as f64 / distribution.total_addresses as f64) * 100.0;
        
        if address_percentile <= 1.0 {
            top_1_percent += range_supply;
        } else if address_percentile <= 10.0 {
            top_10_percent += range_supply;
        } else if address_percentile <= 50.0 {
            top_50_percent += range_supply;
        } else {
            remaining += range_supply;
        }
    }

    // Create pie chart segments with better colors
    supply_ranges.push(("Top 1%", top_1_percent, theme.bitcoin_orange));
    supply_ranges.push(("Top 1-10%", top_10_percent, theme.chart_secondary));
    supply_ranges.push(("Top 10-50%", top_50_percent, theme.chart_accent));
    supply_ranges.push(("Bottom 50%", remaining, theme.text_muted));

    // Filter out zero values
    supply_ranges.retain(|(_, value, _)| *value > 1000.0); // Only show significant amounts

    let mut chart = ChartBuilder::on(&root)
        .caption("Bitcoin Supply Concentration", theme.create_text_style(18))
        .margin(20)
        .build_cartesian_2d(-1.3..1.3, -1.3..1.3)?;

    let total_displayed = supply_ranges.iter().map(|(_, value, _)| *value).sum::<f64>();
    let mut start_angle = 0.0;

    for (label, value, color) in supply_ranges {
        let percentage = (value / total_displayed) * 100.0;
        let angle = (value / total_displayed) * 360.0;

        // Draw pie slice with smoother curves
        let center = (0.0, 0.0);
        let radius = 1.0;
        let mut points = vec![center];

        for i in 0..=50 { // More points for smoother curves
            let current_angle = start_angle + (angle * i as f64 / 50.0);
            let rad = current_angle.to_radians();
            points.push((radius * rad.cos(), radius * rad.sin()));
        }

        chart.draw_series(std::iter::once(Polygon::new(
            points,
            color.mix(0.85).filled(),
        )))?;

        // Enhanced labels with percentage
        if angle > 20.0 { // Only show labels for significant slices
            let label_angle = (start_angle + angle / 2.0).to_radians();
            let label_radius = radius * 1.15;
            let label_pos = (
                label_radius * label_angle.cos(),
                label_radius * label_angle.sin(),
            );

            chart.draw_series(std::iter::once(Text::new(
                format!("{}\n{:.1}%", label, percentage),
                label_pos,
                theme.create_text_style(12),
            )))?;
        }

        start_angle += angle;
    }

    Ok(())
}

/// Draw the address distribution chart as an enhanced bar chart
fn draw_address_distribution_chart(
    canvas: HtmlCanvasElement,
    distribution: &BitcoinDistribution,
) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
    let root = backend.into_drawing_area();
    let theme = MempoolChartTheme::new();
    
    root.fill(&theme.background)?;

    // Select meaningful ranges for display
    let mut ranges = distribution.ranges.clone();
    ranges.sort_by(|a, b| b.address_count.cmp(&a.address_count));
    
    // Take top 8 ranges for better readability
    ranges.truncate(8);

    if ranges.is_empty() {
        return Ok(());
    }

    let max_count = ranges.iter().map(|r| r.address_count).max().unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Top Bitcoin Address Ranges", theme.create_text_style(18))
        .margin(20)
        .x_label_area_size(70)
        .y_label_area_size(80)
        .build_cartesian_2d(0..ranges.len(), 0..max_count)?;

    chart
        .configure_mesh()
        .x_desc("BTC Range")
        .y_desc("Address Count")
        .x_label_formatter(&|x| {
            if *x < ranges.len() {
                let range = &ranges[*x];
                if range.max_btc == f64::INFINITY {
                    format!("{}+", format_large_number(range.min_btc))
                } else if range.max_btc >= 1.0 {
                    format!("{}-{}", format_large_number(range.min_btc), format_large_number(range.max_btc))
                } else {
                    format!("{:.3}-{:.3}", range.min_btc, range.max_btc)
                }
            } else {
                String::new()
            }
        })
        .y_label_formatter(&|y| format_large_number(*y as f64))
        .label_style(theme.create_text_style(10))
        .axis_style(&theme.grid_color)
        .bold_line_style(&theme.border_primary)
        .draw()?;

    // Enhanced gradient colors
    let colors = vec![
        theme.bitcoin_orange,
        theme.chart_secondary,
        theme.chart_accent,
        RGBColor(139, 69, 19),  // Brown
        RGBColor(75, 0, 130),   // Indigo
        RGBColor(255, 20, 147), // Deep pink
        RGBColor(0, 100, 0),    // Dark green
        RGBColor(255, 140, 0),  // Dark orange
    ];

    chart
        .draw_series(ranges.iter().enumerate().map(|(i, range)| {
            let color = colors.get(i % colors.len()).unwrap_or(&theme.bitcoin_orange);
            Rectangle::new([(i, 0), (i, range.address_count)], color.mix(0.8).filled())
        }))?
        .label("Addresses")
        .legend(|(x, y)| Rectangle::new([(x, y), (x + 15, y + 10)], theme.bitcoin_orange.filled()));

    chart
        .configure_series_labels()
        .background_style(&theme.card_background.mix(0.9))
        .border_style(&theme.border_primary)
        .label_font(theme.create_text_style(12))
        .draw()?;

    Ok(())
}
