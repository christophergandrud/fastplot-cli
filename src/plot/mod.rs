pub mod canvas;

use crate::data::{DataFrame, PlotConfig};
use anyhow::Result;

pub use canvas::Canvas;

pub trait Plot {
    fn render(&self, data: &DataFrame, config: &PlotConfig) -> Result<String>;
}

pub struct LinePlot;
pub struct BarPlot;
pub struct ScatterPlot;
pub struct Histogram;

impl Plot for LinePlot {
    fn render(&self, data: &DataFrame, config: &PlotConfig) -> Result<String> {
        // Placeholder implementation
        Ok(format!("Line plot: {} x {} (title: {:?})", 
                  config.width, config.height, config.title))
    }
}

impl Plot for BarPlot {
    fn render(&self, data: &DataFrame, config: &PlotConfig) -> Result<String> {
        // Placeholder implementation
        Ok(format!("Bar plot: {} x {} (title: {:?})", 
                  config.width, config.height, config.title))
    }
}

impl Plot for ScatterPlot {
    fn render(&self, data: &DataFrame, config: &PlotConfig) -> Result<String> {
        // Placeholder implementation
        Ok(format!("Scatter plot: {} x {} (title: {:?})", 
                  config.width, config.height, config.title))
    }
}

impl Plot for Histogram {
    fn render(&self, data: &DataFrame, config: &PlotConfig) -> Result<String> {
        // Placeholder implementation
        Ok(format!("Histogram: {} x {} (title: {:?})", 
                  config.width, config.height, config.title))
    }
}