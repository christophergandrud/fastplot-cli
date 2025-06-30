use crate::data::{DataFrame, PlotConfig};
use crate::plot::Canvas;
use anyhow::{Result, anyhow};
use crossterm::style::Color;

pub struct BoxPlot {
    orientation: Orientation,
    show_outliers: bool,
    outlier_method: OutlierMethod,
}

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy)]
pub enum OutlierMethod {
    IQR,      // 1.5 * IQR beyond Q1/Q3
    Tukey,    // 3 * IQR beyond Q1/Q3
    None,     // No outlier detection
}

impl BoxPlot {
    pub fn new(orientation: Orientation) -> Self {
        Self {
            orientation,
            show_outliers: true,
            outlier_method: OutlierMethod::IQR,
        }
    }

    pub fn vertical() -> Self {
        Self::new(Orientation::Vertical)
    }

    pub fn horizontal() -> Self {
        Self::new(Orientation::Horizontal)
    }

    pub fn hide_outliers(mut self) -> Self {
        self.show_outliers = false;
        self
    }

    pub fn with_outlier_method(mut self, method: OutlierMethod) -> Self {
        self.outlier_method = method;
        self
    }

    pub fn render(&self, data: &DataFrame, config: &PlotConfig) -> Result<String> {
        if data.columns.is_empty() {
            return Err(anyhow!("No data provided for box plot"));
        }

        let mut canvas = Canvas::with_labels(
            config.width,
            config.height,
            config.title.clone(),
            config.xlabel.clone(),
            config.ylabel.clone(),
        );

        if data.columns.len() == 1 {
            // Single box plot
            let box_stats = self.calculate_box_statistics(&data.columns[0].data)?;
            self.render_single_box(&mut canvas, &box_stats, config)?;
        } else {
            // Multiple box plots
            self.render_multiple_boxes(&mut canvas, data, config)?;
        }

        Ok(canvas.render_colored(config.color.is_some()))
    }

    fn calculate_box_statistics(&self, data: &[f64]) -> Result<BoxStatistics> {
        if data.is_empty() {
            return Err(anyhow!("Empty data series"));
        }

        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let n = sorted_data.len();
        let median = self.calculate_median(&sorted_data);
        let q1 = self.calculate_percentile(&sorted_data, 0.25);
        let q3 = self.calculate_percentile(&sorted_data, 0.75);
        let iqr = q3 - q1;

        // Calculate whiskers and outliers
        let (lower_whisker, upper_whisker, outliers) = if self.show_outliers {
            let multiplier = match self.outlier_method {
                OutlierMethod::IQR => 1.5,
                OutlierMethod::Tukey => 3.0,
                OutlierMethod::None => f64::INFINITY,
            };

            let lower_fence = q1 - multiplier * iqr;
            let upper_fence = q3 + multiplier * iqr;

            let mut outliers = Vec::new();
            let mut lower_whisker = q1;
            let mut upper_whisker = q3;

            for &value in &sorted_data {
                if value < lower_fence || value > upper_fence {
                    outliers.push(value);
                } else {
                    if value < q1 {
                        lower_whisker = lower_whisker.min(value);
                    }
                    if value > q3 {
                        upper_whisker = upper_whisker.max(value);
                    }
                }
            }

            // If no values within whisker range, use min/max
            if lower_whisker == q1 && !sorted_data.is_empty() {
                lower_whisker = sorted_data[0];
            }
            if upper_whisker == q3 && !sorted_data.is_empty() {
                upper_whisker = sorted_data[n - 1];
            }

            (lower_whisker, upper_whisker, outliers)
        } else {
            (sorted_data[0], sorted_data[n - 1], Vec::new())
        };

        Ok(BoxStatistics {
            median,
            q1,
            q3,
            lower_whisker,
            upper_whisker,
            outliers,
            mean: data.iter().sum::<f64>() / data.len() as f64,
        })
    }

    fn calculate_median(&self, sorted_data: &[f64]) -> f64 {
        let n = sorted_data.len();
        if n % 2 == 0 {
            (sorted_data[n / 2 - 1] + sorted_data[n / 2]) / 2.0
        } else {
            sorted_data[n / 2]
        }
    }

    fn calculate_percentile(&self, sorted_data: &[f64], percentile: f64) -> f64 {
        let n = sorted_data.len();
        let index = percentile * (n - 1) as f64;
        let lower_index = index.floor() as usize;
        let upper_index = index.ceil() as usize;
        let weight = index - lower_index as f64;

        if lower_index == upper_index {
            sorted_data[lower_index]
        } else {
            sorted_data[lower_index] * (1.0 - weight) + sorted_data[upper_index] * weight
        }
    }

