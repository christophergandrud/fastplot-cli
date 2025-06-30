use crate::data::{DataFrame, PlotConfig};
use crate::plot::Canvas;
use anyhow::{Result, anyhow};
use crossterm::style::Color;

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

        let mut canvas = Canvas::with_labels(
            config.width,
            config.height,
            config.title.clone(),
            config.xlabel.clone(),
            config.ylabel.clone(),
        );

        if self.horizontal {
            self.render_horizontal_bars(&mut canvas, series, config)?;
        } else {
            self.render_vertical_bars(&mut canvas, series, config)?;
        }

        Ok(canvas.render_colored(config.color.is_some()))
    }

    fn render_vertical_bars(&self, canvas: &mut Canvas, series: &crate::data::Series, config: &PlotConfig) -> Result<()> {
        let data = &series.data;
        let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        
        // Handle edge case where all values are the same
        let (y_min, y_max) = if (max_val - min_val).abs() < f64::EPSILON {
            (min_val - 1.0, max_val + 1.0)
        } else {
            (min_val.min(0.0), max_val)
        };

        canvas.set_ranges((0.0, data.len() as f64), (y_min, y_max));
        canvas.draw_axis();

        let bar_width = (config.width as f64 / data.len() as f64).max(1.0);
        let color = self.parse_color(&config.color);
        let symbol = config.symbol.unwrap_or('█');

        for (i, &value) in data.iter().enumerate() {
            let x_start = i as f64;
            let x_end = x_start + bar_width.min(1.0);
            
            if value >= 0.0 {
                // Positive bar from 0 to value
                self.fill_bar(canvas, x_start, 0.0, x_end, value, symbol, color);
            } else {
                // Negative bar from value to 0
                self.fill_bar(canvas, x_start, value, x_end, 0.0, symbol, color);
            }
        }

        Ok(())
    }

    fn render_horizontal_bars(&self, canvas: &mut Canvas, series: &crate::data::Series, config: &PlotConfig) -> Result<()> {
        let data = &series.data;
        let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        
        // Handle edge case where all values are the same
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
                // Positive bar from 0 to value
                self.fill_bar(canvas, 0.0, y_start, value, y_end, symbol, color);
            } else {
                // Negative bar from value to 0
                self.fill_bar(canvas, value, y_start, 0.0, y_end, symbol, color);
            }
        }

        Ok(())
    }

    fn fill_bar(&self, canvas: &mut Canvas, x1: f64, y1: f64, x2: f64, y2: f64, symbol: char, color: Option<Color>) {
        // Fill the rectangular area for the bar
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
            data: vec![1.0, 3.0, 2.0, 5.0, 4.0],
        };
        let dataframe = DataFrame {
            columns: vec![series],
            headers: None,
        };
        let config = PlotConfig {
            width: 40,
            height: 20,
            title: Some("Test Chart".to_string()),
            xlabel: None,
            ylabel: None,
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: Some("blue".to_string()),
            symbol: Some('█'),
        };

        let bar_chart = BarChart::vertical();
        let result = bar_chart.render(&dataframe, &config);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("Test Chart"));
        assert!(!output.is_empty());
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