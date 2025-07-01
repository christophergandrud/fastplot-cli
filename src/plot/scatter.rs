#![allow(dead_code)]

use crate::data::{DataFrame, PlotConfig};
use crate::plot::{Canvas, ColorUtils};
use anyhow::{Result, anyhow};
use crossterm::style::Color;

pub struct ScatterPlot {
    point_size: usize,
}

impl ScatterPlot {
    pub fn new(point_size: usize) -> Self {
        Self { point_size: point_size.max(1) }
    }

    pub fn default() -> Self {
        Self::new(1)
    }

    pub fn large_points() -> Self {
        Self::new(2)
    }

    pub fn render(&self, data: &DataFrame, config: &PlotConfig) -> Result<String> {
        if data.columns.len() < 2 {
            return Err(anyhow!("Scatter plot requires at least 2 data series (X and Y)"));
        }

        let x_series = &data.columns[0];
        let y_series = &data.columns[1];

        if x_series.data.len() != y_series.data.len() {
            return Err(anyhow!("X and Y data series must have the same length"));
        }

        if x_series.data.is_empty() {
            return Err(anyhow!("Empty data series"));
        }

        let mut canvas = Canvas::with_labels(
            config.width,
            config.height,
            config.title.clone(),
            config.xlabel.clone(),
            config.ylabel.clone(),
        );

        self.render_points(&mut canvas, x_series, y_series, config)?;

        Ok(canvas.render_colored(config.color.is_some()))
    }

    fn render_points(
        &self,
        canvas: &mut Canvas,
        x_series: &crate::data::Series,
        y_series: &crate::data::Series,
        config: &PlotConfig,
    ) -> Result<()> {
        let x_data = &x_series.data;
        let y_data = &y_series.data;

        // Find ranges
        let x_min = x_data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let x_max = x_data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let y_min = y_data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let y_max = y_data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        // Handle edge cases where all values are the same
        let (x_range_min, x_range_max) = if (x_max - x_min).abs() < f64::EPSILON {
            (x_min - 1.0, x_max + 1.0)
        } else {
            let padding = (x_max - x_min) * 0.05;
            (x_min - padding, x_max + padding)
        };

        let (y_range_min, y_range_max) = if (y_max - y_min).abs() < f64::EPSILON {
            (y_min - 1.0, y_max + 1.0)
        } else {
            let padding = (y_max - y_min) * 0.05;
            (y_min - padding, y_max + padding)
        };

        // Apply user-specified limits if provided
        let final_x_range = if let Some((x_lim_min, x_lim_max)) = config.xlim {
            (x_lim_min, x_lim_max)
        } else {
            (x_range_min, x_range_max)
        };

        let final_y_range = if let Some((y_lim_min, y_lim_max)) = config.ylim {
            (y_lim_min, y_lim_max)
        } else {
            (y_range_min, y_range_max)
        };

        canvas.set_ranges(final_x_range, final_y_range);
        canvas.draw_axis();

        let color = ColorUtils::parse_color(&config.color);
        let symbol = config.symbol.unwrap_or('●');

        // Plot points
        for (&x, &y) in x_data.iter().zip(y_data.iter()) {
            if canvas.is_point_in_bounds(x, y) {
                self.plot_point(canvas, x, y, symbol, color);
            }
        }

        Ok(())
    }

    fn plot_point(&self, canvas: &mut Canvas, x: f64, y: f64, symbol: char, color: Option<Color>) {
        if self.point_size == 1 {
            canvas.plot_point_with_color(x, y, symbol, color);
        } else {
            // For larger points, plot in a small area around the center
            let offset = (self.point_size as f64 - 1.0) / 2.0;
            for i in 0..self.point_size {
                for j in 0..self.point_size {
                    let px = x - offset + i as f64 * 0.1;
                    let py = y - offset + j as f64 * 0.1;
                    if canvas.is_point_in_bounds(px, py) {
                        canvas.plot_point_with_color(px, py, symbol, color);
                    }
                }
            }
        }
    }

