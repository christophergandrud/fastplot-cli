use crate::data::PlotConfig;
use crate::plot::Canvas;
use crate::plot::color_utils::ColorUtils;
use crate::plot::axis_utils::AxisUtils;
use crate::plot::data_utils::DataUtils;
use anyhow::Result;

/// Rendering utilities for common plot operations
pub struct RenderUtils;

/// Layout constants for consistent spacing across all plot types
impl RenderUtils {
    /// Space to reserve for title (title + blank line)
    pub const TITLE_SPACE: usize = 2;
    /// Space to reserve for X-axis line and labels
    pub const X_AXIS_SPACE: usize = 2;
    /// Space to reserve for Y-axis labels
    pub const Y_AXIS_LABEL_WIDTH: usize = 10;
    /// Minimum padding for chart area
    pub const MIN_PADDING: usize = 1;
    
    /// Calculate total vertical space needed for non-chart elements
    pub const fn total_vertical_overhead() -> usize {
        Self::TITLE_SPACE + Self::X_AXIS_SPACE + Self::MIN_PADDING
    }
}

#[allow(dead_code)]
impl RenderUtils {
    /// Set up a canvas with proper configuration
    pub fn setup_canvas(config: &PlotConfig, x_range: (f64, f64), y_range: (f64, f64)) -> Canvas {
        let mut canvas = Canvas::with_labels(
            config.width,
            config.height,
            config.title.clone(),
            config.xlabel.clone(),
            config.ylabel.clone(),
        );

        canvas.set_ranges(x_range, y_range);
        canvas.draw_axis();
        canvas
    }

    /// Set up a canvas with automatic range calculation
    pub fn setup_canvas_auto_range(
        config: &PlotConfig, 
        x_data: &[f64], 
        y_data: &[f64],
        padding_percent: f64
    ) -> Result<Canvas> {
        let x_range = DataUtils::calculate_range_with_padding(x_data, padding_percent)?;
        let y_range = DataUtils::calculate_range_with_padding(y_data, padding_percent)?;
        
        Ok(Self::setup_canvas(config, x_range, y_range))
    }

    /// Render a complete plot with title, labels, and color support
    pub fn render_complete_plot(canvas: &Canvas, config: &PlotConfig) -> String {
        canvas.render_colored(config.color.is_some())
    }

    /// Create a simple ASCII plot without canvas (for lightweight rendering)
    pub fn create_ascii_plot(
        data: &[f64], 
        config: &PlotConfig, 
        symbol: char,
        renderer: impl Fn(&[f64], &PlotConfig, char) -> Result<String>
    ) -> Result<String> {
        let mut output = String::new();

        // Add title if present
        if let Some(title) = &config.title {
            output.push_str(&AxisUtils::format_title(title, config.width));
        }

        // Render the main plot
        let plot_content = renderer(data, config, symbol)?;
        output.push_str(&plot_content);

        // Apply color if specified
        if let Some(color_str) = &config.color {
            return Ok(ColorUtils::apply_color_string(&output, color_str));
        }

        Ok(output)
    }

    /// Validate common plot requirements
    pub fn validate_plot_data(data: &[f64], _plot_type: &str) -> Result<()> {
        DataUtils::validate_non_empty(data)?;
        
        if DataUtils::has_invalid_values(data) {
            eprintln!("Warning: Data contains NaN or infinite values, they will be filtered out");
        }

        // Check for constant values
        if DataUtils::has_constant_values(data) {
            return Ok(()); // Allow constant values, but handle specially
        }

        Ok(())
    }

    /// Create a standardized error message for plotting issues
    pub fn create_error_message(plot_type: &str, error: &str) -> String {
        format!("Error in {} plot: {}", plot_type, error)
    }

    /// Handle constant value datasets with appropriate visualization
    pub fn handle_constant_values(data: &[f64], config: &PlotConfig) -> Result<String> {
        if data.is_empty() {
            return Ok("No data to display".to_string());
        }

        let value = data[0];
        let mut output = String::new();

        if let Some(title) = &config.title {
            output.push_str(&AxisUtils::format_title(title, config.width));
        }

        output.push_str(&format!(
            "All {} values are constant: {}\n",
            data.len(),
            AxisUtils::format_numeric(value, None)
        ));

        // Simple visualization for constant values
        let bar_width = (config.width as f64 * 0.6) as usize;
        let bar = "█".repeat(bar_width);
        output.push_str(&format!("{:^width$}\n", bar, width = config.width));
        output.push_str(&format!("{:^width$}\n", 
            AxisUtils::format_numeric(value, None), 
            width = config.width
        ));

        Ok(output)
    }

    /// Apply styling and formatting to plot output
    pub fn apply_styling(content: &str, config: &PlotConfig) -> String {
        let mut styled_content = content.to_string();

        // Apply color if specified
        if let Some(color_str) = &config.color {
            styled_content = ColorUtils::apply_color_string(&styled_content, color_str);
        }

        styled_content
    }

