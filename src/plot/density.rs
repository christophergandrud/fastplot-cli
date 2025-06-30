#![allow(dead_code)]

use crate::data::{DataFrame, PlotConfig};
use crate::plot::Canvas;
use anyhow::{Result, anyhow};
use crossterm::style::Color;
use std::f64::consts::{PI, E};

pub struct DensityPlot {
    bandwidth: Option<f64>,
    kernel: KernelType,
    resolution: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum KernelType {
    Gaussian,
    Epanechnikov,
    Uniform,
    Triangular,
}

impl DensityPlot {
    pub fn new(bandwidth: Option<f64>) -> Self {
        Self {
            bandwidth,
            kernel: KernelType::Gaussian,
            resolution: 200,
        }
    }

    pub fn auto_bandwidth() -> Self {
        Self::new(None)
    }

    pub fn with_bandwidth(bandwidth: f64) -> Self {
        Self::new(Some(bandwidth))
    }

    pub fn with_kernel(mut self, kernel: KernelType) -> Self {
        self.kernel = kernel;
        self
    }

    pub fn with_resolution(mut self, resolution: usize) -> Self {
        self.resolution = resolution.max(50);
        self
    }

    pub fn render(&self, data: &DataFrame, config: &PlotConfig) -> Result<String> {
        if data.columns.is_empty() {
            return Err(anyhow!("No data provided for density plot"));
        }

        let series = &data.columns[0];
        if series.data.is_empty() {
            return Err(anyhow!("Empty data series"));
        }

        if series.data.len() < 2 {
            return Err(anyhow!("Need at least 2 data points for density estimation"));
        }

        let mut canvas = Canvas::with_labels(
            config.width,
            config.height,
            config.title.clone(),
            config.xlabel.clone(),
            config.ylabel.clone(),
        );

        let density_data = self.calculate_density(&series.data)?;
        self.render_density(&mut canvas, &density_data, config)?;

        Ok(canvas.render_colored(config.color.is_some()))
    }

    fn calculate_density(&self, data: &[f64]) -> Result<DensityData> {
        let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        if (max_val - min_val).abs() < f64::EPSILON {
            return Err(anyhow!("All data values are the same, cannot estimate density"));
        }

        // Calculate bandwidth using Scott's rule if not provided
        let bandwidth = self.bandwidth.unwrap_or_else(|| {
            let n = data.len() as f64;
            let std_dev = self.calculate_std_dev(data);
            std_dev * n.powf(-1.0 / 5.0) * 1.06
        });

        // Extend range slightly for better visualization
        let range_extension = (max_val - min_val) * 0.1;
        let x_min = min_val - range_extension;
        let x_max = max_val + range_extension;

        let mut x_values = Vec::with_capacity(self.resolution);
        let mut density_values = Vec::with_capacity(self.resolution);

        // Generate evaluation points
        for i in 0..self.resolution {
            let x = x_min + (x_max - x_min) * (i as f64 / (self.resolution - 1) as f64);
            x_values.push(x);
        }

        // Calculate density at each point
        for &x in &x_values {
            let density = self.kernel_density_estimate(data, x, bandwidth);
            density_values.push(density);
        }

        Ok(DensityData {
            x_values,
            density_values,
            bandwidth,
        })
    }

    fn kernel_density_estimate(&self, data: &[f64], x: f64, bandwidth: f64) -> f64 {
        let n = data.len() as f64;
        let mut sum = 0.0;

        for &xi in data {
            let u = (x - xi) / bandwidth;
            sum += self.kernel_function(u);
        }

        sum / (n * bandwidth)
    }