    fn render_single_box(&self, canvas: &mut Canvas, stats: &BoxStatistics, config: &PlotConfig) -> Result<()> {
        match self.orientation {
            Orientation::Vertical => self.render_vertical_box(canvas, stats, 0.0, config),
            Orientation::Horizontal => self.render_horizontal_box(canvas, stats, 0.0, config),
        }
    }

    fn render_vertical_box(&self, canvas: &mut Canvas, stats: &BoxStatistics, x_position: f64, config: &PlotConfig) -> Result<()> {
        let y_min = stats.lower_whisker.min(stats.outliers.iter().copied().fold(f64::INFINITY, f64::min));
        let y_max = stats.upper_whisker.max(stats.outliers.iter().copied().fold(f64::NEG_INFINITY, f64::max));
        let y_padding = (y_max - y_min) * 0.1;

        canvas.set_ranges((x_position - 1.0, x_position + 1.0), (y_min - y_padding, y_max + y_padding));
        canvas.draw_axis();

        let color = self.parse_color(&config.color);
        let box_width = 0.6;

        // Draw whiskers
        canvas.plot_line_with_color(x_position, stats.lower_whisker, x_position, stats.q1, '│', color);
        canvas.plot_line_with_color(x_position, stats.q3, x_position, stats.upper_whisker, '│', color);

        // Draw whisker caps
        canvas.plot_line_with_color(x_position - box_width/4.0, stats.lower_whisker, x_position + box_width/4.0, stats.lower_whisker, '─', color);
        canvas.plot_line_with_color(x_position - box_width/4.0, stats.upper_whisker, x_position + box_width/4.0, stats.upper_whisker, '─', color);

        // Draw box
        self.draw_vertical_box(canvas, x_position, stats.q1, stats.q3, box_width, color);

        // Draw median line
        canvas.plot_line_with_color(x_position - box_width/2.0, stats.median, x_position + box_width/2.0, stats.median, '━', Some(Color::Yellow));

        // Draw outliers
        if self.show_outliers {
            for &outlier in &stats.outliers {
                canvas.plot_point_with_color(x_position, outlier, '●', Some(Color::Red));
            }
        }

        Ok(())
    }

    fn render_horizontal_box(&self, canvas: &mut Canvas, stats: &BoxStatistics, y_position: f64, config: &PlotConfig) -> Result<()> {
        let x_min = stats.lower_whisker.min(stats.outliers.iter().copied().fold(f64::INFINITY, f64::min));
        let x_max = stats.upper_whisker.max(stats.outliers.iter().copied().fold(f64::NEG_INFINITY, f64::max));
        let x_padding = (x_max - x_min) * 0.1;

        canvas.set_ranges((x_min - x_padding, x_max + x_padding), (y_position - 1.0, y_position + 1.0));
        canvas.draw_axis();

        let color = self.parse_color(&config.color);
        let box_height = 0.6;

        // Draw whiskers
        canvas.plot_line_with_color(stats.lower_whisker, y_position, stats.q1, y_position, '─', color);
        canvas.plot_line_with_color(stats.q3, y_position, stats.upper_whisker, y_position, '─', color);

        // Draw whisker caps
        canvas.plot_line_with_color(stats.lower_whisker, y_position - box_height/4.0, stats.lower_whisker, y_position + box_height/4.0, '│', color);
        canvas.plot_line_with_color(stats.upper_whisker, y_position - box_height/4.0, stats.upper_whisker, y_position + box_height/4.0, '│', color);

        // Draw box
        self.draw_horizontal_box(canvas, y_position, stats.q1, stats.q3, box_height, color);

        // Draw median line
        canvas.plot_line_with_color(stats.median, y_position - box_height/2.0, stats.median, y_position + box_height/2.0, '┃', Some(Color::Yellow));

        // Draw outliers
        if self.show_outliers {
            for &outlier in &stats.outliers {
                canvas.plot_point_with_color(outlier, y_position, '●', Some(Color::Red));
            }
        }

        Ok(())
    }

