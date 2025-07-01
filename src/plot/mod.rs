pub mod canvas;
pub mod bar;
pub mod line;
pub mod scatter;
pub mod histogram;
pub mod density;
pub mod boxplot;

// Shared utility modules
pub mod color_utils;
pub mod axis_utils;
pub mod data_utils;
pub mod render_utils;

use crate::data::{DataFrame, PlotConfig};
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
pub use boxplot::BoxPlot;
#[allow(unused_imports)]
pub use boxplot::{NotchedBoxPlot, Orientation, OutlierMethod};

// Export utility modules
pub use color_utils::ColorUtils;
pub use data_utils::DataUtils;
pub use render_utils::RenderUtils;

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
    BoxPlot,
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
            "box" | "boxplot" => Some(PlotType::BoxPlot),
            "count" => Some(PlotType::Count),
            _ => None,
        }
    }
}

#[allow(dead_code)]
pub struct PlotRenderer;

#[allow(dead_code)]
impl PlotRenderer {
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
            PlotType::BoxPlot => {
                let chart = BoxPlot::vertical();
                chart.render(data, config)
            }
            PlotType::Count => {
                // For count plots, convert data to histogram
                let chart = Histogram::auto_bins();
                chart.render(data, config)
            }
        }
    }
}