    fn kernel_function(&self, u: f64) -> f64 {
        match self.kernel {
            KernelType::Gaussian => {
                (1.0 / (2.0 * PI).sqrt()) * E.powf(-0.5 * u * u)
            }
            KernelType::Epanechnikov => {
                if u.abs() <= 1.0 {
                    0.75 * (1.0 - u * u)
                } else {
                    0.0
                }
            }
            KernelType::Uniform => {
                if u.abs() <= 1.0 {
                    0.5
                } else {
                    0.0
                }
            }
            KernelType::Triangular => {
                if u.abs() <= 1.0 {
                    1.0 - u.abs()
                } else {
                    0.0
                }
            }
        }
    }

    fn calculate_std_dev(&self, data: &[f64]) -> f64 {
        let n = data.len() as f64;
        let mean = data.iter().sum::<f64>() / n;
        let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);
        variance.sqrt()
    }

    fn render_density(&self, canvas: &mut Canvas, density_data: &DensityData, config: &PlotConfig) -> Result<()> {
        let x_min = density_data.x_values[0];
        let x_max = density_data.x_values[density_data.x_values.len() - 1];
        let max_density = density_data.density_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        if max_density <= 0.0 {
            return Err(anyhow!("Invalid density calculation"));
        }

        let y_max = max_density * 1.1; // Add 10% padding at the top

        canvas.set_ranges((x_min, x_max), (0.0, y_max));
        canvas.draw_axis();

        let color = self.parse_color(&config.color);
        let line_symbol = '·';
        let point_symbol = config.symbol.unwrap_or('●');

        // Plot density curve
        for i in 0..density_data.x_values.len() {
            let x = density_data.x_values[i];
            let y = density_data.density_values[i];
            
            // Plot point
            canvas.plot_point_with_color(x, y, point_symbol, color);
            
            // Connect with line to next point
            if i < density_data.x_values.len() - 1 {
                let next_x = density_data.x_values[i + 1];
                let next_y = density_data.density_values[i + 1];
                canvas.plot_line_with_color(x, y, next_x, next_y, line_symbol, color);
            }
        }

        Ok(())
    }

    fn parse_color(&self, color_str: &Option<String>) -> Option<Color> {
        color_str.as_ref().and_then(|s| {
            match s.to_lowercase().as_str() {
                "red" => Some(Color::Red),
                "green" => Some(Color::Green),
                "blue" => Some(Color::Blue),
                "yellow" => Some(Color::Yellow),
                "magenta" => Some(Color::Magenta),
                "cyan" => Some(Color::Cyan),
                "white" => Some(Color::White),
                "black" => Some(Color::Black),
                _ => None,
            }
        })
    }
}

#[allow(dead_code)]
struct DensityData {
    x_values: Vec<f64>,
    density_values: Vec<f64>,
    bandwidth: f64,
}

/// Violin plot combines density estimation with box plot information
pub struct ViolinPlot {
    bandwidth: Option<f64>,
    show_quartiles: bool,
    show_median: bool,
}

impl ViolinPlot {
    pub fn new() -> Self {
        Self {
            bandwidth: None,
            show_quartiles: true,
            show_median: true,
        }
    }

    pub fn with_bandwidth(mut self, bandwidth: f64) -> Self {
        self.bandwidth = Some(bandwidth);
        self
    }

    pub fn hide_quartiles(mut self) -> Self {
        self.show_quartiles = false;
        self
    }

    pub fn hide_median(mut self) -> Self {
        self.show_median = false;
        self
    }

    pub fn render(&self, data: &DataFrame, config: &PlotConfig) -> Result<String> {
        if data.columns.is_empty() {
            return Err(anyhow!("No data provided for violin plot"));
        }

        let series = &data.columns[0];
        if series.data.is_empty() {
            return Err(anyhow!("Empty data series"));
        }

        if series.data.len() < 3 {
            return Err(anyhow!("Need at least 3 data points for violin plot"));
        }

        let mut canvas = Canvas::with_labels(
            config.width,
            config.height,
            config.title.clone(),
            config.xlabel.clone(),
            config.ylabel.clone(),
        );

        let violin_data = self.calculate_violin_data(&series.data)?;
        self.render_violin(&mut canvas, &violin_data, config)?;

        Ok(canvas.render_colored(config.color.is_some()))
    }

