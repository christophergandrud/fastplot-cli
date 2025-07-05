/// Advanced axis tick placement using Wilkinson's Extended Algorithm
/// 
/// This module implements sophisticated tick placement algorithms for optimal
/// axis labeling in terminal-based plots, prioritizing human readability.


/// Configuration for axis tick generation
#[derive(Debug, Clone)]
pub struct AxisTickConfig {
    /// Target number of ticks (will be adjusted for optimal spacing)
    pub target_ticks: usize,
    /// Available space in characters/pixels
    pub available_space: usize,
    /// Minimum spacing between tick labels in characters (reserved for future use)
    #[allow(dead_code)]
    pub min_label_spacing: usize,
    /// Whether to prefer round numbers over exact data coverage (reserved for future use)
    #[allow(dead_code)]
    pub prefer_round_numbers: bool,
    /// Maximum number of decimal places to show
    pub max_decimal_places: usize,
}

impl Default for AxisTickConfig {
    fn default() -> Self {
        Self {
            target_ticks: 6,
            available_space: 50,
            min_label_spacing: 8,
            prefer_round_numbers: true,
            max_decimal_places: 3,
        }
    }
}

/// A single axis tick with its value, position, and formatting
#[derive(Debug, Clone)]
pub struct AxisTick {
    /// The numeric value at this tick
    pub value: f64,
    /// The formatted string representation
    pub label: String,
    /// Position along the axis (0.0 to 1.0) (reserved for future use)
    #[allow(dead_code)]
    pub position: f64,
    /// Whether this is a major tick (gets a label) or minor tick (reserved for future use)
    #[allow(dead_code)]
    pub is_major: bool,
    /// Physical position in characters/pixels
    pub pixel_position: usize,
}

/// Result of tick generation with metadata
#[derive(Debug)]
pub struct AxisTickResult {
    /// Generated ticks
    pub ticks: Vec<AxisTick>,
    /// The actual range covered by ticks (may extend beyond data) (reserved for future use)
    #[allow(dead_code)]
    pub tick_range: (f64, f64),
    /// The step size between major ticks (reserved for future use)
    #[allow(dead_code)]
    pub step_size: f64,
    /// Number of decimal places used in formatting (reserved for future use)
    #[allow(dead_code)]
    pub decimal_places: usize,
}

/// Main axis tick generator implementing Wilkinson's Extended Algorithm
pub struct AxisTickGenerator {
    config: AxisTickConfig,
}

impl AxisTickGenerator {
    pub fn new(config: AxisTickConfig) -> Self {
        Self { config }
    }

    /// Generate optimal ticks for the given data range
    pub fn generate_ticks(&self, data_min: f64, data_max: f64) -> AxisTickResult {
        if (data_max - data_min).abs() < f64::EPSILON {
            return self.handle_constant_data(data_min);
        }

        // Use Wilkinson's Extended Algorithm for optimal tick placement
        let best_config = self.find_optimal_tick_configuration(data_min, data_max);
        self.generate_ticks_from_config(best_config, data_min, data_max)
    }

    /// Find the optimal tick configuration using Wilkinson's scoring system
    fn find_optimal_tick_configuration(&self, data_min: f64, data_max: f64) -> TickConfiguration {
        let data_range = data_max - data_min;
        let mut best_config = None;
        let mut best_score = f64::NEG_INFINITY;

        // Test different numbers of ticks around our target
        let tick_range = (self.config.target_ticks.saturating_sub(2))..=(self.config.target_ticks + 3);
        
        for num_ticks in tick_range {
            if num_ticks < 2 { continue; }

            // Calculate initial step size
            let raw_step = data_range / (num_ticks - 1) as f64;
            
            // Test different nice step sizes around the raw step
            for step in self.generate_nice_steps(raw_step) {
                let config = self.create_tick_configuration(step, data_min, data_max, num_ticks);
                let score = self.score_configuration(&config, data_min, data_max);
                
                if score > best_score {
                    best_score = score;
                    best_config = Some(config);
                }
            }
        }

        best_config.unwrap_or_else(|| {
            // Fallback: simple linear spacing
            let step = data_range / (self.config.target_ticks - 1) as f64;
            TickConfiguration {
                step_size: step,
                start_value: data_min,
                num_ticks: self.config.target_ticks,
                decimal_places: self.calculate_decimal_places(step),
            }
        })
    }

