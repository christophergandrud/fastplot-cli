use crate::data::{DataFrame, PlotConfig};
use crate::plot::{ColorUtils, DataUtils, RenderUtils};
use crate::plot::{ElementLayout, TickStyle};
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

        // Calculate chart dimensions using consistent layout constants
        let chart_height = config.height.saturating_sub(RenderUtils::total_vertical_overhead());
        let chart_width = config.width.saturating_sub(RenderUtils::Y_AXIS_LABEL_WIDTH);
        
        // Add some padding to the Y range
        let y_range = max_val - min_val;
        let padding = y_range * 0.1;
        let y_min = min_val - padding;
        let y_max = max_val + padding;
        let y_range_padded = y_max - y_min;
        
        let mut output = String::new();
        
        // Add title
        if let Some(title) = &config.title {
            output.push_str(&format!("{:^width$}\n\n", title, width = config.width));
        }

        // Create a 2D grid to place points and lines
        let mut grid = vec![vec![' '; chart_width]; chart_height];
        
        // Plot data points and connect with lines
        for (i, &value) in data.iter().enumerate() {
            // Calculate position on grid
            let x_ratio = if data.len() > 1 { i as f64 / (data.len() - 1) as f64 } else { 0.5 };
            let y_ratio = (value - y_min) / y_range_padded;
            
            let x_pos = (x_ratio * (chart_width - 1) as f64) as usize;
            let y_pos = chart_height - 1 - ((y_ratio * (chart_height - 1) as f64) as usize);
            
            // Place the point
            if x_pos < chart_width && y_pos < chart_height {
                grid[y_pos][x_pos] = symbol;
            }
            
            // Draw line to next point
            if i < data.len() - 1 {
                let next_value = data[i + 1];
                let next_x_ratio = (i + 1) as f64 / (data.len() - 1) as f64;
                let next_y_ratio = (next_value - y_min) / y_range_padded;
                
                let next_x_pos = (next_x_ratio * (chart_width - 1) as f64) as usize;
                let next_y_pos = chart_height - 1 - ((next_y_ratio * (chart_height - 1) as f64) as usize);
                
                // Smooth line drawing between points with better characters
                if config.width > 100 {
                    // Use high-resolution Braille for wide plots
                    self.draw_braille_line(&mut grid, x_pos, y_pos, next_x_pos, next_y_pos);
                } else {
                    // Use standard smooth line for smaller plots
                    self.draw_smooth_line(&mut grid, x_pos, y_pos, next_x_pos, next_y_pos);
                }
            }
        }
        
        // Render the grid with Y-axis labels
        for (row_idx, row) in grid.iter().enumerate() {
            let y_value = y_max - (row_idx as f64 / (chart_height - 1) as f64) * y_range_padded;
            let is_label_row = row_idx % 3 == 0; // Show labels every 3 rows
            
            if is_label_row {
                output.push_str(&format!("{:>6.1} ┤", y_value));
            } else {
                output.push_str("       ┤");
            }
            
            // Add the row content with color support
            let row_str: String = row.iter().collect();
            if let Some(color_name) = &config.color {
                let colored_str = ColorUtils::apply_color_string(&row_str, color_name);
                output.push_str(&colored_str);
            } else {
                output.push_str(&row_str);
            }
            output.push('\n');
        }
        
        // X-axis with properly aligned tick marks using ElementLayout
        output.push_str("     └");
        let tick_layout = ElementLayout::for_ticks(chart_width, 5); // 5 tick marks
        let mut x_axis = vec!['─'; chart_width];
        
        // Add tick marks at calculated positions
        for i in 0..5 {
            let tick_pos = tick_layout.element_position(i);
            if tick_pos < chart_width {
                x_axis[tick_pos] = TickStyle::Standard.get_symbol();
            }
        }
        
        let x_axis_str: String = x_axis.iter().collect();
        output.push_str(&x_axis_str);
        output.push('\n');
        
        // X-axis labels with proper positioning using ElementLayout
        if chart_width > 20 {
            let x_labels = self.create_axis_labels_with_layout(data.len(), chart_width);
            output.push_str("      ");
            output.push_str(&x_labels);
            output.push('\n');
        }

        Ok(output)
    }



    
    fn create_axis_labels_with_layout(&self, data_len: usize, chart_width: usize) -> String {
        let mut axis_line = vec![' '; chart_width];
        let num_labels = 5.min(data_len);
        
        if num_labels == 0 {
            return axis_line.into_iter().collect();
        }
        
        // Use ElementLayout for consistent positioning
        let label_layout = ElementLayout::for_ticks(chart_width, num_labels);
        
        // Calculate label values and positions using the layout
        for i in 0..num_labels {
            let x_index = if data_len > 1 { 
                i * (data_len - 1) / (num_labels - 1)
            } else { 
                0 
            };
            let position = label_layout.element_position(i);
            let label = x_index.to_string();
            
            // Center the label at the calculated position
            let label_start = position.saturating_sub(label.len() / 2);
            let label_end = (label_start + label.len()).min(chart_width);
            
            // Check if there's enough space for the label
            let mut can_place = true;
            for j in label_start..label_end {
                if axis_line[j] != ' ' {
                    can_place = false;
                    break;
                }
            }
            
            // Place the label if there's space
            if can_place {
                for (j, ch) in label.chars().enumerate() {
                    if label_start + j < chart_width {
                        axis_line[label_start + j] = ch;
                    }
                }
            }
        }
        
        axis_line.into_iter().collect()
    }

    fn draw_braille_line(&self, grid: &mut Vec<Vec<char>>, x1: usize, y1: usize, x2: usize, y2: usize) {
        // Use Braille patterns for sub-character precision
        let x1_f = x1 as f64;
        let y1_f = y1 as f64;
        let x2_f = x2 as f64;
        let y2_f = y2 as f64;
        
        let dx = x2_f - x1_f;
        let dy = y2_f - y1_f;
        let steps = ((dx.abs() + dy.abs()) * 8.0).max(20.0) as usize; // Very fine resolution
        
        if steps == 0 {
            return;
        }
        
        for i in 1..steps {
            let t = i as f64 / steps as f64;
            let x = x1_f + t * dx;
            let y = y1_f + t * dy;
            
            let base_x = x.floor() as usize;
            let base_y = y.floor() as usize;
            let frac_x = x.fract();
            let frac_y = y.fract();
            
            if base_x < grid[0].len() && base_y < grid.len() {
                if grid[base_y][base_x] == ' ' {
                    // Use Braille dots for sub-character positioning
                    grid[base_y][base_x] = self.get_braille_char(frac_x, frac_y);
                }
            }
        }
    }

    fn get_braille_char(&self, x_fraction: f64, y_fraction: f64) -> char {
        // Braille Unicode patterns for sub-character resolution
        // Each character represents a 2x4 grid of dots
        const BRAILLE_BASE: u32 = 0x2800;
        
        let x_idx = if x_fraction < 0.5 { 0 } else { 1 };
        let y_idx = (y_fraction * 4.0).min(3.9) as usize;
        
        let dot_pattern = match (x_idx, y_idx) {
            (0, 0) => 0x01, // ⠁
            (0, 1) => 0x02, // ⠂
            (0, 2) => 0x04, // ⠄
            (0, 3) => 0x40, // ⡀
            (1, 0) => 0x08, // ⠈
            (1, 1) => 0x10, // ⠐
            (1, 2) => 0x20, // ⠠
            (1, 3) => 0x80, // ⢀
            _ => 0x01,
        };
        
        char::from_u32(BRAILLE_BASE + dot_pattern).unwrap_or('∙')
    }

    fn draw_smooth_line(&self, grid: &mut Vec<Vec<char>>, x1: usize, y1: usize, x2: usize, y2: usize) {
        // Use fine-grained interpolation to avoid staircase effect
        let x1_f = x1 as f64;
        let y1_f = y1 as f64;
        let x2_f = x2 as f64;
        let y2_f = y2 as f64;
        
        // Use many more steps for truly smooth lines
        let dx = (x2_f - x1_f).abs();
        let dy = (y2_f - y1_f).abs();
        let steps = ((dx + dy) * 4.0).max(10.0) as usize; // Much finer granularity
        
        if steps == 0 {
            return;
        }
        
        for i in 1..steps {
            let t = i as f64 / steps as f64;
            let x = x1_f + t * (x2_f - x1_f);
            let y = y1_f + t * (y2_f - y1_f);
            
            let ux = x.round() as usize;
            let uy = y.round() as usize;
            
            if ux < grid[0].len() && uy < grid.len() {
                if grid[uy][ux] == ' ' {  // Don't overwrite data points
                    // Use very light character for subtle connection
                    grid[uy][ux] = '∙';  // Even lighter than ·
                }
            }
        }
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