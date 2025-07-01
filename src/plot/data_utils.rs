use anyhow::{Result, anyhow};

/// Data processing and validation utilities
pub struct DataUtils;

#[allow(dead_code)]
impl DataUtils {
    /// Calculate the range (min, max) of a data series
    pub fn calculate_range(data: &[f64]) -> Result<(f64, f64)> {
        if data.is_empty() {
            return Err(anyhow!("Cannot calculate range of empty data"));
        }

        let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        Ok((min_val, max_val))
    }

    /// Calculate range with padding for better visualization
    pub fn calculate_range_with_padding(data: &[f64], padding_percent: f64) -> Result<(f64, f64)> {
        let (min_val, max_val) = Self::calculate_range(data)?;
        
        if (max_val - min_val).abs() < f64::EPSILON {
            // Handle case where all values are the same
            let padding = min_val.abs() * 0.1 + 1.0; // Add some padding
            Ok((min_val - padding, max_val + padding))
        } else {
            let range = max_val - min_val;
            let padding = range * padding_percent / 100.0;
            Ok((min_val - padding, max_val + padding))
        }
    }

    /// Validate that data is not empty
    pub fn validate_non_empty(data: &[f64]) -> Result<()> {
        if data.is_empty() {
            Err(anyhow!("Empty data series provided"))
        } else {
            Ok(())
        }
    }

    /// Check if all values in the data are the same (within epsilon)
    pub fn has_constant_values(data: &[f64]) -> bool {
        if data.len() <= 1 {
            return true;
        }

        let first = data[0];
        data.iter().all(|&val| (val - first).abs() < f64::EPSILON)
    }

    /// Check if data contains any NaN or infinite values
    pub fn has_invalid_values(data: &[f64]) -> bool {
        data.iter().any(|&val| val.is_nan() || val.is_infinite())
    }

    /// Filter out NaN and infinite values from data
    pub fn filter_valid_values(data: &[f64]) -> Vec<f64> {
        data.iter()
            .filter(|&&val| val.is_finite())
            .copied()
            .collect()
    }

    /// Calculate statistics for a data series
    pub fn calculate_statistics(data: &[f64]) -> Result<DataStatistics> {
        if data.is_empty() {
            return Err(anyhow!("Cannot calculate statistics for empty data"));
        }

        let valid_data = Self::filter_valid_values(data);
        if valid_data.is_empty() {
            return Err(anyhow!("No valid data points found"));
        }

        let (min, max) = Self::calculate_range(&valid_data)?;
        let sum: f64 = valid_data.iter().sum();
        let mean = sum / valid_data.len() as f64;
        
        // Calculate variance and standard deviation
        let variance = valid_data.iter()
            .map(|&val| (val - mean).powi(2))
            .sum::<f64>() / valid_data.len() as f64;
        let std_dev = variance.sqrt();

        Ok(DataStatistics {
            min,
            max,
            mean,
            std_dev,
            count: valid_data.len(),
            has_invalid: data.len() != valid_data.len(),
        })
    }

    /// Normalize data to a specified range
    pub fn normalize_to_range(data: &[f64], target_min: f64, target_max: f64) -> Result<Vec<f64>> {
        let (data_min, data_max) = Self::calculate_range(data)?;
        
        if (data_max - data_min).abs() < f64::EPSILON {
            // All values are the same, return target midpoint
            let midpoint = (target_min + target_max) / 2.0;
            return Ok(vec![midpoint; data.len()]);
        }

        let data_range = data_max - data_min;
        let target_range = target_max - target_min;
        
        Ok(data.iter().map(|&val| {
            let normalized = (val - data_min) / data_range;
            target_min + normalized * target_range
        }).collect())
    }

    /// Create bins for histogram data
    pub fn create_histogram_bins(data: &[f64], num_bins: usize) -> Result<Vec<HistogramBin>> {
        if data.is_empty() {
            return Err(anyhow!("Cannot create histogram bins for empty data"));
        }

        let valid_data = Self::filter_valid_values(data);
        if valid_data.is_empty() {
            return Err(anyhow!("No valid data points for histogram"));
        }

        let (min, max) = Self::calculate_range(&valid_data)?;
        
        if (max - min).abs() < f64::EPSILON {
            // All values are the same
            return Ok(vec![HistogramBin {
                start: min,
                end: max,
                count: valid_data.len(),
            }]);
        }

        let bin_width = (max - min) / num_bins as f64;
        let mut bins = Vec::new();

        for i in 0..num_bins {
            let start = min + i as f64 * bin_width;
            let end = if i == num_bins - 1 { max } else { start + bin_width };
            
            let count = valid_data.iter()
                .filter(|&&val| val >= start && (val < end || (i == num_bins - 1 && val <= end)))
                .count();

            bins.push(HistogramBin { start, end, count });
        }

        Ok(bins)
    }