    /// Generate a series of "nice" step sizes around a base value
    fn generate_nice_steps(&self, base_step: f64) -> Vec<f64> {
        let magnitude = 10_f64.powf(base_step.log10().floor());
        let _normalized = base_step / magnitude;
        
        let nice_values = [1.0, 2.0, 5.0, 10.0];
        let mut steps = Vec::new();
        
        // Test nice values at the current magnitude and adjacent magnitudes
        for &nice in &nice_values {
            steps.push(nice * magnitude);
            steps.push(nice * magnitude * 0.1);
            steps.push(nice * magnitude * 10.0);
        }
        
        // Also include the exact step for comparison
        steps.push(base_step);
        steps.sort_by(|a, b| a.partial_cmp(b).unwrap());
        steps.dedup_by(|a, b| (*a - *b).abs() < f64::EPSILON);
        
        steps
    }

    /// Create a tick configuration for a given step size
    fn create_tick_configuration(&self, step: f64, data_min: f64, data_max: f64, _target_ticks: usize) -> TickConfiguration {
        // Find the start value (first tick) - round down to nice boundary
        let start_value = (data_min / step).floor() * step;
        
        // Calculate how many ticks we'll actually need
        let end_value = (data_max / step).ceil() * step;
        let num_ticks = ((end_value - start_value) / step).round() as usize + 1;
        
        TickConfiguration {
            step_size: step,
            start_value,
            num_ticks,
            decimal_places: self.calculate_decimal_places(step),
        }
    }

    /// Score a tick configuration using Wilkinson's criteria
    fn score_configuration(&self, config: &TickConfiguration, data_min: f64, data_max: f64) -> f64 {
        let mut score = 0.0;
        
        // Reduced weight - terminal doesn't need "pretty" numbers as much
        score += self.simplicity_score(config.step_size) * 0.3;
        
        // High weight - don't waste vertical space in terminal
        score += self.coverage_score(config, data_min, data_max) * 3.0;
        
        // Keep other weights normal
        score += self.density_score(config) * 1.0;
        score += self.formatting_score(config.decimal_places) * 1.0;
        score += self.granularity_score(config.step_size) * 0.2;
        
        score
    }

    /// Calculate simplicity score based on step size niceness
    fn simplicity_score(&self, step: f64) -> f64 {
        let magnitude = 10_f64.powf(step.log10().floor());
        let normalized = step / magnitude;
        
        // Prefer 1, 2, 5, 10 in that order
        if (normalized - 1.0).abs() < 0.01 { 1.0 }
        else if (normalized - 2.0).abs() < 0.01 { 0.8 }
        else if (normalized - 5.0).abs() < 0.01 { 0.6 }
        else if (normalized - 10.0).abs() < 0.01 { 0.4 }
        else { 0.0 }
    }

    /// Calculate coverage score - penalize if ticks don't span data well
    fn coverage_score(&self, config: &TickConfiguration, data_min: f64, data_max: f64) -> f64 {
        let tick_min = config.start_value;
        let tick_max = config.start_value + (config.num_ticks - 1) as f64 * config.step_size;
        
        let coverage = ((tick_max - tick_min) / (data_max - data_min)).min(2.0);
        let overhang = ((tick_min - data_min).abs() + (tick_max - data_max).abs()) / (data_max - data_min);
        
        // Heavily penalize wasted space (changed from 0.5 to 1.5)
        coverage - overhang * 1.5
    }

    /// Calculate density score - prefer target number of ticks
    fn density_score(&self, config: &TickConfiguration) -> f64 {
        let target = self.config.target_ticks as f64;
        let actual = config.num_ticks as f64;
        
        // Strong penalty for being far from target
        let ratio = (actual / target).min(target / actual);
        let distance_penalty = ((actual - target).abs() / target).min(1.0);
        
        // Weight this more heavily to avoid extreme configurations
        (ratio - distance_penalty * 0.5) * 2.0
    }

