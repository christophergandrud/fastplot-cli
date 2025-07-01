#![allow(dead_code)]

use crate::data::{DataFrame, PlotConfig};
use crate::plot::Canvas;
use anyhow::{Result, anyhow};
use crossterm::style::Color;

pub struct Histogram {
    bins: Option<usize>,
    bin_width: Option<f64>,
    normalize: bool,
}

impl Histogram {
    pub fn new(bins: Option<usize>) -> Self {
        Self {
            bins,
            bin_width: None,
            normalize: false,
        }
    }

    pub fn with_bin_width(bin_width: f64) -> Self {
        Self {
            bins: None,
            bin_width: Some(bin_width),
            normalize: false,
        }
    }

    pub fn auto_bins() -> Self {
        Self::new(None)
    }

    pub fn with_bins(bins: usize) -> Self {
        Self::new(Some(bins))
    }

    pub fn normalized(mut self) -> Self {
        self.normalize = true;
        self
    }

    pub fn render(&self, data: &DataFrame, config: &PlotConfig) -> Result<String> {
        if data.columns.is_empty() {
            return Err(anyhow!("No data provided for histogram"));
        }

        let series = &data.columns[0];
        if series.data.is_empty() {
            return Err(anyhow!("Empty data series"));
        }

        let mut canvas = Canvas::with_labels(
            config.width,
            config.height,
            config.title.clone(),
            config.xlabel.clone(),
            config.ylabel.clone(),
        );

        let histogram_data = self.calculate_histogram(&series.data)?;
        self.render_histogram(&mut canvas, &histogram_data, config)?;

        if config.color.is_some() {
            Ok(self.render_histogram_with_labels_colored(&canvas, &histogram_data, config))
        } else {
            Ok(self.render_histogram_with_labels(&canvas, &histogram_data, config))
        }
    }

    fn calculate_histogram(&self, data: &[f64]) -> Result<HistogramData> {
        let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        if (max_val - min_val).abs() < f64::EPSILON {
            return Err(anyhow!("All data values are the same, cannot create histogram"));
        }

        let (bins, bin_width) = if let Some(width) = self.bin_width {
            let num_bins = ((max_val - min_val) / width).ceil() as usize;
            (num_bins, width)
        } else if let Some(num_bins) = self.bins {
            let width = (max_val - min_val) / num_bins as f64;
            (num_bins, width)
        } else {
            // Auto-calculate bins using Sturges' rule
            let num_bins = (1.0 + (data.len() as f64).log2()).ceil() as usize;
            let width = (max_val - min_val) / num_bins as f64;
            (num_bins.max(1), width)
        };

        let mut bin_counts = vec![0; bins];
        let mut bin_edges = Vec::with_capacity(bins + 1);

        // Calculate bin edges
        for i in 0..=bins {
            bin_edges.push(min_val + i as f64 * bin_width);
        }

        // Count data points in each bin
        for &value in data {
            let bin_index = if value >= max_val {
                bins - 1 // Put the maximum value in the last bin
            } else {
                ((value - min_val) / bin_width).floor() as usize
            };
            
            if bin_index < bins {
                bin_counts[bin_index] += 1;
            }
        }

        // Convert counts to frequencies if normalizing
        let bin_values: Vec<f64> = if self.normalize {
            let total_count = data.len() as f64;
            bin_counts.into_iter().map(|count| count as f64 / total_count).collect()
        } else {
            bin_counts.into_iter().map(|count| count as f64).collect()
        };

        Ok(HistogramData {
            bin_edges,
            bin_values,
            bin_width,
        })
    }

    fn render_histogram(&self, canvas: &mut Canvas, hist_data: &HistogramData, config: &PlotConfig) -> Result<()> {
        let max_count = hist_data.bin_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        
        if max_count <= 0.0 {
            return Err(anyhow!("No data to display in histogram"));
        }

        let x_min = hist_data.bin_edges[0];
        let x_max = hist_data.bin_edges[hist_data.bin_edges.len() - 1];
        let y_max = max_count * 1.1; // Add 10% padding at the top

        canvas.set_ranges((x_min, x_max), (0.0, y_max));
        
        // Draw axes with proper tick marks
        let num_bins = hist_data.bin_values.len();
        let y_ticks = 5.min(max_count as usize);
        canvas.draw_axes_with_ticks(num_bins, y_ticks);

        let color = self.parse_color(&config.color);
        let symbol = config.symbol.unwrap_or('█');

        // Draw histogram bars
        for (i, &count) in hist_data.bin_values.iter().enumerate() {
            if count > 0.0 {
                let x_left = hist_data.bin_edges[i];
                let x_right = hist_data.bin_edges[i + 1];
                
                self.draw_histogram_bar(canvas, x_left, x_right, count, symbol, color);
            }
        }

        Ok(())
    }