    // Color parsing method removed - now using shared ColorUtils
}

/// Special scatter plot for multi-dimensional data
pub struct MultiScatterPlot {
    color_series_index: Option<usize>,
    size_series_index: Option<usize>,
}

impl MultiScatterPlot {
    pub fn new() -> Self {
        Self {
            color_series_index: None,
            size_series_index: None,
        }
    }

    pub fn with_color_series(mut self, index: usize) -> Self {
        self.color_series_index = Some(index);
        self
    }

    pub fn with_size_series(mut self, index: usize) -> Self {
        self.size_series_index = Some(index);
        self
    }

    pub fn render(&self, data: &DataFrame, config: &PlotConfig) -> Result<String> {
        if data.columns.len() < 2 {
            return Err(anyhow!("Scatter plot requires at least 2 data series (X and Y)"));
        }

        let x_series = &data.columns[0];
        let y_series = &data.columns[1];

        if x_series.data.len() != y_series.data.len() {
            return Err(anyhow!("X and Y data series must have the same length"));
        }

        if x_series.data.is_empty() {
            return Err(anyhow!("Empty data series"));
        }

        let mut canvas = Canvas::with_labels(
            config.width,
            config.height,
            config.title.clone(),
            config.xlabel.clone(),
            config.ylabel.clone(),
        );

        self.render_multi_dimensional_points(&mut canvas, data, config)?;

        Ok(canvas.render_colored(true)) // Always use colors for multi-dimensional plots
    }

    fn render_multi_dimensional_points(
        &self,
        canvas: &mut Canvas,
        data: &DataFrame,
        config: &PlotConfig,
    ) -> Result<()> {
        let x_data = &data.columns[0].data;
        let y_data = &data.columns[1].data;

        // Find ranges for X and Y
        let x_min = x_data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let x_max = x_data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let y_min = y_data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let y_max = y_data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        let x_padding = (x_max - x_min) * 0.05;
        let y_padding = (y_max - y_min) * 0.05;

        canvas.set_ranges((x_min - x_padding, x_max + x_padding), (y_min - y_padding, y_max + y_padding));
        canvas.draw_axis();

        // Get color data if specified
        let color_data = self.color_series_index
            .and_then(|idx| data.columns.get(idx))
            .map(|series| &series.data);

        // Get size data if specified
        let size_data = self.size_series_index
            .and_then(|idx| data.columns.get(idx))
            .map(|series| &series.data);

        let colors = [
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Yellow,
            Color::Magenta,
            Color::Cyan,
        ];

        let symbols = ['●', '■', '▲', '◆', '▼', '★'];

        // Plot points with varying colors and sizes
        for i in 0..x_data.len() {
            let x = x_data[i];
            let y = y_data[i];

            if !canvas.is_point_in_bounds(x, y) {
                continue;
            }

            // Determine color
            let color = if let Some(color_values) = color_data {
                let color_val = color_values.get(i).copied().unwrap_or(0.0);
                let color_idx = (color_val.abs() as usize) % colors.len();
                Some(colors[color_idx])
            } else {
                None
            };

            // Determine symbol/size
            let (symbol, point_size) = if let Some(size_values) = size_data {
                let size_val = size_values.get(i).copied().unwrap_or(1.0);
                let symbol_idx = (size_val.abs() as usize) % symbols.len();
                let point_size = ((size_val.abs() / 2.0).max(1.0).min(3.0)) as usize;
                (symbols[symbol_idx], point_size)
            } else {
                (config.symbol.unwrap_or('●'), 1)
            };

            // Plot the point
            self.plot_point_with_size(canvas, x, y, symbol, color, point_size);
        }

        Ok(())
    }

