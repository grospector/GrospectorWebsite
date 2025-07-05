use crate::types::bitcoin::{BitcoinDistribution, PercentileResult};
use crate::utils::chart_theme::{format_bitcoin_amount, format_percentile, MempoolChartTheme};
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ComparisonChartProps {
    pub user_result: PercentileResult,
    pub distribution: BitcoinDistribution,
}

#[function_component(ComparisonChart)]
pub fn comparison_chart(props: &ComparisonChartProps) -> Html {
    let canvas_ref = use_node_ref();

    // Effect to draw the chart when data changes
    {
        let canvas_ref = canvas_ref.clone();
        let user_result = props.user_result.clone();
        let distribution = props.distribution.clone();

        use_effect_with((user_result.clone(), distribution.clone()), move |_| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let _ = draw_comparison_chart(canvas, &user_result, &distribution);
            }
            || ()
        });
    }

    html! {
        <div class="bg-gradient-to-br from-white to-gray-50 dark:from-gray-800 dark:to-gray-900 rounded-xl shadow-xl p-8 border border-gray-200 dark:border-gray-700 transform hover:shadow-2xl transition-all duration-300">
            // Enhanced Header Section
            <div class="mb-8">
                <div class="flex items-center mb-4">
                    <div class="text-3xl mr-3">{"🎯"}</div>
                    <div>
                        <h3 class="text-3xl font-bold text-gray-900 dark:text-white">{"Your Position Comparison"}</h3>
                        <p class="text-lg text-gray-600 dark:text-gray-300">{"Detailed analysis of your Bitcoin holdings in the global context"}</p>
                    </div>
                </div>
            </div>

            // Enhanced Chart Container
            <div class="relative bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700 shadow-lg hover:shadow-xl transition-shadow duration-300">
                <div class="flex items-center mb-4">
                    <div class="text-2xl mr-3">{"📈"}</div>
                    <div>
                        <h4 class="text-xl font-bold text-gray-900 dark:text-white">{"Percentile Distribution Curve"}</h4>
                        <p class="text-sm text-gray-600 dark:text-gray-300">{"Your position on the global Bitcoin wealth distribution curve"}</p>
                    </div>
                </div>
                
                <canvas
                    ref={canvas_ref}
                    width="800"
                    height="300"
                    class="w-full h-auto rounded-lg"
                    style="max-width: 100%; height: auto;"
                />
                
                // Enhanced Legend and Insights
                <div class="mt-6 grid grid-cols-1 lg:grid-cols-2 gap-6">
                    // Chart Legend
                    <div class="bg-gray-50 dark:bg-gray-900 rounded-xl p-4">
                        <h5 class="text-lg font-semibold mb-3 text-gray-900 dark:text-white flex items-center">
                            <div class="text-xl mr-2">{"🎨"}</div>
                            {"Chart Elements"}
                        </h5>
                        <div class="space-y-3">
                            <div class="flex items-center p-3 bg-white dark:bg-gray-800 rounded-lg shadow-sm">
                                <div class="w-6 h-6 rounded-lg mr-3" style="background: linear-gradient(135deg, #3B82F6, #1E40AF);"></div>
                                <div>
                                    <div class="font-semibold text-gray-900 dark:text-white">{"Distribution Curve"}</div>
                                    <div class="text-sm text-gray-600 dark:text-gray-300">{"Global Bitcoin percentile curve"}</div>
                                </div>
                            </div>
                            <div class="flex items-center p-3 bg-white dark:bg-gray-800 rounded-lg shadow-sm">
                                <div class="w-6 h-6 rounded-lg mr-3" style="background: linear-gradient(135deg, #F7931A, #FF8C42);"></div>
                                <div>
                                    <div class="font-semibold text-gray-900 dark:text-white">{"Your Position"}</div>
                                    <div class="text-sm text-gray-600 dark:text-gray-300">{"Highlighted marker showing your rank"}</div>
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    // Position Insights
                    <div class="bg-gradient-to-r from-green-50 to-emerald-50 dark:from-green-900/20 dark:to-emerald-900/20 rounded-xl p-4 border border-green-200 dark:border-green-800">
                        <div class="flex items-center mb-3">
                            <div class="text-2xl mr-2">{"💡"}</div>
                            <h5 class="text-lg font-semibold text-gray-900 dark:text-white">{"Position Analysis"}</h5>
                        </div>
                        <div class="space-y-2">
                            <p class="text-sm text-gray-700 dark:text-gray-300">
                                {"Your position at the "}<span class="font-semibold text-green-600 dark:text-green-400">{format!("{:.1}%", props.user_result.percentile)}</span>{" percentile means you hold more Bitcoin than "}<span class="font-semibold text-green-600 dark:text-green-400">{format!("{:.1}%", props.user_result.percentile)}</span>{" of all Bitcoin addresses."}
                            </p>
                            <p class="text-sm text-gray-700 dark:text-gray-300">
                                {"The curve shows how Bitcoin wealth is distributed globally, with your exact position highlighted in orange."}
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn draw_comparison_chart(
    canvas: HtmlCanvasElement,
    user_result: &PercentileResult,
    distribution: &BitcoinDistribution,
) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
    let root = backend.into_drawing_area();

    // Get mempool.space inspired theme
    let theme = MempoolChartTheme::new();
    root.fill(&theme.background)?;

    // Create a percentile-based visualization with more data points for smooth curve
    let percentiles = vec![0.0, 10.0, 25.0, 50.0, 75.0, 90.0, 95.0, 99.0, 99.9, 100.0];
    let mut percentile_data = Vec::new();

    for percentile in percentiles {
        if let Some(btc_amount) = calculate_percentile_amount(distribution, percentile) {
            percentile_data.push((percentile, btc_amount));
        }
    }

    if percentile_data.is_empty() {
        return Ok(());
    }

    let max_btc = percentile_data
        .iter()
        .map(|(_, btc)| *btc)
        .fold(0.0, f64::max);
    let min_btc = percentile_data
        .iter()
        .map(|(_, btc)| *btc)
        .fold(f64::INFINITY, f64::min)
        .max(0.0001);

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Bitcoin Holdings by Percentile",
            theme.create_text_style(18),
        )
        .margin(15)
        .x_label_area_size(50)
        .y_label_area_size(70)
        .build_cartesian_2d(0.0..100.0, (min_btc..max_btc).log_scale())?;

    chart
        .configure_mesh()
        .x_desc("Percentile (%)")
        .y_desc("Bitcoin Amount (BTC)")
        .x_label_formatter(&|x| format!("{:.0}%", x))
        .y_label_formatter(&|y| format_bitcoin_amount(*y).replace(" BTC", ""))
        .label_style(theme.create_secondary_text_style(12))
        .axis_style(&theme.grid_color)
        .draw()?;

    // Highlight wealth categories with subtle background colors
    let wealth_ranges = get_wealth_category_ranges(&theme);
    for (category, color, start, end) in wealth_ranges {
        if start <= 100.0 && end >= 0.0 {
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(start.max(0.0), min_btc), (end.min(100.0), max_btc)],
                    color.mix(0.05).filled(),
                )))?
                .label(&category);
        }
    }

    // Draw the percentile curve with Bitcoin orange gradient
    chart
        .draw_series(percentile_data.windows(2).map(|window| {
            PathElement::new(
                vec![(window[0].0, window[0].1), (window[1].0, window[1].1)],
                theme.chart_secondary.stroke_width(3),
            )
        }))?
        .label("Distribution Curve")
        .legend(|(x, y)| {
            PathElement::new(
                vec![(x, y), (x + 15, y)],
                theme.chart_secondary.stroke_width(3),
            )
        });

    // Draw user position with prominent Bitcoin orange styling
    let user_percentile = user_result.percentile;
    let user_amount = user_result.user_bitcoin_amount;

    if user_amount >= min_btc && user_amount <= max_btc {
        // Vertical line at user percentile with Bitcoin orange
        chart.draw_series(std::iter::once(PathElement::new(
            vec![(user_percentile, min_btc), (user_percentile, max_btc)],
            theme.bitcoin_orange.stroke_width(4),
        )))?;

        // Point at user position with Bitcoin orange
        chart
            .draw_series(std::iter::once(Circle::new(
                (user_percentile, user_amount),
                10,
                theme.bitcoin_orange.filled(),
            )))?
            .label("Your Position")
            .legend(|(x, y)| Circle::new((x + 7, y), 5, theme.bitcoin_orange.filled()));

        // User label with better formatting
        let user_label = format!(
            "You: {} ({})",
            format_percentile(user_percentile),
            format_bitcoin_amount(user_amount)
        );

        let label_x = if user_percentile > 70.0 {
            user_percentile - 15.0
        } else {
            user_percentile + 2.0
        };
        chart.draw_series(std::iter::once(Text::new(
            user_label,
            (label_x, user_amount * 2.0),
            theme.create_text_style(12),
        )))?;
    }

    // Draw key percentile markers with subtle styling
    for (percentile, btc_amount) in percentile_data.iter() {
        if (*percentile == 25.0
            || *percentile == 50.0
            || *percentile == 75.0
            || *percentile == 90.0
            || *percentile == 95.0)
            && (*percentile - user_percentile).abs() > 5.0
        {
            // Draw subtle marker
            chart.draw_series(std::iter::once(Circle::new(
                (*percentile, *btc_amount),
                4,
                theme.text_muted.filled(),
            )))?;

            // Add percentile label
            chart.draw_series(std::iter::once(Text::new(
                format!("{}%", percentile),
                (*percentile, *btc_amount * 0.6),
                theme.create_muted_text_style(9),
            )))?;
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
            let range_position = (target_count - (cumulative_addresses - range.address_count))
                as f64
                / range.address_count as f64;
            return Some(range.min_btc + (range.max_btc - range.min_btc) * range_position);
        }
    }

    None
}

fn get_wealth_category_ranges(theme: &MempoolChartTheme) -> Vec<(String, RGBColor, f64, f64)> {
    let wealth_colors = theme.get_wealth_colors();
    vec![
        ("Shrimp".to_string(), wealth_colors[0].1, 0.0, 50.0),
        ("Crab".to_string(), wealth_colors[1].1, 50.0, 75.0),
        ("Fish".to_string(), wealth_colors[2].1, 75.0, 90.0),
        ("Dolphin".to_string(), wealth_colors[3].1, 90.0, 95.0),
        ("Shark".to_string(), wealth_colors[4].1, 95.0, 99.0),
        ("Whale".to_string(), wealth_colors[5].1, 99.0, 100.0),
    ]
}
