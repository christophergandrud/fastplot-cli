use crate::coordinates::{NumericCoordinate, DataBounds, CoordinateTransformer};
use crate::layout::LayoutEngine;
use crate::layered_canvas::{LayeredCanvas, RenderPriority};
use crate::line_style::LineStyle;
use crate::line_drawing::LineRenderer;
use crate::data::{Dataset, DataPoint};

pub struct LinePlot {
    width: usize,
    height: usize,
    data: Vec<DataPoint>,
    style: LineStyle,
    title: String,
    x_label: String,
    y_label: String,
}

impl LinePlot {
    pub fn new(dataset: &Dataset, title: &str, width: usize, height: usize) -> Self {
        let data = dataset.points.clone();
        
        Self {
            width,
            height,
            data,
            style: LineStyle::default(),
            title: title.to_string(),
            x_label: dataset.x_label.clone(),
            y_label: dataset.y_label.clone(),
        }
    }

    pub fn with_style(mut self, style: LineStyle) -> Self {
        self.style = style;
        self
    }


    pub fn render(&self, color: Option<&str>) -> String {
        if self.data.is_empty() {
            return format!("{}\n\nNo data to plot\n", self.title);
        }

        // Sort data by x coordinate for proper line connections
        let mut sorted_data = self.data.clone();
        sorted_data.sort_by(|a, b| {
            let a_x = match a {
                DataPoint::Numeric(x, _) => *x,
                DataPoint::Categorical(_, _) => 0.0, // Categorical not supported in line plots
            };
            let b_x = match b {
                DataPoint::Numeric(x, _) => *x,
                DataPoint::Categorical(_, _) => 0.0,
            };
            a_x.partial_cmp(&b_x).unwrap()
        });

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
            .filter_map(|p| transformer.transform_data_point(p))
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
        output.push_str(&crate::layout::format_x_axis_label(&self.x_label, &layout));
        
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
        // Convert to numeric coordinates for bounds calculation
        let numeric_coords: Vec<NumericCoordinate> = data
            .iter()
            .filter_map(|p| match p {
                DataPoint::Numeric(x, y) => Some(NumericCoordinate::new(*x, *y)),
                DataPoint::Categorical(_, _) => None, // Line plots don't support categorical data
            })
            .collect();

        if numeric_coords.is_empty() {
            return DataBounds {
                min_x: 0.0,
                max_x: 1.0,
                min_y: 0.0,
                max_y: 1.0,
            };
        }

        let min_x = numeric_coords.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
        let max_x = numeric_coords.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
        let min_y = numeric_coords.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
        let max_y = numeric_coords.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
        
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
        
        // Calculate the maximum width of y-axis labels for consistent alignment
        let max_label_width = layout.y_ticks.iter()
            .map(|(_, tick)| tick.label.len())
            .max()
            .unwrap_or(0);
        
        let mut output = String::new();
        let plot_start = layout.plot_area.top;
        let plot_end = layout.plot_area.top + layout.plot_area.height;
        
        for row in plot_start..=plot_end {
            let y_label = layout.y_ticks.iter()
                .find(|(tick_row, _)| *tick_row == row)
                .map(|(_, tick)| &tick.label);
            
            if let Some(label) = y_label {
                output.push_str(&format!("{:>width$} ", label, width = max_label_width));
            } else {
                output.push_str(&" ".repeat(max_label_width + 1));
            }
            
            if row < lines.len() {
                output.push_str(lines[row]);
            }
            output.push('\n');
        }
        
        // Add x-axis labels below the plot
        let x_axis_row = plot_end + 1;
        if x_axis_row < lines.len() {
            output.push_str(&" ".repeat(max_label_width + 1));
            output.push_str(lines[x_axis_row]);
            output.push('\n');
        }
        
        output
    }
}


pub fn render_line_plot(dataset: &Dataset, title: &str, style: LineStyle, color: Option<&str>) -> String {
    let plot = LinePlot::new(dataset, title, 80, 24).with_style(style);
    plot.render(color)
}