use crate::data::{DataFrame, PlotConfig};
use crate::plot::Canvas;
use anyhow::{Result, anyhow};
use crossterm::style::Color;

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

        let mut canvas = Canvas::with_labels(
            config.width,
            config.height,
            config.title.clone(),
            config.xlabel.clone(),
            config.ylabel.clone(),
        );

        if self.multi_series {
            self.render_multi_series(&mut canvas, data, config)?;
        } else {
            self.render_single_series(&mut canvas, &data.columns[0], config)?;
        }

        Ok(canvas.render_colored(config.color.is_some()))
    }

    fn render_single_series(&self, canvas: &mut Canvas, series: &crate::data::Series, config: &PlotConfig) -> Result<()> {
        let data = &series.data;
        if data.is_empty() {
            return Err(anyhow!("Empty data series"));
        }

        let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        
        // Handle edge case where all values are the same
        let (y_min, y_max) = if (max_val - min_val).abs() < f64::EPSILON {
            (min_val - 1.0, max_val + 1.0)
        } else {
            // Add small padding
            let padding = (max_val - min_val) * 0.05;
            (min_val - padding, max_val + padding)
        };

        canvas.set_ranges((0.0, (data.len() - 1) as f64), (y_min, y_max));
        canvas.draw_axis();

        let color = self.parse_color(&config.color);
        let symbol = config.symbol.unwrap_or('●');

        // Plot points
        for (i, &value) in data.iter().enumerate() {
            let x = i as f64;
            canvas.plot_point_with_color(x, value, symbol, color);
        }

        // Connect points with lines
        for i in 0..data.len() - 1 {
            let x1 = i as f64;
            let y1 = data[i];
            let x2 = (i + 1) as f64;
            let y2 = data[i + 1];
            
            canvas.plot_line_with_color(x1, y1, x2, y2, '·', color);
        }

        Ok(())
    }

    fn render_multi_series(&self, canvas: &mut Canvas, data: &DataFrame, config: &PlotConfig) -> Result<()> {
        if data.columns.is_empty() {
            return Err(anyhow!("No data series provided"));
        }

        // Find global min/max across all series
        let mut global_min = f64::INFINITY;
        let mut global_max = f64::NEG_INFINITY;
        let mut max_length = 0;

        for series in &data.columns {
            if !series.data.is_empty() {
                let min_val = series.data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                let max_val = series.data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                global_min = global_min.min(min_val);
                global_max = global_max.max(max_val);
                max_length = max_length.max(series.data.len());
            }
        }

        if max_length == 0 {
            return Err(anyhow!("All data series are empty"));
        }

        // Handle edge case where all values are the same
        let (y_min, y_max) = if (global_max - global_min).abs() < f64::EPSILON {
            (global_min - 1.0, global_max + 1.0)
        } else {
            let padding = (global_max - global_min) * 0.05;
            (global_min - padding, global_max + padding)
        };

        canvas.set_ranges((0.0, (max_length - 1) as f64), (y_min, y_max));
        canvas.draw_axis();

        let colors = [
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Yellow,
            Color::Magenta,
            Color::Cyan,
        ];
        let symbols = ['●', '■', '▲', '◆', '▼', '★'];

        // Plot each series with different colors/symbols
        for (series_idx, series) in data.columns.iter().enumerate() {
            if series.data.is_empty() {
                continue;
            }

            let color = Some(colors[series_idx % colors.len()]);
            let symbol = symbols[series_idx % symbols.len()];

            // Plot points
            for (i, &value) in series.data.iter().enumerate() {
                let x = i as f64;
                canvas.plot_point_with_color(x, value, symbol, color);
            }

            // Connect points with lines
            for i in 0..series.data.len() - 1 {
                let x1 = i as f64;
                let y1 = series.data[i];
                let x2 = (i + 1) as f64;
                let y2 = series.data[i + 1];
                
                canvas.plot_line_with_color(x1, y1, x2, y2, '·', color);
            }
        }

        Ok(())
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