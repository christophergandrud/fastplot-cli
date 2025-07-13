use crate::coordinates::{DataPoint, DataBounds, CoordinateTransformer};
use crate::layout::LayoutEngine;
use crate::layered_canvas::{LayeredCanvas, RenderPriority};
use crate::line_style::LineStyle;
use crate::line_drawing::LineRenderer;
use crate::data::Dataset;

pub struct LinePlot {
    width: usize,
    height: usize,
    data: Vec<DataPoint>,
    style: LineStyle,
    title: String,
    x_label: String,
    y_label: String,
    connect_gaps: bool,
}

impl LinePlot {
    pub fn new(dataset: &Dataset, title: &str, width: usize, height: usize) -> Self {
        let data = dataset.points.iter().map(|p| DataPoint::from(p.clone())).collect();
        
        Self {
            width,
            height,
            data,
            style: LineStyle::default(),
            title: title.to_string(),
            x_label: dataset.x_label.clone(),
            y_label: dataset.y_label.clone(),
            connect_gaps: true,
        }
    }

    pub fn with_style(mut self, style: LineStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_connect_gaps(mut self, connect: bool) -> Self {
        self.connect_gaps = connect;
        self
    }

    pub fn add_point(&mut self, x: f64, y: f64) {
        self.data.push(DataPoint { x, y });
    }

    pub fn add_points(&mut self, points: &[(f64, f64)]) {
        for (x, y) in points {
            self.add_point(*x, *y);
        }
    }

    pub fn render(&self, color: Option<&str>) -> String {
        if self.data.is_empty() {
            return format!("{}\n\nNo data to plot\n", self.title);
        }

        // Sort data by x coordinate for proper line connections
        let mut sorted_data = self.data.clone();
        sorted_data.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

        // Calculate bounds and layout
        let bounds = self.calculate_bounds_with_padding(&sorted_data);
        let layout_engine = LayoutEngine::new(self.width, self.height);
        let layout = layout_engine.calculate_layout(&bounds);
        
        // Create transformer
        let transformer = CoordinateTransformer::new(
            bounds,
            self.width,
            self.height,
            layout.margins.clone(),
        );
        
        // Create layered canvas
        let mut canvas = LayeredCanvas::new(self.width, self.height);
        
        // Draw axes and ticks
        self.draw_axes(&mut canvas, &layout);
        self.draw_ticks(&mut canvas, &layout);
        
        // Convert data points to screen coordinates
        let screen_points: Vec<_> = sorted_data
            .iter()
            .filter_map(|p| transformer.data_to_screen(*p))
            .collect();
        
        // Draw connecting lines
        if self.style.show_lines && screen_points.len() > 1 {
            self.draw_lines(&mut canvas, &screen_points, color);
        }
        
        // Draw data points (on top of lines)
        if self.style.show_points {
            self.draw_points(&mut canvas, &screen_points, color);
        }
        
        // Flatten layers and format output
        let final_canvas = canvas.flatten();
        let mut output = String::new();
        output.push_str(&self.title);
        output.push_str("\n\n");
        output.push_str(&self.y_label);
        output.push('\n');
        output.push_str(&self.render_with_y_labels(&final_canvas, &layout));
        output.push_str(&format!("\n{}\n", self.x_label));
        
        output
    }

    fn draw_lines(&self, canvas: &mut LayeredCanvas, points: &[crate::coordinates::ScreenPoint], color: Option<&str>) {
        let line_layer = canvas.get_layer(RenderPriority::Lines);
        
        for i in 0..points.len() - 1 {
            let start = &points[i];
            let end = &points[i + 1];
            
            // Skip if points are the same
            if start.col == end.col && start.row == end.row {
                continue;
            }
            
            // Get line points
            let line_points = LineRenderer::bresenham_line(start.clone(), end.clone());
            
            // Draw line, but skip the actual data points if we're showing them
            for point in line_points {
                // Skip if this is one of the data points and we're showing points
                let is_data_point = points.iter().any(|p| p.col == point.col && p.row == point.row);
                if !is_data_point || !self.style.show_points {
                    line_layer.draw_point_with_color(point.col, point.row, self.style.line_char, color);
                }
            }
        }
    }

    fn draw_points(&self, canvas: &mut LayeredCanvas, points: &[crate::coordinates::ScreenPoint], color: Option<&str>) {
        let point_layer = canvas.get_layer(RenderPriority::Points);
        
        for point in points {
            point_layer.draw_point_with_color(point.col, point.row, self.style.point_char, color);
        }
    }

    fn calculate_bounds_with_padding(&self, data: &[DataPoint]) -> DataBounds {
        if data.is_empty() {
            return DataBounds {
                min_x: 0.0,
                max_x: 1.0,
                min_y: 0.0,
                max_y: 1.0,
            };
        }

        let min_x = data.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
        let max_x = data.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
        let min_y = data.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
        let max_y = data.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
        
        let x_range = max_x - min_x;
        let y_range = max_y - min_y;
        let x_padding = if x_range > 0.0 { x_range * 0.1 } else { 1.0 };
        let y_padding = if y_range > 0.0 { y_range * 0.1 } else { 1.0 };
        
        DataBounds {
            min_x: min_x - x_padding,
            max_x: max_x + x_padding,
            min_y: min_y - y_padding,
            max_y: max_y + y_padding,
        }
    }

    fn draw_axes(&self, canvas: &mut LayeredCanvas, layout: &crate::layout::Layout) {
        let axes_layer = canvas.get_layer(RenderPriority::Axes);
        let area = &layout.plot_area;
        
        // Draw Y axis
        axes_layer.draw_vertical_line(area.left.saturating_sub(1), area.top, area.top + area.height, '│');
        
        // Draw X axis
        axes_layer.draw_line(area.top + area.height, area.left.saturating_sub(1), area.left + area.width, '─');
        
        // Draw corner
        axes_layer.draw_point(area.left.saturating_sub(1), area.top + area.height, '└');
    }

    fn draw_ticks(&self, canvas: &mut LayeredCanvas, layout: &crate::layout::Layout) {
        let area = &layout.plot_area;
        
        // Draw X ticks
        {
            let axes_layer = canvas.get_layer(RenderPriority::Axes);
            for (col, _tick) in &layout.x_ticks {
                axes_layer.draw_point(*col, area.top + area.height, '┬');
            }
            
            // Draw Y ticks
            for (row, _tick) in &layout.y_ticks {
                axes_layer.draw_point(area.left.saturating_sub(1), *row, '┤');
            }
        }
        
        // Draw X tick labels
        {
            let label_layer = canvas.get_layer(RenderPriority::Labels);
            for (col, tick) in &layout.x_ticks {
                let label_start = col.saturating_sub(tick.label.len() / 2);
                label_layer.draw_text(label_start, area.top + area.height + 1, &tick.label);
            }
        }
    }

    fn render_with_y_labels(&self, canvas: &crate::layered_canvas::Canvas, layout: &crate::layout::Layout) -> String {
        // Use the canvas's to_string method and add y-labels
        let canvas_output = canvas.to_string();
        let lines: Vec<&str> = canvas_output.lines().collect();
        
        let mut output = String::new();
        let plot_start = layout.plot_area.top;
        let plot_end = layout.plot_area.top + layout.plot_area.height;
        
        for row in plot_start..=plot_end {
            let y_label = layout.y_ticks.iter()
                .find(|(tick_row, _)| *tick_row == row)
                .map(|(_, tick)| &tick.label);
            
            if let Some(label) = y_label {
                output.push_str(&format!("{:>3} ", label));
            } else {
                output.push_str("    ");
            }
            
            if row < lines.len() {
                output.push_str(lines[row]);
            }
            output.push('\n');
        }
        
        // Add x-axis labels below the plot
        let x_axis_row = plot_end + 1;
        if x_axis_row < lines.len() {
            output.push_str("    ");
            output.push_str(lines[x_axis_row]);
            output.push('\n');
        }
        
        output
    }
}

fn apply_color(ch: char, color_str: &str) -> Option<String> {
    use colored::Colorize;
    
    let ch_str = ch.to_string();
    
    if color_str.starts_with('#') && color_str.len() == 7 {
        if let Ok(r) = u8::from_str_radix(&color_str[1..3], 16) {
            if let Ok(g) = u8::from_str_radix(&color_str[3..5], 16) {
                if let Ok(b) = u8::from_str_radix(&color_str[5..7], 16) {
                    return Some(ch_str.truecolor(r, g, b).to_string());
                }
            }
        }
    }
    
    match color_str.to_lowercase().as_str() {
        "red" => Some(ch_str.red().to_string()),
        "green" => Some(ch_str.green().to_string()),
        "blue" => Some(ch_str.blue().to_string()),
        "yellow" => Some(ch_str.yellow().to_string()),
        "magenta" | "purple" => Some(ch_str.magenta().to_string()),
        "cyan" => Some(ch_str.cyan().to_string()),
        "white" => Some(ch_str.white().to_string()),
        "black" => Some(ch_str.black().to_string()),
        "bright_red" => Some(ch_str.bright_red().to_string()),
        "bright_green" => Some(ch_str.bright_green().to_string()),
        "bright_blue" => Some(ch_str.bright_blue().to_string()),
        "bright_yellow" => Some(ch_str.bright_yellow().to_string()),
        "bright_magenta" | "bright_purple" => Some(ch_str.bright_magenta().to_string()),
        "bright_cyan" => Some(ch_str.bright_cyan().to_string()),
        _ => None,
    }
}

pub fn render_line_plot(dataset: &Dataset, title: &str, style: LineStyle, color: Option<&str>) -> String {
    let plot = LinePlot::new(dataset, title, 80, 24).with_style(style);
    plot.render(color)
}