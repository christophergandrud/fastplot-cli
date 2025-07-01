/// Axis label generation and positioning utilities
pub struct AxisUtils;

#[allow(dead_code)]
impl AxisUtils {
    /// Generate Y-axis labels with proper formatting and spacing
    pub fn create_y_labels(min: f64, max: f64, height: usize, precision: Option<usize>) -> Vec<(f64, String)> {
        let precision = precision.unwrap_or(1);
        let num_labels = (height / 4).max(3).min(10); // Reasonable number of labels
        let mut labels = Vec::new();
        
        for i in 0..num_labels {
            let ratio = i as f64 / (num_labels - 1) as f64;
            let value = min + (max - min) * ratio;
            let label = if precision == 0 {
                format!("{:>6.0}", value)
            } else {
                format!("{:>6.*}", precision, value)
            };
            labels.push((value, label));
        }
        
        labels
    }

    /// Generate X-axis labels for bar charts and histograms
    pub fn create_x_labels_for_bars(count: usize, _width: usize, max_labels: Option<usize>) -> Vec<(usize, String)> {
        let max_labels = max_labels.unwrap_or(10);
        if count <= max_labels {
            // Show all indices if we have few enough bars
            (0..count).map(|i| (i, i.to_string())).collect()
        } else {
            // Show evenly spaced labels
            let step = count / max_labels;
            (0..max_labels)
                .map(|i| {
                    let index = i * step;
                    (index, index.to_string())
                })
                .collect()
        }
    }

    /// Calculate label positions for X-axis with proper spacing
    pub fn calculate_label_positions(data_len: usize, chart_width: usize, num_labels: usize) -> Vec<(String, usize)> {
        let mut positions = Vec::new();
        
        if num_labels == 0 || data_len == 0 {
            return positions;
        }

        // Calculate step size for label placement
        let step = if num_labels >= data_len {
            1
        } else {
            data_len / num_labels
        };

        let bar_width = chart_width / data_len.max(1);
        
        for i in 0..num_labels {
            let data_index = i * step;
            if data_index < data_len {
                let position = data_index * bar_width + bar_width / 2;
                positions.push((data_index.to_string(), position));
            }
        }
        
        positions
    }

    /// Center text within a given width
    pub fn center_text(text: &str, width: usize) -> String {
        if text.len() >= width {
            return text.to_string();
        }
        
        let padding = width - text.len();
        let left_padding = padding / 2;
        let right_padding = padding - left_padding;
        
        format!("{}{}{}", 
            " ".repeat(left_padding), 
            text, 
            " ".repeat(right_padding)
        )
    }

    /// Format title with centering and optional padding
    pub fn format_title(title: &str, width: usize) -> String {
        format!("{:^width$}\n\n", title, width = width)
    }

    /// Generate tick marks for Y-axis
    pub fn generate_y_ticks(min: f64, max: f64, height: usize) -> Vec<(usize, f64, bool)> {
        let num_ticks = (height / 3).max(3).min(15);
        let mut ticks = Vec::new();
        
        for i in 0..num_ticks {
            let ratio = i as f64 / (num_ticks - 1) as f64;
            let value = max - (max - min) * ratio; // Reverse for top-to-bottom display
            let row = (i * height / (num_ticks - 1)).min(height - 1);
            let is_major = i % 2 == 0; // Every other tick is major
            ticks.push((row, value, is_major));
        }
        
        ticks
    }

    /// Create X-axis ruler line with tick marks
    pub fn create_x_axis_ruler(width: usize, tick_positions: &[usize]) -> String {
        let mut ruler = vec![' '; width];
        
        // Add tick marks at specified positions
        for &pos in tick_positions {
            if pos < width {
                ruler[pos] = '┬';
            }
        }
        
        // Set the baseline
        for i in 0..width {
            if ruler[i] == ' ' {
                ruler[i] = '─';
            }
        }
        
        ruler.into_iter().collect()
    }

    /// Format numeric value with automatic precision
    pub fn format_numeric(value: f64, precision: Option<usize>) -> String {
        let precision = precision.unwrap_or_else(|| {
            // Auto-detect precision based on value magnitude
            if value.abs() >= 1000.0 {
                0
            } else if value.abs() >= 10.0 {
                1
            } else {
                2
            }
        });
        
        format!("{:.*}", precision, value)
    }

    /// Create spacing between chart elements
    pub fn create_spacing(lines: usize) -> String {
        "\n".repeat(lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center_text() {
        assert_eq!(AxisUtils::center_text("test", 10), "   test   ");
        assert_eq!(AxisUtils::center_text("test", 6), " test ");
        assert_eq!(AxisUtils::center_text("test", 4), "test");
        assert_eq!(AxisUtils::center_text("test", 3), "test"); // Longer than width
    }

    #[test]
    fn test_format_title() {
        let result = AxisUtils::format_title("Test Title", 20);
        assert!(result.contains("Test Title"));
        assert!(result.ends_with("\n\n"));
    }

    #[test]
    fn test_create_y_labels() {
        let labels = AxisUtils::create_y_labels(0.0, 100.0, 20, Some(1));
        assert!(!labels.is_empty());
        assert!(labels.len() >= 3);
        assert!(labels.len() <= 10);
        
        // Check that labels are properly ordered
        for i in 1..labels.len() {
            assert!(labels[i].0 >= labels[i-1].0);
        }
    }

    #[test]
    fn test_format_numeric() {
        assert_eq!(AxisUtils::format_numeric(1234.5, Some(0)), "1235");
        assert_eq!(AxisUtils::format_numeric(12.34, Some(1)), "12.3");
        assert_eq!(AxisUtils::format_numeric(1.234, Some(2)), "1.23");
    }

    #[test]
    fn test_generate_y_ticks() {
        let ticks = AxisUtils::generate_y_ticks(0.0, 100.0, 20);
        assert!(!ticks.is_empty());
        
        // Check tick positions are within bounds
        for (row, _value, _is_major) in ticks {
            assert!(row < 20);
        }
    }
}