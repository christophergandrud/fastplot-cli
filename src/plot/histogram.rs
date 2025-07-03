use crate::data::{DataFrame, PlotConfig, Series};
use crate::plot::bar::BarChart;
use crate::plot::DataUtils;
use anyhow::{Result, anyhow};

#[allow(dead_code)] // Public API methods may not all be used in tests

pub struct Histogram {
    bins: Option<usize>,
    bin_width: Option<f64>,
    normalize: bool,
}

#[allow(dead_code)] // Public API methods
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

        // Calculate histogram bins and convert to bar chart data
        let histogram_data = self.calculate_histogram(&series.data)?;
        
        // Create a new DataFrame with the histogram data for the bar chart
        let bar_series = Series {
            name: if self.normalize { "Density".to_string() } else { "Frequency".to_string() },
            data: histogram_data.bin_values.clone(),
        };
        
        let bar_dataframe = DataFrame {
            columns: vec![bar_series],
            headers: None,
        };

        // Create a modified config for the bar chart
        let bar_config = PlotConfig {
            title: config.title.clone(),
            xlabel: config.xlabel.clone(),
            ylabel: if config.ylabel.is_some() { 
                config.ylabel.clone() 
            } else { 
                Some(if self.normalize { "Density".to_string() } else { "Frequency".to_string() })
            },
            ..config.clone()
        };

        // Use the bar chart to render the histogram bars, then customize X-axis
        let bar_chart = BarChart::vertical();
        let mut bar_output = bar_chart.render(&bar_dataframe, &bar_config)?;
        
        // Replace the bar chart's X-axis labels with histogram bin labels
        self.replace_x_axis_labels(&mut bar_output, &histogram_data, &bar_config)
    }
    
    fn replace_x_axis_labels(&self, bar_output: &mut String, hist_data: &HistogramData, config: &PlotConfig) -> Result<String> {
        let lines: Vec<&str> = bar_output.lines().collect();
        let mut result = String::new();
        
        // Copy all lines except the last one (which contains the X-axis labels)
        for (i, line) in lines.iter().enumerate() {
            if i == lines.len() - 1 {
                // Replace the last line with histogram bin labels
                result.push_str(&self.generate_histogram_x_axis(hist_data, config));
            } else {
                result.push_str(line);
                result.push('\n');
            }
        }
        
        Ok(result)
    }
    
    fn generate_histogram_x_axis(&self, hist_data: &HistogramData, config: &PlotConfig) -> String {
        let mut result = String::new();
        
        // Generate bin center labels
        let mut bin_labels = Vec::new();
        for i in 0..hist_data.bin_edges.len().saturating_sub(1) {
            let bin_center = (hist_data.bin_edges[i] + hist_data.bin_edges[i + 1]) / 2.0;
            bin_labels.push(format!("{:.1}", bin_center));
        }
        
        // Only show labels if we have a reasonable number of bins
        if bin_labels.len() <= 15 {
            result.push_str("     "); // Y-axis space
            let chart_width = config.width.saturating_sub(5); // Account for Y-axis space
            let mut labels = vec![' '; chart_width];
            
            // Calculate positions for each label (similar to bar chart logic)
            for (i, label) in bin_labels.iter().enumerate() {
                let bar_width = chart_width / bin_labels.len().max(1);
                let label_center = i * bar_width + bar_width / 2;
                let label_start = label_center.saturating_sub(label.len() / 2);
                
                for (j, ch) in label.chars().enumerate() {
                    if label_start + j < chart_width {
                        labels[label_start + j] = ch;
                    }
                }
            }
            
            let labels_str: String = labels.iter().collect();
            result.push_str(&labels_str);
        }
        
        result.push('\n');
        result
    }

    fn calculate_histogram(&self, data: &[f64]) -> Result<HistogramData> {
        // Use shared utilities for range calculation
        if DataUtils::has_constant_values(data) {
            return Err(anyhow!("All data values are the same, cannot create histogram"));
        }
        
        let (min_val, max_val) = DataUtils::calculate_range(data)?;

        let (bins, bin_width) = if let Some(width) = self.bin_width {
            let num_bins = ((max_val - min_val) / width).ceil() as usize;
            (num_bins, width)
        } else if let Some(num_bins) = self.bins {
            let width = (max_val - min_val) / num_bins as f64;
            (num_bins, width)
        } else {
            // Use shared utility for optimal bin calculation
            let num_bins = DataUtils::calculate_optimal_bins(data.len());
            let width = (max_val - min_val) / num_bins as f64;
            (num_bins, width)
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
            bin_values,
            bin_edges,
        })
    }
}

struct HistogramData {
    bin_values: Vec<f64>,
    bin_edges: Vec<f64>,
}

/// Cumulative histogram for displaying cumulative distributions
#[allow(dead_code)] // Public API struct may not be used in all contexts
pub struct CumulativeHistogram {
    bins: Option<usize>,
    normalize: bool,
}

#[allow(dead_code)] // Public API methods
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

        // Calculate cumulative histogram and convert to bar chart data
        let cumulative_data = self.calculate_cumulative_histogram(&series.data)?;
        
        // Create a new DataFrame with the cumulative histogram data for the bar chart
        let bar_series = Series {
            name: if self.normalize { "Cumulative Density".to_string() } else { "Cumulative Frequency".to_string() },
            data: cumulative_data.bin_values,
        };
        
        let bar_dataframe = DataFrame {
            columns: vec![bar_series],
            headers: None,
        };

        // Create a modified config for the bar chart
        let bar_config = PlotConfig {
            title: config.title.clone(),
            xlabel: config.xlabel.clone(),
            ylabel: if config.ylabel.is_some() { 
                config.ylabel.clone() 
            } else { 
                Some(if self.normalize { "Cumulative Density".to_string() } else { "Cumulative Frequency".to_string() })
            },
            ..config.clone()
        };

        // Use the bar chart to render the cumulative histogram
        let bar_chart = BarChart::vertical();
        bar_chart.render(&bar_dataframe, &bar_config)
    }

    fn calculate_cumulative_histogram(&self, data: &[f64]) -> Result<HistogramData> {
        let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        if (max_val - min_val).abs() < f64::EPSILON {
            return Err(anyhow!("All data values are the same"));
        }

        let bins = self.bins.unwrap_or_else(|| {
            DataUtils::calculate_optimal_bins(data.len())
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
            bin_values,
            bin_edges,
        })
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