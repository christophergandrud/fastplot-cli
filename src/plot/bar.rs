use crate::data::{DataFrame, PlotConfig};
use crate::plot::{Canvas, ColorUtils, DataUtils, RenderUtils};
use crate::plot::{ElementLayout, BarStyle};
use anyhow::{Result, anyhow};
use crossterm::style::Color;

// Note: BarLayout and BarStyle have been moved to the layout module

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
        
        // Validate and analyze data
        RenderUtils::validate_plot_data(data, "bar chart")?;
        
        if DataUtils::has_constant_values(data) {
            return RenderUtils::handle_constant_values(data, config);
        }

        // Calculate data range with utilities
        let (min_val_orig, max_val) = DataUtils::calculate_range(data)?;
        let min_val = min_val_orig.min(0.0); // Ensure we include zero for bar charts

        // Calculate chart dimensions using consistent layout constants
        let chart_height = config.height.saturating_sub(RenderUtils::total_vertical_overhead());
        let chart_width = config.width.saturating_sub(RenderUtils::Y_AXIS_LABEL_WIDTH);
        
        // Create clean Y-axis labels (round numbers)
        let y_range = max_val - min_val;
        let label_step = 2; // Show labels every 2 rows
        
        // Calculate dynamic layout using new layout system
        let layout = ElementLayout::for_bars(chart_width, data.len());
        let max_displayable = layout.max_elements_for_width(chart_width);
        
        // Check if we need to truncate data
        let display_data = if data.len() > max_displayable {
            eprintln!("Warning: Showing only first {} of {} data points", 
                      max_displayable, data.len());
            &data[..max_displayable]
        } else {
            data
        };
        
        let num_bars = display_data.len();
        let bar_style = BarStyle::from_width(layout.element_width);
        
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
                // Skip to next line immediately for last row - no bars
                output.push('\n');
                continue;
            } else {
                output.push_str("     ┤");
            }
            
            // Draw bars for this row (skip bars on the last row which is reserved for x-axis)
            if !is_last_row {
                // Create a line buffer for this row
                let mut line = vec![' '; chart_width];
                let row_threshold = 1.0 - (row as f64 / (chart_height - 1) as f64);
                
                for (i, &value) in display_data.iter().enumerate() {
                    let normalized = if y_range > 0.0 {
                        (value - min_val) / y_range
                    } else {
                        0.5
                    };
                    
                    // Check if this bar should be filled at this height
                    if normalized >= row_threshold || 
                       (normalized == 0.0 && row == chart_height - 2) {
                        let x_pos = layout.element_position(i);
                        let bar_symbol = bar_style.get_symbol();
                        
                        // Draw the bar at this position
                        for j in 0..layout.element_width {
                            if x_pos + j < chart_width {
                                line[x_pos + j] = bar_symbol;
                            }
                        }
                    }
                }
                
                // Apply color if specified and convert to string
                let line_str: String = line.iter().collect();
                if let Some(color_name) = &config.color {
                    let colored_line = ColorUtils::apply_color_string(&line_str, color_name);
                    output.push_str(&colored_line);
                } else {
                    output.push_str(&line_str);
                }
            }
            
            output.push('\n');
        }
        
        // X-axis base line with dynamic positioning
        output.push_str("     ");
        let mut x_axis = vec![' '; chart_width];
        
        // Draw horizontal line and tick marks
        let start_pos = layout.offset;
        let end_pos = if num_bars > 0 {
            layout.element_position(num_bars - 1) + layout.element_width
        } else {
            start_pos
        };
        
        // Fill with horizontal line
        for i in start_pos..end_pos.min(chart_width) {
            x_axis[i] = '─';
        }
        
        // Add tick marks under bar centers
        for i in 0..num_bars {
            let tick_pos = layout.element_position(i) + layout.element_width / 2;
            if tick_pos < chart_width {
                x_axis[tick_pos] = '┴';
            }
        }
        
        let x_axis_str: String = x_axis.iter().collect();
        output.push_str(&x_axis_str);
        output.push('\n');
        
        // X-axis labels with dynamic positioning
        if num_bars <= 15 {
            output.push_str("     ");
            let mut labels = vec![' '; chart_width];
            
            for i in 0..num_bars {
                let label = format!("{}", i + 1);
                let label_center = layout.element_position(i) + layout.element_width / 2;
                let label_start = label_center.saturating_sub(label.len() / 2);
                
                for (j, ch) in label.chars().enumerate() {
                    if label_start + j < chart_width {
                        labels[label_start + j] = ch;
                    }
                }
            }
            
            let labels_str: String = labels.iter().collect();
            output.push_str(&labels_str);
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
        
        // Use shared utilities for data processing
        if DataUtils::has_constant_values(data) {
            let (x_min, x_max) = DataUtils::calculate_range_with_padding(data, 10.0)?;
            canvas.set_ranges((x_min, x_max), (0.0, data.len() as f64));
        } else {
            let (min_val, max_val) = DataUtils::calculate_range(data)?;
            let (x_min, x_max) = (min_val.min(0.0), max_val);
            canvas.set_ranges((x_min, x_max), (0.0, data.len() as f64));
        }

        canvas.draw_axis();

        let bar_height = (config.height as f64 / data.len() as f64).max(1.0);
        let color = ColorUtils::parse_color(&config.color);
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

    // Color parsing methods removed - now using shared ColorUtils
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