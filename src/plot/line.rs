use crate::data::{DataFrame, PlotConfig};
use crate::plot::{Canvas, ColorUtils, DataUtils, RenderUtils};
use crate::plot::AxisRenderer;
use anyhow::{Result, anyhow};

pub struct LinePlot {
    multi_series: bool,
}

impl LinePlot {
    pub fn new(multi_series: bool) -> Self {
        Self { multi_series }
    }

    pub fn single() -> Self {
        Self::new(false)
    }

    pub fn multi() -> Self {
        Self::new(true)
    }

    pub fn render(&self, data: &DataFrame, config: &PlotConfig) -> Result<String> {
        if data.columns.is_empty() {
            return Err(anyhow!("No data provided for line plot"));
        }

        if self.multi_series {
            self.render_multi_series_ascii(data, config)
        } else {
            self.render_single_series_ascii(&data.columns[0], config)
        }
    }

    fn render_single_series_ascii(&self, series: &crate::data::Series, config: &PlotConfig) -> Result<String> {
        let data = &series.data;
        if data.is_empty() {
            return Err(anyhow!("Empty data series"));
        }

        let symbol = config.symbol.unwrap_or('●');
        
        // Validate and analyze data using shared utilities
        RenderUtils::validate_plot_data(data, "line plot")?;
        
        if DataUtils::has_constant_values(data) {
            return RenderUtils::handle_constant_values(data, config);
        }

        // Calculate data range with utilities
        let (min_val, max_val) = DataUtils::calculate_range(data)?;

        // Use Canvas for consistent axis rendering like scatter plot
        let mut canvas = Canvas::with_labels(
            config.width,
            config.height,
            config.title.clone(),
            config.xlabel.clone(),
            config.ylabel.clone(),
        );
        
        // Add some padding to the Y range
        let y_range = max_val - min_val;
        let padding = y_range * 0.1;
        let y_min = min_val - padding;
        let y_max = max_val + padding;
        
        // Set the canvas ranges and draw axes
        canvas.set_ranges((0.0, (data.len() - 1) as f64), (y_min, y_max));
        canvas.draw_axis();

        let color = ColorUtils::parse_color(&config.color);
        
        // Plot data points and connect with lines
        for (i, &value) in data.iter().enumerate() {
            let x = i as f64;
            
            // Plot the point
            canvas.plot_point_with_color(x, value, symbol, color);
            
            // Draw line to next point
            if i < data.len() - 1 {
                let next_value = data[i + 1];
                let next_x = (i + 1) as f64;
                
                // Use Canvas line drawing for consistency
                canvas.plot_line_with_color(x, value, next_x, next_value, '∙', color);
            }
        }
        
        // Get canvas output and add Y-axis labels using unified system
        let canvas_output = canvas.render_colored(config.color.is_some());
        let axis_renderer = AxisRenderer::new(canvas.get_width());
        
        // Add Y-axis labels using the same system as scatter plot
        let output_with_y_labels = axis_renderer.render_y_axis_labels(
            &canvas_output, 
            y_min, 
            y_max, 
            canvas.get_height()
        );
        
        // Add X-axis labels
        let x_labels = axis_renderer.generate_numeric_labels(data.len(), 5);
        let final_output = format!("{}{}", output_with_y_labels, axis_renderer.render_x_axis_labels(&x_labels));
        
        Ok(final_output)
    }





    fn render_multi_series_ascii(&self, data: &DataFrame, config: &PlotConfig) -> Result<String> {
        // For now, just render the first series. Multi-series can be enhanced later
        if data.columns.is_empty() {
            return Err(anyhow!("No data series provided"));
        }
        self.render_single_series_ascii(&data.columns[0], config)
    }

    // Color parsing methods removed - now using shared ColorUtils

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{Series, DataFrame, PlotConfig, DataFormat};

    #[test]
    fn test_single_line_plot() {
        let series = Series {
            name: "Test".to_string(),
            data: vec![1.0, 3.0, 2.0, 5.0, 4.0, 2.5],
        };
        let dataframe = DataFrame {
            columns: vec![series],
            headers: None,
        };
        let config = PlotConfig {
            width: 50,
            height: 25,
            title: Some("Line Chart".to_string()),
            xlabel: Some("X Axis".to_string()),
            ylabel: Some("Y Axis".to_string()),
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: Some("blue".to_string()),
            symbol: Some('●'),
        };

        let line_plot = LinePlot::single();
        let result = line_plot.render(&dataframe, &config);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("Line Chart"));
        assert!(!output.is_empty());
    }

    #[test]
    fn test_multi_series_line_plot() {
        let series1 = Series {
            name: "Series 1".to_string(),
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        };
        let series2 = Series {
            name: "Series 2".to_string(),
            data: vec![5.0, 4.0, 3.0, 2.0, 1.0],
        };
        let series3 = Series {
            name: "Series 3".to_string(),
            data: vec![2.5, 3.5, 2.0, 4.5, 3.0],
        };
        
        let dataframe = DataFrame {
            columns: vec![series1, series2, series3],
            headers: None,
        };
        
        let config = PlotConfig {
            width: 60,
            height: 30,
            title: Some("Multi-Series Plot".to_string()),
            xlabel: Some("Time".to_string()),
            ylabel: Some("Value".to_string()),
            delimiter: ',',
            has_header: false,
            format: DataFormat::XYY,
            xlim: None,
            ylim: None,
            color: None,
            symbol: None,
        };

        let line_plot = LinePlot::multi();
        let result = line_plot.render(&dataframe, &config);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("Multi-Series Plot"));
        assert!(!output.is_empty());
    }

    #[test]
    fn test_empty_data_error() {
        let dataframe = DataFrame {
            columns: vec![],
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

        let line_plot = LinePlot::single();
        let result = line_plot.render(&dataframe, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_single_point_line() {
        let series = Series {
            name: "Single Point".to_string(),
            data: vec![42.0],
        };
        let dataframe = DataFrame {
            columns: vec![series],
            headers: None,
        };
        let config = PlotConfig {
            width: 40,
            height: 20,
            title: Some("Single Point".to_string()),
            xlabel: None,
            ylabel: None,
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: Some("red".to_string()),
            symbol: Some('●'),
        };

        let line_plot = LinePlot::single();
        let result = line_plot.render(&dataframe, &config);
        assert!(result.is_ok());
    }
}