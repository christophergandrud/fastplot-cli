use crate::data::{DataFrame, PlotConfig};
use crate::plot::Canvas;
use anyhow::{Result, anyhow};
use crossterm::style::{Color, Stylize};

pub struct BarChart {
    horizontal: bool,
}

impl BarChart {
    pub fn new(horizontal: bool) -> Self {
        Self { horizontal }
    }

    pub fn vertical() -> Self {
        Self::new(false)
    }

    pub fn horizontal() -> Self {
        Self::new(true)
    }

    pub fn render(&self, data: &DataFrame, config: &PlotConfig) -> Result<String> {
        if data.columns.is_empty() {
            return Err(anyhow!("No data provided for bar chart"));
        }

        let series = &data.columns[0];
        if series.data.is_empty() {
            return Err(anyhow!("Empty data series"));
        }

        if self.horizontal {
            self.render_horizontal_bars(series, config)
        } else {
            self.render_vertical_bars_ascii(series, config)
        }
    }

    fn render_vertical_bars_ascii(&self, series: &crate::data::Series, config: &PlotConfig) -> Result<String> {
        let data = &series.data;
        let symbol = config.symbol.unwrap_or('█');
        
        // Find min and max values for scaling
        let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b)).min(0.0);
        let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        
        if (max_val - min_val).abs() < f64::EPSILON {
            return Ok(format!("{}\n\nAll values are the same: {}", 
                config.title.as_deref().unwrap_or(""), max_val));
        }

        // Calculate chart dimensions
        let chart_height = config.height.saturating_sub(5); // Reserve space for title and axis
        let chart_width = config.width.saturating_sub(10);  // Reserve space for Y-axis labels
        
        // Create clean Y-axis labels (round numbers)
        let y_range = max_val - min_val;
        let label_step = 2; // Show labels every 2 rows
        
        // Calculate bar positions
        let bar_width = 2; // Each bar is 2 characters wide
        let bar_spacing = 1; // Single space between bars  
        let total_bar_space = bar_width + bar_spacing;
        let num_bars = data.len().min(chart_width / total_bar_space);
        
        let mut output = String::new();
        
        // Add title
        if let Some(title) = &config.title {
            output.push_str(&format!("{:^width$}\n\n", title, width = config.width));
        }

        // Build the chart from top to bottom
        for row in 0..chart_height {
            let is_label_row = row % label_step == 0;
            let is_last_row = row == chart_height - 1;
            
            // Calculate Y value for this row
            let y_value = max_val - (row as f64 / chart_height as f64) * y_range;
            
            // Y-axis label and tick
            if is_label_row && !is_last_row {
                output.push_str(&format!("{:>4.0} ┤", y_value));
            } else if is_last_row {
                output.push_str(&format!("{:>4.0} └", min_val));
            } else {
                output.push_str("     ┤");
            }
            
            // Draw bars for this row
            for (i, &value) in data.iter().enumerate().take(num_bars) {
                if i > 0 {
                    output.push(' '); // Single space between bars
                }
                
                // Calculate if this bar should be filled at this height
                let bar_height_ratio = (value - min_val) / y_range;
                let bar_fill_threshold = 1.0 - (row as f64 / chart_height as f64);
                
                if bar_height_ratio >= bar_fill_threshold {
                    // Fill this part of the bar
                    if let Some(color_name) = &config.color {
                        let colored_symbols = self.apply_color(&format!("{}{}", symbol, symbol), color_name);
                        output.push_str(&colored_symbols);
                    } else {
                        output.push_str(&format!("{}{}", symbol, symbol));
                    }
                } else {
                    // Empty space above the bar
                    output.push_str("  ");
                }
            }
            
            output.push('\n');
        }
        
        // X-axis base line
        output.push_str("     ");
        for i in 0..num_bars {
            if i == 0 {
                output.push('─');
            } else {
                output.push('┴');
            }
            output.push('─');
            if i < num_bars - 1 {
                output.push('─');
            }
        }
        output.push_str("─\n");
        
        // X-axis labels
        if num_bars <= 15 {
            output.push_str("     ");
            for i in 0..num_bars {
                if i == 0 {
                    output.push_str(&format!("{}", i + 1));
                } else {
                    output.push_str(&format!("  {}", i + 1));
                }
            }
            output.push('\n');
        }

        Ok(output)
    }

    fn render_horizontal_bars(&self, series: &crate::data::Series, config: &PlotConfig) -> Result<String> {
        // Keep the existing horizontal bar implementation using canvas
        let mut canvas = Canvas::with_labels(
            config.width,
            config.height,
            config.title.clone(),
            config.xlabel.clone(),
            config.ylabel.clone(),
        );

        let data = &series.data;
        let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        
        let (x_min, x_max) = if (max_val - min_val).abs() < f64::EPSILON {
            (min_val - 1.0, max_val + 1.0)
        } else {
            (min_val.min(0.0), max_val)
        };

        canvas.set_ranges((x_min, x_max), (0.0, data.len() as f64));
        canvas.draw_axis();

        let bar_height = (config.height as f64 / data.len() as f64).max(1.0);
        let color = self.parse_color(&config.color);
        let symbol = config.symbol.unwrap_or('█');

        for (i, &value) in data.iter().enumerate() {
            let y_start = i as f64;
            let y_end = y_start + bar_height.min(1.0);
            
            if value >= 0.0 {
                self.fill_bar(&mut canvas, 0.0, y_start, value, y_end, symbol, color);
            } else {
                self.fill_bar(&mut canvas, value, y_start, 0.0, y_end, symbol, color);
            }
        }

        Ok(canvas.render_colored(config.color.is_some()))
    }

    fn fill_bar(&self, canvas: &mut Canvas, x1: f64, y1: f64, x2: f64, y2: f64, symbol: char, color: Option<Color>) {
        let steps_x = ((x2 - x1) * 10.0) as usize + 1;
        let steps_y = ((y2 - y1) * 10.0) as usize + 1;
        
        for i in 0..steps_x {
            for j in 0..steps_y {
                let x = x1 + (x2 - x1) * (i as f64 / steps_x.max(1) as f64);
                let y = y1 + (y2 - y1) * (j as f64 / steps_y.max(1) as f64);
                
                if canvas.is_point_in_bounds(x, y) {
                    canvas.plot_point_with_color(x, y, symbol, color);
                }
            }
        }
    }

    fn apply_color(&self, text: &str, color_name: &str) -> String {
        if let Some(color) = self.parse_hex_color(color_name) {
            format!("{}", text.with(color))
        } else {
            text.to_string()
        }
    }

    fn parse_hex_color(&self, color_str: &str) -> Option<Color> {
        // Try hex color first
        if color_str.starts_with('#') && color_str.len() == 7 {
            if let Ok(hex_value) = u32::from_str_radix(&color_str[1..], 16) {
                let r = ((hex_value >> 16) & 0xFF) as u8;
                let g = ((hex_value >> 8) & 0xFF) as u8;
                let b = (hex_value & 0xFF) as u8;
                return Some(Color::Rgb { r, g, b });
            }
        }

        // Fall back to named colors
        match color_str.to_lowercase().as_str() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{Series, DataFrame, PlotConfig, DataFormat};

    #[test]
    fn test_vertical_bar_chart() {
        let series = Series {
            name: "Test".to_string(),
            data: vec![10.0, 25.0, 15.0, 30.0, 20.0],
        };
        let dataframe = DataFrame {
            columns: vec![series],
            headers: None,
        };
        let config = PlotConfig {
            width: 50,
            height: 20,
            title: Some("Revenue by Quarter".to_string()),
            xlabel: None,
            ylabel: None,
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: None,
            symbol: Some('█'),
        };

        let bar_chart = BarChart::vertical();
        let result = bar_chart.render(&dataframe, &config);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("Revenue by Quarter"));
        assert!(output.contains("30"));
        assert!(output.contains("┤"));
        assert!(output.contains("└"));
    }

    #[test]
    fn test_horizontal_bar_chart() {
        let series = Series {
            name: "Test".to_string(),
            data: vec![1.0, 3.0, 2.0, 5.0, 4.0],
        };
        let dataframe = DataFrame {
            columns: vec![series],
            headers: None,
        };
        let config = PlotConfig {
            width: 40,
            height: 20,
            title: Some("Horizontal Test".to_string()),
            xlabel: None,
            ylabel: None,
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: Some("green".to_string()),
            symbol: Some('▓'),
        };

        let bar_chart = BarChart::horizontal();
        let result = bar_chart.render(&dataframe, &config);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("Horizontal Test"));
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

        let bar_chart = BarChart::vertical();
        let result = bar_chart.render(&dataframe, &config);
        assert!(result.is_err());
    }
}