    fn plot_point_with_size(&self, canvas: &mut Canvas, x: f64, y: f64, symbol: char, color: Option<Color>, size: usize) {
        if size == 1 {
            canvas.plot_point_with_color(x, y, symbol, color);
        } else {
            let offset = (size as f64 - 1.0) / 2.0;
            for i in 0..size {
                for j in 0..size {
                    let px = x - offset + i as f64 * 0.05;
                    let py = y - offset + j as f64 * 0.05;
                    if canvas.is_point_in_bounds(px, py) {
                        canvas.plot_point_with_color(px, py, symbol, color);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{Series, DataFrame, PlotConfig, DataFormat};

    #[test]
    fn test_basic_scatter_plot() {
        let x_series = Series {
            name: "X".to_string(),
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        };
        let y_series = Series {
            name: "Y".to_string(),
            data: vec![2.0, 4.0, 1.0, 5.0, 3.0],
        };
        let dataframe = DataFrame {
            columns: vec![x_series, y_series],
            headers: None,
        };
        let config = PlotConfig {
            width: 50,
            height: 25,
            title: Some("Scatter Plot".to_string()),
            xlabel: Some("X Values".to_string()),
            ylabel: Some("Y Values".to_string()),
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: Some("blue".to_string()),
            symbol: Some('●'),
        };

        let scatter_plot = ScatterPlot::default();
        let result = scatter_plot.render(&dataframe, &config);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("Scatter Plot"));
        assert!(!output.is_empty());
    }

    #[test]
    fn test_large_points_scatter() {
        let x_series = Series {
            name: "X".to_string(),
            data: vec![1.0, 2.0, 3.0],
        };
        let y_series = Series {
            name: "Y".to_string(),
            data: vec![1.0, 2.0, 3.0],
        };
        let dataframe = DataFrame {
            columns: vec![x_series, y_series],
            headers: None,
        };
        let config = PlotConfig {
            width: 30,
            height: 20,
            title: Some("Large Points".to_string()),
            xlabel: None,
            ylabel: None,
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: Some("red".to_string()),
            symbol: Some('■'),
        };

        let scatter_plot = ScatterPlot::large_points();
        let result = scatter_plot.render(&dataframe, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mismatched_series_length() {
        let x_series = Series {
            name: "X".to_string(),
            data: vec![1.0, 2.0, 3.0],
        };
        let y_series = Series {
            name: "Y".to_string(),
            data: vec![1.0, 2.0], // Different length
        };
        let dataframe = DataFrame {
            columns: vec![x_series, y_series],
            headers: None,
        };
        let config = PlotConfig {
            width: 30,
            height: 20,
            title: None,
            xlabel: None,
            ylabel: None,
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: None,
            symbol: None,
        };

        let scatter_plot = ScatterPlot::default();
        let result = scatter_plot.render(&dataframe, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_multi_dimensional_scatter() {
        let x_series = Series {
            name: "X".to_string(),
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        };
        let y_series = Series {
            name: "Y".to_string(),
            data: vec![2.0, 4.0, 1.0, 5.0, 3.0],
        };
        let color_series = Series {
            name: "Color".to_string(),
            data: vec![1.0, 2.0, 1.0, 3.0, 2.0],
        };
        let size_series = Series {
            name: "Size".to_string(),
            data: vec![1.0, 2.0, 3.0, 2.0, 1.0],
        };
        
        let dataframe = DataFrame {
            columns: vec![x_series, y_series, color_series, size_series],
            headers: None,
        };
        
        let config = PlotConfig {
            width: 50,
            height: 25,
            title: Some("Multi-Dimensional Scatter".to_string()),
            xlabel: Some("X".to_string()),
            ylabel: Some("Y".to_string()),
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: None,
            symbol: None,
        };

        let scatter_plot = MultiScatterPlot::new()
            .with_color_series(2)
            .with_size_series(3);
        
        let result = scatter_plot.render(&dataframe, &config);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("Multi-Dimensional Scatter"));
        assert!(!output.is_empty());
    }
}