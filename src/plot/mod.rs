pub mod canvas;
pub mod bar;
pub mod line;
pub mod scatter;
pub mod histogram;
pub mod density;
pub mod layout;

// Shared utility modules
pub mod color_utils;
pub mod axis_utils;
pub mod axis_ticks;
pub mod data_utils;
pub mod render_utils;

use crate::data::{DataFrame, PlotConfig, Series};
use anyhow::anyhow;
use anyhow::Result;

pub use canvas::Canvas;
pub use bar::BarChart;
pub use line::LinePlot;
pub use scatter::ScatterPlot;
#[allow(unused_imports)]
pub use scatter::MultiScatterPlot;
pub use histogram::Histogram;
#[allow(unused_imports)]
pub use histogram::CumulativeHistogram;
pub use density::{DensityPlot, KernelType};

// Export utility modules
pub use color_utils::ColorUtils;
pub use data_utils::DataUtils;
pub use render_utils::RenderUtils;
pub use axis_ticks::AxisTickGenerator;

// Export layout system
pub use layout::{
    ElementLayout, BarStyle, AxisRenderer
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PlotType {
    Bar,
    BarHorizontal,
    Line,
    Lines,
    Scatter,
    Histogram,
    Density,
    Count,
}

#[allow(dead_code)]
impl PlotType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "bar" => Some(PlotType::Bar),
            "barh" | "bar-horizontal" => Some(PlotType::BarHorizontal),
            "line" => Some(PlotType::Line),
            "lines" => Some(PlotType::Lines),
            "scatter" => Some(PlotType::Scatter),
            "hist" | "histogram" => Some(PlotType::Histogram),
            "density" => Some(PlotType::Density),
            "count" => Some(PlotType::Count),
            _ => None,
        }
    }
}

#[allow(dead_code)]
pub struct PlotRenderer;

#[allow(dead_code)]
impl PlotRenderer {
    /// Create frequency count data from raw values for count plots
    fn create_count_data(data: &DataFrame) -> Result<DataFrame> {
        use std::collections::HashMap;
        
        if data.columns.is_empty() {
            return Err(anyhow!("No data provided for count plot"));
        }
        
        let series = &data.columns[0];
        let mut counts: HashMap<i64, usize> = HashMap::new();
        
        // Count occurrences of each value (assuming they can be converted to integers)
        for &value in &series.data {
            let rounded_value = value.round() as i64;
            *counts.entry(rounded_value).or_insert(0) += 1;
        }
        
        // Sort by value for consistent ordering
        let mut sorted_counts: Vec<(i64, usize)> = counts.into_iter().collect();
        sorted_counts.sort_by_key(|&(value, _)| value);
        
        // Create new DataFrame with counts as the data
        let count_values: Vec<f64> = sorted_counts.iter().map(|(_, count)| *count as f64).collect();
        
        let count_series = Series {
            name: format!("Count of {}", series.name),
            data: count_values,
        };
        
        Ok(DataFrame {
            columns: vec![count_series],
            headers: Some(sorted_counts.iter().map(|(value, _)| value.to_string()).collect()),
        })
    }

    pub fn render(plot_type: PlotType, data: &DataFrame, config: &PlotConfig) -> Result<String> {
        match plot_type {
            PlotType::Bar => {
                let chart = BarChart::vertical();
                chart.render(data, config)
            }
            PlotType::BarHorizontal => {
                let chart = BarChart::horizontal();
                chart.render(data, config)
            }
            PlotType::Line => {
                let chart = LinePlot::single();
                chart.render(data, config)
            }
            PlotType::Lines => {
                let chart = LinePlot::multi();
                chart.render(data, config)
            }
            PlotType::Scatter => {
                let chart = ScatterPlot::default();
                chart.render(data, config)
            }
            PlotType::Histogram => {
                let chart = Histogram::auto_bins();
                chart.render(data, config)
            }
            PlotType::Density => {
                let chart = DensityPlot::auto_bandwidth()
                    .with_kernel(KernelType::Gaussian)
                    .with_resolution(200);
                chart.render(data, config)
            }
            PlotType::Count => {
                // For count plots, create frequency counts and use bar chart
                let count_data = Self::create_count_data(data)?;
                let chart = BarChart::vertical();
                chart.render(&count_data, config)
            }
        }
    }
}