    /// Calculate appropriate number of bins for histogram using Sturges' rule
    pub fn calculate_optimal_bins(data_len: usize) -> usize {
        if data_len <= 1 {
            return 1;
        }
        
        // Sturges' rule: k = ceil(log2(n) + 1)
        let k = (data_len as f64).log2().ceil() + 1.0;
        k.max(1.0).min(50.0) as usize // Cap at reasonable limits
    }
}

/// Statistical information about a data series
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DataStatistics {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub std_dev: f64,
    pub count: usize,
    pub has_invalid: bool,
}

/// Represents a single bin in a histogram
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HistogramBin {
    pub start: f64,
    pub end: f64,
    pub count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_range() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let (min, max) = DataUtils::calculate_range(&data).unwrap();
        assert_eq!(min, 1.0);
        assert_eq!(max, 5.0);
    }

    #[test]
    fn test_calculate_range_empty() {
        let data: Vec<f64> = vec![];
        assert!(DataUtils::calculate_range(&data).is_err());
    }

    #[test]
    fn test_has_constant_values() {
        assert!(DataUtils::has_constant_values(&[1.0, 1.0, 1.0]));
        assert!(!DataUtils::has_constant_values(&[1.0, 2.0, 1.0]));
        assert!(DataUtils::has_constant_values(&[1.0])); // Single value
        assert!(DataUtils::has_constant_values(&[])); // Empty
    }

    #[test]
    fn test_has_invalid_values() {
        assert!(DataUtils::has_invalid_values(&[1.0, f64::NAN, 3.0]));
        assert!(DataUtils::has_invalid_values(&[1.0, f64::INFINITY, 3.0]));
        assert!(!DataUtils::has_invalid_values(&[1.0, 2.0, 3.0]));
    }

    #[test]
    fn test_filter_valid_values() {
        let data = vec![1.0, f64::NAN, 3.0, f64::INFINITY, 5.0];
        let valid = DataUtils::filter_valid_values(&data);
        assert_eq!(valid, vec![1.0, 3.0, 5.0]);
    }

    #[test]
    fn test_calculate_range_with_padding() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let (min, max) = DataUtils::calculate_range_with_padding(&data, 10.0).unwrap();
        assert!(min < 1.0);
        assert!(max > 5.0);
        assert_eq!(max - min, (5.0 - 1.0) * 1.2); // 10% padding on each side
    }

    #[test]
    fn test_normalize_to_range() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let normalized = DataUtils::normalize_to_range(&data, 0.0, 10.0).unwrap();
        assert_eq!(normalized[0], 0.0);
        assert_eq!(normalized[4], 10.0);
        assert_eq!(normalized[2], 5.0); // Middle value
    }

    #[test]
    fn test_calculate_optimal_bins() {
        assert_eq!(DataUtils::calculate_optimal_bins(1), 1);
        assert_eq!(DataUtils::calculate_optimal_bins(10), 5); // log2(10) + 1 ≈ 4.32 + 1 = 5.32 → 5
        assert_eq!(DataUtils::calculate_optimal_bins(100), 8); // log2(100) + 1 ≈ 6.64 + 1 = 7.64 → 8
    }

    #[test]
    fn test_create_histogram_bins() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let bins = DataUtils::create_histogram_bins(&data, 3).unwrap();
        assert_eq!(bins.len(), 3);
        
        // Check that all data points are accounted for
        let total_count: usize = bins.iter().map(|b| b.count).sum();
        assert_eq!(total_count, data.len());
    }

    #[test]
    fn test_calculate_statistics() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = DataUtils::calculate_statistics(&data).unwrap();
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
        assert_eq!(stats.mean, 3.0);
        assert_eq!(stats.count, 5);
        assert!(!stats.has_invalid);
    }
}