    fn draw_vertical_box(&self, canvas: &mut Canvas, x_center: f64, q1: f64, q3: f64, width: f64, color: Option<Color>) {
        let x_left = x_center - width / 2.0;
        let x_right = x_center + width / 2.0;

        // Draw box outline
        canvas.plot_line_with_color(x_left, q1, x_right, q1, '─', color);      // Bottom
        canvas.plot_line_with_color(x_left, q3, x_right, q3, '─', color);      // Top
        canvas.plot_line_with_color(x_left, q1, x_left, q3, '│', color);       // Left
        canvas.plot_line_with_color(x_right, q1, x_right, q3, '│', color);     // Right

        // Fill box lightly
        let fill_steps_x = (width * 10.0) as usize + 1;
        let fill_steps_y = ((q3 - q1) * 10.0) as usize + 1;

        for i in 1..fill_steps_x {
            for j in 1..fill_steps_y {
                let x = x_left + width * (i as f64 / fill_steps_x as f64);
                let y = q1 + (q3 - q1) * (j as f64 / fill_steps_y as f64);
                canvas.plot_point_with_color(x, y, '░', color);
            }
        }
    }

    fn draw_horizontal_box(&self, canvas: &mut Canvas, y_center: f64, q1: f64, q3: f64, height: f64, color: Option<Color>) {
        let y_bottom = y_center - height / 2.0;
        let y_top = y_center + height / 2.0;

        // Draw box outline
        canvas.plot_line_with_color(q1, y_bottom, q1, y_top, '│', color);      // Left
        canvas.plot_line_with_color(q3, y_bottom, q3, y_top, '│', color);      // Right
        canvas.plot_line_with_color(q1, y_bottom, q3, y_bottom, '─', color);   // Bottom
        canvas.plot_line_with_color(q1, y_top, q3, y_top, '─', color);         // Top

        // Fill box lightly
        let fill_steps_x = ((q3 - q1) * 10.0) as usize + 1;
        let fill_steps_y = (height * 10.0) as usize + 1;

        for i in 1..fill_steps_x {
            for j in 1..fill_steps_y {
                let x = q1 + (q3 - q1) * (i as f64 / fill_steps_x as f64);
                let y = y_bottom + height * (j as f64 / fill_steps_y as f64);
                canvas.plot_point_with_color(x, y, '░', color);
            }
        }
    }

    fn render_multiple_boxes(&self, canvas: &mut Canvas, data: &DataFrame, config: &PlotConfig) -> Result<()> {
        let num_boxes = data.columns.len();
        let mut all_stats = Vec::with_capacity(num_boxes);

        // Calculate statistics for all series
        for series in &data.columns {
            if !series.data.is_empty() {
                all_stats.push(self.calculate_box_statistics(&series.data)?);
            }
        }

        if all_stats.is_empty() {
            return Err(anyhow!("No valid data series found"));
        }

        // Find global range
        let mut global_min = f64::INFINITY;
        let mut global_max = f64::NEG_INFINITY;

        for stats in &all_stats {
            global_min = global_min.min(stats.lower_whisker).min(stats.outliers.iter().copied().fold(f64::INFINITY, f64::min));
            global_max = global_max.max(stats.upper_whisker).max(stats.outliers.iter().copied().fold(f64::NEG_INFINITY, f64::max));
        }

        let padding = (global_max - global_min) * 0.1;

        match self.orientation {
            Orientation::Vertical => {
                canvas.set_ranges((0.0, num_boxes as f64), (global_min - padding, global_max + padding));
                canvas.draw_axis();

                for (i, stats) in all_stats.iter().enumerate() {
                    let x_pos = i as f64 + 0.5;
                    self.render_vertical_box(canvas, stats, x_pos, config)?;
                }
            }
            Orientation::Horizontal => {
                canvas.set_ranges((global_min - padding, global_max + padding), (0.0, num_boxes as f64));
                canvas.draw_axis();

                for (i, stats) in all_stats.iter().enumerate() {
                    let y_pos = i as f64 + 0.5;
                    self.render_horizontal_box(canvas, stats, y_pos, config)?;
                }
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
                _ => Some(Color::Blue),
            }
        })
    }
}

struct BoxStatistics {
    median: f64,
    q1: f64,
    q3: f64,
    lower_whisker: f64,
    upper_whisker: f64,
    outliers: Vec<f64>,
    mean: f64,
}

/// Notched box plot that shows confidence intervals around the median
pub struct NotchedBoxPlot {
    orientation: Orientation,
    confidence_level: f64,
}

impl NotchedBoxPlot {
    pub fn new(orientation: Orientation) -> Self {
        Self {
            orientation,
            confidence_level: 0.95,
        }
    }

    pub fn with_confidence_level(mut self, level: f64) -> Self {
        self.confidence_level = level.max(0.5).min(0.99);
        self
    }