    /// Create a plot footer with optional statistics or metadata
    pub fn create_plot_footer(data: &[f64], show_stats: bool) -> Result<String> {
        if !show_stats {
            return Ok(String::new());
        }

        let stats = DataUtils::calculate_statistics(data)?;
        let mut footer = String::new();
        
        footer.push_str(&format!("\nData points: {}", stats.count));
        footer.push_str(&format!(" | Range: {:.2} - {:.2}", stats.min, stats.max));
        footer.push_str(&format!(" | Mean: {:.2}", stats.mean));
        
        if stats.has_invalid {
            footer.push_str(" | Contains invalid values");
        }

        Ok(footer)
    }

    /// Create a legend for multi-series plots
    pub fn create_legend(series_names: &[String], colors: &[Option<String>]) -> String {
        if series_names.is_empty() {
            return String::new();
        }

        let mut legend = String::from("\nLegend:\n");
        
        for (i, name) in series_names.iter().enumerate() {
            let symbol = "■";
            let colored_symbol = colors.get(i)
                .and_then(|c| c.as_ref())
                .map(|color| ColorUtils::apply_color_string(symbol, color))
                .unwrap_or_else(|| symbol.to_string());
            
            legend.push_str(&format!("  {} {}\n", colored_symbol, name));
        }

        legend
    }

    /// Determine appropriate chart dimensions based on data size
    pub fn calculate_optimal_dimensions(data_len: usize, config: &PlotConfig) -> (usize, usize) {
        let width = if config.width > 0 {
            config.width
        } else {
            // Auto-calculate based on data size
            (data_len * 2).min(120).max(40)
        };

        let height = if config.height > 0 {
            config.height
        } else {
            // Auto-calculate based on data size
            (data_len / 2).min(30).max(10)
        };

        (width, height)
    }
}

/// Helper trait for plot types to implement common functionality
#[allow(dead_code)]
pub trait PlotRenderer {
    fn render_with_utils(&self, data: &[f64], config: &PlotConfig) -> Result<String>;
    fn get_plot_type_name(&self) -> &'static str;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::PlotConfig;

    fn create_test_config() -> PlotConfig {
        PlotConfig {
            width: 80,
            height: 20,
            title: Some("Test Plot".to_string()),
            xlabel: None,
            ylabel: None,
            delimiter: ',',
            has_header: false,
            format: crate::data::DataFormat::XY,
            xlim: None,
            ylim: None,
            color: None,
            symbol: None,
        }
    }

    #[test]
    fn test_validate_plot_data() {
        let valid_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert!(RenderUtils::validate_plot_data(&valid_data, "test").is_ok());

        let empty_data: Vec<f64> = vec![];
        assert!(RenderUtils::validate_plot_data(&empty_data, "test").is_err());

        let constant_data = vec![1.0, 1.0, 1.0];
        assert!(RenderUtils::validate_plot_data(&constant_data, "test").is_ok());
    }

    #[test]
    fn test_handle_constant_values() {
        let config = create_test_config();
        let constant_data = vec![5.0, 5.0, 5.0];
        let result = RenderUtils::handle_constant_values(&constant_data, &config).unwrap();
        
        assert!(result.contains("Test Plot"));
        assert!(result.contains("constant: 5"));
        assert!(result.contains("█"));
    }

    #[test]
    fn test_create_plot_footer() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let footer = RenderUtils::create_plot_footer(&data, true).unwrap();
        
        assert!(footer.contains("Data points: 5"));
        assert!(footer.contains("Range: 1.00 - 5.00"));
        assert!(footer.contains("Mean: 3.00"));
    }

    #[test]
    fn test_create_legend() {
        let names = vec!["Series 1".to_string(), "Series 2".to_string()];
        let colors = vec![Some("red".to_string()), Some("blue".to_string())];
        let legend = RenderUtils::create_legend(&names, &colors);
        
        assert!(legend.contains("Legend:"));
        assert!(legend.contains("Series 1"));
        assert!(legend.contains("Series 2"));
    }

    #[test]
    fn test_calculate_optimal_dimensions() {
        let config = PlotConfig {
            width: 0,  // Auto-calculate
            height: 0, // Auto-calculate
            ..create_test_config()
        };
        
        let (width, height) = RenderUtils::calculate_optimal_dimensions(50, &config);
        assert!(width >= 40);
        assert!(width <= 120);
        assert!(height >= 10);
        assert!(height <= 30);
    }

    #[test]
    fn test_apply_styling() {
        let config = PlotConfig {
            color: Some("red".to_string()),
            ..create_test_config()
        };
        
        let content = "Test content";
        let styled = RenderUtils::apply_styling(content, &config);
        
        // The output should contain ANSI color codes
        assert_ne!(styled, content);
    }
}