    /// Calculate formatting score - penalize excessive decimal places
    fn formatting_score(&self, decimal_places: usize) -> f64 {
        match decimal_places {
            0 => 0.5,
            1 => 0.3,
            2 => 0.1,
            _ => -0.2,
        }
    }

    /// Calculate granularity score - prefer familiar magnitudes
    fn granularity_score(&self, step: f64) -> f64 {
        let log_step = step.log10();
        let frac_part = log_step - log_step.floor();
        
        // Prefer steps that are powers of 10 or close to them
        if frac_part < 0.1 || frac_part > 0.9 { 0.2 } else { 0.0 }
    }

    /// Generate actual ticks from a configuration
    fn generate_ticks_from_config(&self, config: TickConfiguration, data_min: f64, data_max: f64) -> AxisTickResult {
        let mut ticks = Vec::new();
        let data_range = data_max - data_min;
        
        for i in 0..config.num_ticks {
            let value = config.start_value + i as f64 * config.step_size;
            
            // Only skip ticks that are extremely far outside the data range
            if value < data_min - data_range * 2.0 || value > data_max + data_range * 2.0 {
                continue;
            }
            
            let position = if data_range > 0.0 {
                (value - data_min) / data_range
            } else {
                0.5
            };
            
            let pixel_position = (position * self.config.available_space as f64) as usize;
            
            let label = self.format_tick_value(value, config.decimal_places);
            
            ticks.push(AxisTick {
                value,
                label,
                position,
                is_major: true, // All generated ticks are major for now
                pixel_position,
            });
        }
        
        let tick_range = if ticks.is_empty() {
            (data_min, data_max)
        } else {
            (ticks[0].value, ticks[ticks.len() - 1].value)
        };
        
        AxisTickResult {
            ticks,
            tick_range,
            step_size: config.step_size,
            decimal_places: config.decimal_places,
        }
    }

    /// Calculate appropriate number of decimal places for a step size
    fn calculate_decimal_places(&self, step: f64) -> usize {
        if step >= 1.0 {
            0
        } else {
            let log_step = -step.log10();
            (log_step.ceil() as usize).min(self.config.max_decimal_places)
        }
    }

    /// Format a tick value with appropriate precision
    fn format_tick_value(&self, value: f64, decimal_places: usize) -> String {
        if decimal_places == 0 {
            format!("{:.0}", value)
        } else {
            format!("{:.prec$}", value, prec = decimal_places)
        }
    }

    /// Handle the special case where all data values are the same
    fn handle_constant_data(&self, value: f64) -> AxisTickResult {
        let range = if value == 0.0 { 2.0 } else { value.abs() * 0.2 };
        let min_val = value - range;
        let max_val = value + range;
        
        self.generate_ticks(min_val, max_val)
    }
}

/// Internal configuration for tick generation
#[derive(Debug, Clone)]
struct TickConfiguration {
    step_size: f64,
    start_value: f64,
    num_ticks: usize,
    decimal_places: usize,
}

/// Convenient functions for common axis tick scenarios
impl AxisTickGenerator {
    /// Create a generator optimized for Y-axis (vertical) layout
    pub fn for_y_axis(height: usize) -> Self {
        let target_ticks = (height / 4).max(3).min(8);
        let config = AxisTickConfig {
            target_ticks,
            available_space: height,
            min_label_spacing: 2,
            prefer_round_numbers: true,
            max_decimal_places: 2,
        };
        Self::new(config)
    }

    /// Create a generator optimized for X-axis (horizontal) layout (reserved for future use)
    #[allow(dead_code)]
    pub fn for_x_axis(width: usize) -> Self {
        let target_ticks = (width / 10).max(3).min(10);
        let config = AxisTickConfig {
            target_ticks,
            available_space: width,
            min_label_spacing: 8,
            prefer_round_numbers: true,
            max_decimal_places: 1,
        };
        Self::new(config)
    }