    pub fn render(&self, data: &DataFrame, config: &PlotConfig) -> Result<String> {
        if data.columns.is_empty() {
            return Err(anyhow!("No data provided for notched box plot"));
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

        let stats = self.calculate_notched_statistics(&series.data)?;
        self.render_notched_box(&mut canvas, &stats, config)?;

        Ok(canvas.render_colored(config.color.is_some()))
    }

    fn calculate_notched_statistics(&self, data: &[f64]) -> Result<NotchedBoxStatistics> {
        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let n = sorted_data.len();
        let median = self.calculate_median(&sorted_data);
        let q1 = self.calculate_percentile(&sorted_data, 0.25);
        let q3 = self.calculate_percentile(&sorted_data, 0.75);

        // Calculate confidence interval for median
        let z_score = match self.confidence_level {
            0.90 => 1.645,
            0.95 => 1.96,
            0.99 => 2.576,
            _ => 1.96, // Default to 95%
        };

        let iqr = q3 - q1;
        let notch_size = z_score * iqr / (1.57 * (n as f64).sqrt());
        let lower_notch = median - notch_size;
        let upper_notch = median + notch_size;

        Ok(NotchedBoxStatistics {
            median,
            q1,
            q3,
            lower_whisker: sorted_data[0],
            upper_whisker: sorted_data[n - 1],
            lower_notch,
            upper_notch,
        })
    }

    fn calculate_median(&self, sorted_data: &[f64]) -> f64 {
        let n = sorted_data.len();
        if n % 2 == 0 {
            (sorted_data[n / 2 - 1] + sorted_data[n / 2]) / 2.0
        } else {
            sorted_data[n / 2]
        }
    }

    fn calculate_percentile(&self, sorted_data: &[f64], percentile: f64) -> f64 {
        let n = sorted_data.len();
        let index = percentile * (n - 1) as f64;
        let lower_index = index.floor() as usize;
        let upper_index = index.ceil() as usize;
        let weight = index - lower_index as f64;

        if lower_index == upper_index {
            sorted_data[lower_index]
        } else {
            sorted_data[lower_index] * (1.0 - weight) + sorted_data[upper_index] * weight
        }
    }

    fn render_notched_box(&self, canvas: &mut Canvas, stats: &NotchedBoxStatistics, config: &PlotConfig) -> Result<()> {
        let y_padding = (stats.upper_whisker - stats.lower_whisker) * 0.1;
        canvas.set_ranges((-1.0, 1.0), (stats.lower_whisker - y_padding, stats.upper_whisker + y_padding));
        canvas.draw_axis();

        let color = if let Some(ref color_str) = config.color {
            match color_str.to_lowercase().as_str() {
                "red" => Some(Color::Red),
                "green" => Some(Color::Green),
                "blue" => Some(Color::Blue),
                _ => Some(Color::Blue),
            }
        } else {
            Some(Color::Blue)
        };

        // Draw whiskers
        canvas.plot_line_with_color(0.0, stats.lower_whisker, 0.0, stats.q1, '│', color);
        canvas.plot_line_with_color(0.0, stats.q3, 0.0, stats.upper_whisker, '│', color);

        // Draw box with notches
        let box_width = 0.6;
        let notch_width = 0.3;

        // Box outline with notches
        canvas.plot_line_with_color(-box_width/2.0, stats.q1, box_width/2.0, stats.q1, '─', color);
        canvas.plot_line_with_color(-box_width/2.0, stats.q3, box_width/2.0, stats.q3, '─', color);
        canvas.plot_line_with_color(-box_width/2.0, stats.q1, -box_width/2.0, stats.lower_notch, '│', color);
        canvas.plot_line_with_color(-box_width/2.0, stats.upper_notch, -box_width/2.0, stats.q3, '│', color);
        canvas.plot_line_with_color(box_width/2.0, stats.q1, box_width/2.0, stats.lower_notch, '│', color);
        canvas.plot_line_with_color(box_width/2.0, stats.upper_notch, box_width/2.0, stats.q3, '│', color);

        // Notch lines
        canvas.plot_line_with_color(-box_width/2.0, stats.lower_notch, -notch_width/2.0, stats.median, '╲', color);
        canvas.plot_line_with_color(-notch_width/2.0, stats.median, -box_width/2.0, stats.upper_notch, '╱', color);
        canvas.plot_line_with_color(box_width/2.0, stats.lower_notch, notch_width/2.0, stats.median, '╱', color);
        canvas.plot_line_with_color(notch_width/2.0, stats.median, box_width/2.0, stats.upper_notch, '╲', color);

        // Median line
        canvas.plot_line_with_color(-notch_width/2.0, stats.median, notch_width/2.0, stats.median, '━', Some(Color::Yellow));

        Ok(())
    }
}

struct NotchedBoxStatistics {
    median: f64,
    q1: f64,
    q3: f64,
    lower_whisker: f64,
    upper_whisker: f64,
    lower_notch: f64,
    upper_notch: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{Series, DataFrame, PlotConfig, DataFormat};