    fn calculate_violin_data(&self, data: &[f64]) -> Result<ViolinData> {
        // Calculate basic statistics
        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let median = self.calculate_median(&sorted_data);
        let q1 = self.calculate_percentile(&sorted_data, 0.25);
        let q3 = self.calculate_percentile(&sorted_data, 0.75);

        // Calculate density
        let density_plot = DensityPlot::new(self.bandwidth)
            .with_resolution(100);
        let density_data = density_plot.calculate_density(data)?;

        Ok(ViolinData {
            density_x: density_data.x_values,
            density_y: density_data.density_values,
            median,
            q1,
            q3,
            min: sorted_data[0],
            max: sorted_data[sorted_data.len() - 1],
        })
    }

    fn calculate_median(&self, sorted_data: &[f64]) -> f64 {
        let n = sorted_data.len();
        if n % 2 == 0 {
            (sorted_data[n / 2 - 1] + sorted_data[n / 2]) / 2.0
        } else {
            sorted_data[n / 2]
        }
    }

    fn calculate_percentile(&self, sorted_data: &[f64], percentile: f64) -> f64 {
        let n = sorted_data.len();
        let index = percentile * (n - 1) as f64;
        let lower_index = index.floor() as usize;
        let upper_index = index.ceil() as usize;
        let weight = index - lower_index as f64;

        if lower_index == upper_index {
            sorted_data[lower_index]
        } else {
            sorted_data[lower_index] * (1.0 - weight) + sorted_data[upper_index] * weight
        }
    }

    fn render_violin(&self, canvas: &mut Canvas, violin_data: &ViolinData, config: &PlotConfig) -> Result<()> {
        let y_min = violin_data.min;
        let y_max = violin_data.max;
        let y_padding = (y_max - y_min) * 0.05;

        // Find max density for scaling
        let max_density = violin_data.density_y.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let density_scale = 0.4; // Scale factor for violin width

        canvas.set_ranges((-1.0, 1.0), (y_min - y_padding, y_max + y_padding));
        canvas.draw_axis();

        let color = self.parse_color(&config.color);

        // Draw violin shape (mirrored density)
        for i in 0..violin_data.density_x.len() {
            let y = violin_data.density_x[i];
            let density = violin_data.density_y[i];
            let width = (density / max_density) * density_scale;

            // Draw both sides of the violin
            canvas.plot_point_with_color(-width, y, '·', color);
            canvas.plot_point_with_color(width, y, '·', color);

            // Fill the violin
            let steps = (width * 20.0) as usize + 1;
            for j in 0..steps {
                let x = -width + (2.0 * width) * (j as f64 / steps.max(1) as f64);
                canvas.plot_point_with_color(x, y, '░', color);
            }
        }

        // Draw quartiles and median
        if self.show_quartiles {
            canvas.plot_line_with_color(-0.2, violin_data.q1, 0.2, violin_data.q1, '─', Some(Color::White));
            canvas.plot_line_with_color(-0.2, violin_data.q3, 0.2, violin_data.q3, '─', Some(Color::White));
        }

        if self.show_median {
            canvas.plot_line_with_color(-0.3, violin_data.median, 0.3, violin_data.median, '━', Some(Color::Yellow));
        }

        Ok(())
    }

    fn parse_color(&self, color_str: &Option<String>) -> Option<Color> {
        color_str.as_ref().and_then(|s| {
            match s.to_lowercase().as_str() {
                "red" => Some(Color::Red),
                "green" => Some(Color::Green),
                "blue" => Some(Color::Blue),
                "yellow" => Some(Color::Yellow),
                "magenta" => Some(Color::Magenta),
                "cyan" => Some(Color::Cyan),
                "white" => Some(Color::White),
                "black" => Some(Color::Black),
                _ => Some(Color::Blue),
            }
        })
    }
}