    /// Create a generator for histogram bins (shows actual bin ranges)
    pub fn for_histogram_bins(bin_edges: &[f64], width: usize) -> AxisTickResult {
        let mut ticks = Vec::new();
        let data_min = bin_edges.first().copied().unwrap_or(0.0);
        let data_max = bin_edges.last().copied().unwrap_or(1.0);
        let data_range = data_max - data_min;
        
        // For histograms, show bin ranges rather than centers
        let max_labels = (width / 12).max(3).min(bin_edges.len().saturating_sub(1));
        let step = if bin_edges.len() > max_labels {
            (bin_edges.len() - 1) / max_labels
        } else {
            1
        };
        
        for i in (0..bin_edges.len().saturating_sub(1)).step_by(step) {
            let bin_start = bin_edges[i];
            let bin_end = bin_edges.get(i + 1).copied().unwrap_or(bin_start);
            let bin_center = (bin_start + bin_end) / 2.0;
            
            let position = if data_range > 0.0 {
                (bin_center - data_min) / data_range
            } else {
                i as f64 / (bin_edges.len() - 1) as f64
            };
            
            let pixel_position = (position * width as f64) as usize;
            
            // Format as center values for simplicity when dealing with small integer ranges
            let bin_width = bin_end - bin_start;
            let label = if bin_width >= 1.0 && bin_start.fract() == 0.0 && bin_end.fract() == 0.0 {
                // For integer bins, show the center value
                format!("{:.0}", bin_center)
            } else if bin_width >= 1.0 {
                // For wide bins with non-integers, show range
                format!("{:.0}-{:.0}", bin_start, bin_end)
            } else {
                // For narrow bins, show range with decimals
                format!("{:.1}-{:.1}", bin_start, bin_end)
            };
            
            ticks.push(AxisTick {
                value: bin_center,
                label,
                position,
                is_major: true,
                pixel_position,
            });
        }
        
        AxisTickResult {
            tick_range: (data_min, data_max),
            step_size: if bin_edges.len() > 1 { bin_edges[1] - bin_edges[0] } else { 1.0 },
            decimal_places: 1,
            ticks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tick_generation() {
        let generator = AxisTickGenerator::for_y_axis(20);
        let result = generator.generate_ticks(0.0, 100.0);
        
        
        assert!(!result.ticks.is_empty());
        assert!(result.ticks.len() >= 3, "Expected at least 3 ticks, got {}", result.ticks.len());
        assert!(result.ticks.len() <= 10);
        
        // Check that ticks are ordered
        for i in 1..result.ticks.len() {
            assert!(result.ticks[i].value >= result.ticks[i-1].value);
        }
    }

    #[test]
    fn test_nice_numbers() {
        let generator = AxisTickGenerator::for_y_axis(20);
        let result = generator.generate_ticks(0.0, 10.0);
        
        // Should prefer round numbers like 0, 2, 4, 6, 8, 10
        // or 0, 5, 10 depending on configuration
        for tick in &result.ticks {
            let remainder = tick.value % result.step_size;
            assert!(remainder.abs() < 1e-10, "Tick {} is not aligned to step {}", tick.value, result.step_size);
        }
    }

    #[test]
    fn test_constant_data_handling() {
        let generator = AxisTickGenerator::for_y_axis(20);
        let result = generator.generate_ticks(5.0, 5.0);
        
        assert!(!result.ticks.is_empty());
        // Should create a reasonable range around the constant value
        assert!(result.tick_range.0 < 5.0);
        assert!(result.tick_range.1 > 5.0);
    }

    #[test]
    fn test_histogram_bin_labels() {
        let bin_edges = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let result = AxisTickGenerator::for_histogram_bins(&bin_edges, 50);
        
        assert!(!result.ticks.is_empty());
        // Should show ranges like "0.0-1.0", "1.0-2.0", etc.
        for tick in &result.ticks {
            assert!(tick.label.contains('-'));
        }
    }

    #[test]
    fn test_decimal_places_calculation() {
        let generator = AxisTickGenerator::for_y_axis(20);
        
        // Test with small numbers
        let result = generator.generate_ticks(0.0, 0.1);
        assert!(result.decimal_places > 0);
        
        // Test with large numbers
        let result = generator.generate_ticks(0.0, 1000.0);
        assert_eq!(result.decimal_places, 0);
    }
}