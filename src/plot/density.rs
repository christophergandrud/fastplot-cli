#![allow(dead_code)]

use crate::data::{DataFrame, PlotConfig};
use crate::plot::{Canvas, ColorUtils};
use anyhow::{Result, anyhow};
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

        let color = ColorUtils::parse_color(&config.color);
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

    // Color parsing method removed - now using shared ColorUtils
}

#[allow(dead_code)]
struct DensityData {
    x_values: Vec<f64>,
    density_values: Vec<f64>,
    bandwidth: f64,
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