struct ViolinData {
    density_x: Vec<f64>,
    density_y: Vec<f64>,
    median: f64,
    q1: f64,
    q3: f64,
    min: f64,
    max: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{Series, DataFrame, PlotConfig, DataFormat};

    #[test]
    fn test_gaussian_density_plot() {
        // Generate some sample data
        let data: Vec<f64> = vec![
            1.0, 1.2, 1.5, 1.8, 2.0, 2.2, 2.5, 2.8, 3.0, 3.2,
            3.5, 3.8, 4.0, 4.2, 4.5, 4.8, 5.0, 5.2, 5.5, 5.8
        ];
        
        let series = Series {
            name: "Test".to_string(),
            data,
        };
        let dataframe = DataFrame {
            columns: vec![series],
            headers: None,
        };
        let config = PlotConfig {
            width: 60,
            height: 30,
            title: Some("Density Plot".to_string()),
            xlabel: Some("Values".to_string()),
            ylabel: Some("Density".to_string()),
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: Some("blue".to_string()),
            symbol: Some('●'),
        };

        let density_plot = DensityPlot::auto_bandwidth()
            .with_kernel(KernelType::Gaussian)
            .with_resolution(150);
        
        let result = density_plot.render(&dataframe, &config);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("Density Plot"));
        assert!(!output.is_empty());
    }

    #[test]
    fn test_epanechnikov_kernel() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let series = Series {
            name: "Epanechnikov".to_string(),
            data,
        };
        let dataframe = DataFrame {
            columns: vec![series],
            headers: None,
        };
        let config = PlotConfig {
            width: 50,
            height: 25,
            title: Some("Epanechnikov Kernel".to_string()),
            xlabel: None,
            ylabel: None,
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: Some("green".to_string()),
            symbol: None,
        };

        let density_plot = DensityPlot::with_bandwidth(1.5)
            .with_kernel(KernelType::Epanechnikov);
        
        let result = density_plot.render(&dataframe, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_violin_plot() {
        let data = vec![
            1.0, 1.5, 2.0, 2.2, 2.5, 2.8, 3.0, 3.2, 3.5, 3.8,
            4.0, 4.2, 4.5, 4.8, 5.0, 5.2, 5.5, 6.0, 6.5, 7.0
        ];
        
        let series = Series {
            name: "Violin Data".to_string(),
            data,
        };
        let dataframe = DataFrame {
            columns: vec![series],
            headers: None,
        };
        let config = PlotConfig {
            width: 40,
            height: 30,
            title: Some("Violin Plot".to_string()),
            xlabel: Some("Distribution".to_string()),
            ylabel: Some("Values".to_string()),
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: Some("magenta".to_string()),
            symbol: None,
        };

        let violin_plot = ViolinPlot::new();
        let result = violin_plot.render(&dataframe, &config);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("Violin Plot"));
        assert!(!output.is_empty());
    }

    #[test]
    fn test_insufficient_data_error() {
        let series = Series {
            name: "Too Few Points".to_string(),
            data: vec![1.0], // Only one data point
        };
        let dataframe = DataFrame {
            columns: vec![series],
            headers: None,
        };
        let config = PlotConfig {
            width: 40,
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

        let density_plot = DensityPlot::auto_bandwidth();
        let result = density_plot.render(&dataframe, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_triangular_kernel() {
        let data = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let series = Series {
            name: "Triangular".to_string(),
            data,
        };
        let dataframe = DataFrame {
            columns: vec![series],
            headers: None,
        };
        let config = PlotConfig {
            width: 40,
            height: 20,
            title: Some("Triangular Kernel".to_string()),
            xlabel: None,
            ylabel: None,
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: Some("cyan".to_string()),
            symbol: Some('*'),
        };

        let density_plot = DensityPlot::with_bandwidth(0.8)
            .with_kernel(KernelType::Triangular)
            .with_resolution(80);
        
        let result = density_plot.render(&dataframe, &config);
        assert!(result.is_ok());
    }
}