    fn draw_histogram_bar(&self, canvas: &mut Canvas, x_left: f64, x_right: f64, height: f64, symbol: char, color: Option<Color>) {
        // Use fill_area to create solid rectangular bars
        canvas.fill_area_with_color(x_left, 0.0, x_right, height, symbol, color);
        
        // Draw clear bar edges for definition
        canvas.plot_line_with_color(x_left, 0.0, x_left, height, '│', color);
        canvas.plot_line_with_color(x_right, 0.0, x_right, height, '│', color);
        canvas.plot_line_with_color(x_left, height, x_right, height, '─', color);
    }

    fn render_histogram_with_labels(&self, canvas: &Canvas, hist_data: &HistogramData, _config: &PlotConfig) -> String {
        let mut result = String::new();
        
        // Add title if present
        if let Some(title) = canvas.get_title() {
            let padding = if title.len() < canvas.get_width() {
                (canvas.get_width() - title.len()) / 2
            } else {
                0
            };
            result.push_str(&" ".repeat(padding));
            result.push_str(title);
            result.push('\n');
            result.push('\n');
        }
        
        // Get canvas lines
        let mut canvas_lines: Vec<String> = Vec::new();
        for row in canvas.get_buffer() {
            let line: String = row.iter().collect();
            canvas_lines.push(line);
        }
        
        // Add Y-axis values (integer values for frequency)
        let max_freq = hist_data.bin_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)) as i32;
        let y_range = canvas.get_y_range();
        let y_step = (y_range.1 - y_range.0) / (canvas.get_height() as f64 - 1.0);
        
        let mut last_shown_label = -1i32;
        for (i, line) in canvas_lines.iter().enumerate() {
            let y_value = y_range.1 - (i as f64 * y_step);
            let y_int = y_value.round() as i32;
            
            // Only show the label if it's different from the last one shown and within range
            if y_int >= 0 && y_int <= max_freq && y_int != last_shown_label {
                result.push_str(&format!("{:8} {}\n", y_int, line));
                last_shown_label = y_int;
            } else {
                result.push_str(&format!("{:8} {}\n", " ", line));
            }
        }
        
        // Add X-axis line
        result.push_str("         ");
        for _ in 0..canvas.get_width() {
            result.push('─');
        }
        result.push('\n');
        
        // Add X-axis labels (bin centers)
        result.push_str("         ");
        for i in 0..hist_data.bin_values.len() {
            let bin_center = (hist_data.bin_edges[i] + hist_data.bin_edges[i + 1]) / 2.0;
            let label = format!("{:.0}", bin_center);
            let bin_width = canvas.get_width() / hist_data.bin_values.len();
            let padding = bin_width.saturating_sub(label.len()) / 2;
            result.push_str(&" ".repeat(padding));
            result.push_str(&label);
            result.push_str(&" ".repeat(bin_width - padding - label.len()));
        }
        result.push('\n');
        
        // Add x-label if present
        if let Some(xlabel) = canvas.get_xlabel() {
            let padding = if xlabel.len() < canvas.get_width() {
                (canvas.get_width() - xlabel.len()) / 2
            } else {
                0
            };
            result.push_str(&" ".repeat(padding + 9)); // +9 for Y-axis space
            result.push_str(xlabel);
            result.push('\n');
        }
        
        result
    }

    fn render_histogram_with_labels_colored(&self, canvas: &Canvas, hist_data: &HistogramData, _config: &PlotConfig) -> String {
        let mut result = String::new();
        
        // Add title if present
        if let Some(title) = canvas.get_title() {
            let padding = if title.len() < canvas.get_width() {
                (canvas.get_width() - title.len()) / 2
            } else {
                0
            };
            result.push_str(&" ".repeat(padding));
            result.push_str(&format!("\x1b[1m{}\x1b[0m", title)); // Bold title
            result.push('\n');
            result.push('\n');
        }
        
        // Get canvas lines with colors
        let canvas_colored = canvas.render_colored(true);
        let canvas_lines: Vec<String> = canvas_colored.lines().skip(if canvas.get_title().is_some() { 2 } else { 0 }).map(|s| s.to_string()).collect();
        
        // Add Y-axis values (integer values for frequency)
        let max_freq = hist_data.bin_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)) as i32;
        let y_range = canvas.get_y_range();
        let y_step = (y_range.1 - y_range.0) / (canvas.get_height() as f64 - 1.0);
        
        let mut last_shown_label = -1i32;
        for (i, line) in canvas_lines.iter().enumerate() {
            let y_value = y_range.1 - (i as f64 * y_step);
            let y_int = y_value.round() as i32;
            
            // Only show the label if it's different from the last one shown and within range
            if y_int >= 0 && y_int <= max_freq && y_int != last_shown_label {
                result.push_str(&format!("{:8} {}\n", y_int, line));
                last_shown_label = y_int;
            } else {
                result.push_str(&format!("{:8} {}\n", " ", line));
            }
        }
        
        // Add X-axis line
        result.push_str("         ");
        for _ in 0..canvas.get_width() {
            result.push('─');
        }
        result.push('\n');
        
        // Add X-axis labels (bin centers)
        result.push_str("         ");
        for i in 0..hist_data.bin_values.len() {
            let bin_center = (hist_data.bin_edges[i] + hist_data.bin_edges[i + 1]) / 2.0;
            let label = format!("{:.0}", bin_center);
            let bin_width = canvas.get_width() / hist_data.bin_values.len();
            let padding = bin_width.saturating_sub(label.len()) / 2;
            result.push_str(&" ".repeat(padding));
            result.push_str(&label);
            result.push_str(&" ".repeat(bin_width - padding - label.len()));
        }
        result.push('\n');
        
        // Add x-label if present
        if let Some(xlabel) = canvas.get_xlabel() {
            let padding = if xlabel.len() < canvas.get_width() {
                (canvas.get_width() - xlabel.len()) / 2
            } else {
                0
            };
            result.push_str(&" ".repeat(padding + 9)); // +9 for Y-axis space
            result.push_str(xlabel);
            result.push('\n');
        }
        
        result
    }

    fn parse_color(&self, color_str: &Option<String>) -> Option<Color> {
        color_str.as_ref().and_then(|s| {
            // Try hex color first
            if s.starts_with('#') && s.len() == 7 {
                if let Ok(hex_value) = u32::from_str_radix(&s[1..], 16) {
                    let r = ((hex_value >> 16) & 0xFF) as u8;
                    let g = ((hex_value >> 8) & 0xFF) as u8;
                    let b = (hex_value & 0xFF) as u8;
                    return Some(Color::Rgb { r, g, b });
                }
            }
            
            // Fall back to named colors
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

struct HistogramData {
    bin_edges: Vec<f64>,
    bin_values: Vec<f64>,
    bin_width: f64,
}

/// Cumulative histogram for displaying cumulative distributions
pub struct CumulativeHistogram {
    bins: Option<usize>,
    normalize: bool,
}

impl CumulativeHistogram {
    pub fn new(bins: Option<usize>) -> Self {
        Self { bins, normalize: false }
    }

    pub fn normalized(mut self) -> Self {
        self.normalize = true;
        self
    }

    pub fn render(&self, data: &DataFrame, config: &PlotConfig) -> Result<String> {
        if data.columns.is_empty() {
            return Err(anyhow!("No data provided for cumulative histogram"));
        }

        let series = &data.columns[0];
        if series.data.is_empty() {
            return Err(anyhow!("Empty data series"));
        }

        let mut canvas = Canvas::with_labels(
            config.width,
            config.height,
            config.title.clone(),
            config.xlabel.clone(),
            config.ylabel.clone(),
        );

        let cumulative_data = self.calculate_cumulative_histogram(&series.data)?;
        self.render_cumulative(&mut canvas, &cumulative_data, config)?;

        Ok(canvas.render_colored(config.color.is_some()))
    }

    fn calculate_cumulative_histogram(&self, data: &[f64]) -> Result<HistogramData> {
        let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        if (max_val - min_val).abs() < f64::EPSILON {
            return Err(anyhow!("All data values are the same"));
        }

        let bins = self.bins.unwrap_or_else(|| {
            (1.0 + (data.len() as f64).log2()).ceil() as usize
        });

        let bin_width = (max_val - min_val) / bins as f64;
        let mut bin_counts = vec![0; bins];
        let mut bin_edges = Vec::with_capacity(bins + 1);

        // Calculate bin edges
        for i in 0..=bins {
            bin_edges.push(min_val + i as f64 * bin_width);
        }

        // Count data points in each bin
        for &value in data {
            let bin_index = if value >= max_val {
                bins - 1
            } else {
                ((value - min_val) / bin_width).floor() as usize
            };
            
            if bin_index < bins {
                bin_counts[bin_index] += 1;
            }
        }

        // Calculate cumulative counts
        let mut cumulative_counts = Vec::with_capacity(bins);
        let mut running_total = 0;
        
        for count in bin_counts {
            running_total += count;
            cumulative_counts.push(running_total);
        }

        // Convert to frequencies if normalizing
        let bin_values: Vec<f64> = if self.normalize {
            let total_count = data.len() as f64;
            cumulative_counts.into_iter().map(|count| count as f64 / total_count).collect()
        } else {
            cumulative_counts.into_iter().map(|count| count as f64).collect()
        };

        Ok(HistogramData {
            bin_edges,
            bin_values,
            bin_width,
        })
    }

    fn render_cumulative(&self, canvas: &mut Canvas, hist_data: &HistogramData, config: &PlotConfig) -> Result<()> {
        let max_count = hist_data.bin_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        
        let x_min = hist_data.bin_edges[0];
        let x_max = hist_data.bin_edges[hist_data.bin_edges.len() - 1];
        let y_max = max_count * 1.1;

        canvas.set_ranges((x_min, x_max), (0.0, y_max));
        canvas.draw_axis();

        let color = if let Some(ref color_str) = config.color {
            match color_str.to_lowercase().as_str() {
                "red" => Some(Color::Red),
                "green" => Some(Color::Green),
                "blue" => Some(Color::Blue),
                "yellow" => Some(Color::Yellow),
                "magenta" => Some(Color::Magenta),
                "cyan" => Some(Color::Cyan),
                _ => Some(Color::Blue),
            }
        } else {
            Some(Color::Blue)
        };

        // Draw cumulative line
        for i in 0..hist_data.bin_values.len() {
            let x = hist_data.bin_edges[i] + hist_data.bin_width / 2.0; // Center of bin
            let y = hist_data.bin_values[i];
            
            canvas.plot_point_with_color(x, y, '●', color);
            
            // Connect with line to next point
            if i < hist_data.bin_values.len() - 1 {
                let next_x = hist_data.bin_edges[i + 1] + hist_data.bin_width / 2.0;
                let next_y = hist_data.bin_values[i + 1];
                canvas.plot_line_with_color(x, y, next_x, next_y, '·', color);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{Series, DataFrame, PlotConfig, DataFormat};

    #[test]
    fn test_basic_histogram() {
        let series = Series {
            name: "Test".to_string(),
            data: vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 5.0],
        };
        let dataframe = DataFrame {
            columns: vec![series],
            headers: None,
        };
        let config = PlotConfig {
            width: 50,
            height: 25,
            title: Some("Histogram".to_string()),
            xlabel: Some("Values".to_string()),
            ylabel: Some("Frequency".to_string()),
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: Some("green".to_string()),
            symbol: Some('█'),
        };

        let histogram = Histogram::with_bins(5);
        let result = histogram.render(&dataframe, &config);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("Histogram"));
        assert!(!output.is_empty());
    }

    #[test]
    fn test_auto_bins_histogram() {
        let series = Series {
            name: "Auto Bins".to_string(),
            data: (1..=100).map(|x| x as f64 / 10.0).collect(),
        };
        let dataframe = DataFrame {
            columns: vec![series],
            headers: None,
        };
        let config = PlotConfig {
            width: 60,
            height: 30,
            title: Some("Auto Bins Histogram".to_string()),
            xlabel: None,
            ylabel: None,
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: Some("blue".to_string()),
            symbol: None,
        };

        let histogram = Histogram::auto_bins();
        let result = histogram.render(&dataframe, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_normalized_histogram() {
        let series = Series {
            name: "Normalized".to_string(),
            data: vec![1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0, 4.0],
        };
        let dataframe = DataFrame {
            columns: vec![series],
            headers: None,
        };
        let config = PlotConfig {
            width: 40,
            height: 20,
            title: Some("Normalized Histogram".to_string()),
            xlabel: None,
            ylabel: Some("Density".to_string()),
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: Some("magenta".to_string()),
            symbol: Some('▓'),
        };

        let histogram = Histogram::with_bins(4).normalized();
        let result = histogram.render(&dataframe, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cumulative_histogram() {
        let series = Series {
            name: "Cumulative".to_string(),
            data: vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 5.0],
        };
        let dataframe = DataFrame {
            columns: vec![series],
            headers: None,
        };
        let config = PlotConfig {
            width: 50,
            height: 25,
            title: Some("Cumulative Histogram".to_string()),
            xlabel: Some("Values".to_string()),
            ylabel: Some("Cumulative Count".to_string()),
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: Some("cyan".to_string()),
            symbol: None,
        };

        let cumulative = CumulativeHistogram::new(Some(6));
        let result = cumulative.render(&dataframe, &config);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("Cumulative Histogram"));
        assert!(!output.is_empty());
    }

    #[test]
    fn test_all_same_values_error() {
        let series = Series {
            name: "Same Values".to_string(),
            data: vec![5.0, 5.0, 5.0, 5.0, 5.0],
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

        let histogram = Histogram::auto_bins();
        let result = histogram.render(&dataframe, &config);
        assert!(result.is_err());
    }
}