    #[test]
    fn test_vertical_box_plot() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 15.0, 20.0]; // Include some outliers
        let series = Series {
            name: "Test Data".to_string(),
            data,
        };
        let dataframe = DataFrame {
            columns: vec![series],
            headers: None,
        };
        let config = PlotConfig {
            width: 40,
            height: 30,
            title: Some("Box Plot".to_string()),
            xlabel: Some("Category".to_string()),
            ylabel: Some("Values".to_string()),
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: Some("blue".to_string()),
            symbol: None,
        };

        let box_plot = BoxPlot::vertical();
        let result = box_plot.render(&dataframe, &config);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("Box Plot"));
        assert!(!output.is_empty());
    }

    #[test]
    fn test_horizontal_box_plot() {
        let data = vec![5.0, 7.0, 8.0, 9.0, 10.0, 12.0, 13.0, 14.0, 15.0, 16.0];
        let series = Series {
            name: "Horizontal Data".to_string(),
            data,
        };
        let dataframe = DataFrame {
            columns: vec![series],
            headers: None,
        };
        let config = PlotConfig {
            width: 50,
            height: 20,
            title: Some("Horizontal Box Plot".to_string()),
            xlabel: Some("Values".to_string()),
            ylabel: Some("Category".to_string()),
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: Some("green".to_string()),
            symbol: None,
        };

        let box_plot = BoxPlot::horizontal();
        let result = box_plot.render(&dataframe, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_box_plots() {
        let series1 = Series {
            name: "Group 1".to_string(),
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        };
        let series2 = Series {
            name: "Group 2".to_string(),
            data: vec![3.0, 4.0, 5.0, 6.0, 7.0],
        };
        let series3 = Series {
            name: "Group 3".to_string(),
            data: vec![2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0],
        };
        
        let dataframe = DataFrame {
            columns: vec![series1, series2, series3],
            headers: None,
        };
        
        let config = PlotConfig {
            width: 60,
            height: 30,
            title: Some("Multiple Box Plots".to_string()),
            xlabel: Some("Groups".to_string()),
            ylabel: Some("Values".to_string()),
            delimiter: ',',
            has_header: false,
            format: DataFormat::XYY,
            xlim: None,
            ylim: None,
            color: Some("magenta".to_string()),
            symbol: None,
        };

        let box_plot = BoxPlot::vertical().with_outlier_method(OutlierMethod::Tukey);
        let result = box_plot.render(&dataframe, &config);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("Multiple Box Plots"));
        assert!(!output.is_empty());
    }

    #[test]
    fn test_notched_box_plot() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let series = Series {
            name: "Notched Data".to_string(),
            data,
        };
        let dataframe = DataFrame {
            columns: vec![series],
            headers: None,
        };
        let config = PlotConfig {
            width: 40,
            height: 25,
            title: Some("Notched Box Plot".to_string()),
            xlabel: None,
            ylabel: Some("Values".to_string()),
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: Some("cyan".to_string()),
            symbol: None,
        };

        let notched_box = NotchedBoxPlot::new(Orientation::Vertical)
            .with_confidence_level(0.95);
        
        let result = notched_box.render(&dataframe, &config);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("Notched Box Plot"));
        assert!(!output.is_empty());
    }

    #[test]
    fn test_box_plot_without_outliers() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 100.0]; // 100.0 would be an outlier
        let series = Series {
            name: "No Outliers".to_string(),
            data,
        };
        let dataframe = DataFrame {
            columns: vec![series],
            headers: None,
        };
        let config = PlotConfig {
            width: 40,
            height: 20,
            title: Some("Box Plot No Outliers".to_string()),
            xlabel: None,
            ylabel: None,
            delimiter: ',',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: Some("red".to_string()),
            symbol: None,
        };

        let box_plot = BoxPlot::vertical().hide_outliers();
        let result = box_plot.render(&dataframe, &config);
        assert!(result.is_ok());
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

        let box_plot = BoxPlot::vertical();
        let result = box_plot.render(&dataframe, &config);
        assert!(result.is_err());
    }
}