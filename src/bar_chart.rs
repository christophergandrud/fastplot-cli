use crate::coordinates::{NumericCoordinate, DataBounds, CoordinateTransformer, CategoricalTransformer};
use crate::layout::LayoutEngine;
use crate::layered_canvas::{LayeredCanvas, RenderPriority};
use crate::data::{Dataset, DataPoint};
use crate::ticks::CategoricalTickGenerator;

pub struct BarChart {
    width: usize,
    height: usize,
    data: Vec<DataPoint>,
    title: String,
    x_label: String,
    y_label: String,
    bar_char: char,
    bar_width: usize,
    is_categorical: bool,
    categories: Vec<String>,
}

impl BarChart {
    pub fn new(dataset: &Dataset, title: &str, width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: dataset.points.clone(),
            title: title.to_string(),
            x_label: dataset.x_label.clone(),
            y_label: dataset.y_label.clone(),
            bar_char: '█',
            bar_width: 1,
            is_categorical: dataset.is_categorical,
            categories: dataset.categories.clone(),
        }
    }

    pub fn with_bar_char(mut self, bar_char: char) -> Self {
        self.bar_char = bar_char;
        self
    }

    pub fn with_bar_width(mut self, bar_width: usize) -> Self {
        self.bar_width = bar_width.max(1);
        self
    }

    pub fn render(&self, color: Option<&str>) -> String {
        if self.data.is_empty() {
            return format!("{}\n\nNo data to plot\n", self.title);
        }

        if self.is_categorical {
            self.render_categorical(color)
        } else {
            self.render_numeric(color)
        }
    }
    
    fn render_numeric(&self, color: Option<&str>) -> String {
        // Convert to coordinate points for compatibility
        let coord_points: Vec<NumericCoordinate> = self.data.iter()
            .filter_map(|p| {
                if let DataPoint::Numeric(x, y) = p {
                    Some(NumericCoordinate::new(*x, *y))
                } else {
                    None
                }
            })
            .collect();
            
        if coord_points.is_empty() {
            return format!("{}\n\nNo numeric data to plot\n", self.title);
        }

        // Sort data by x coordinate for consistent bar ordering
        let mut sorted_data = coord_points.clone();
        sorted_data.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

        // Calculate bounds with special handling for bar charts
        let bounds = self.calculate_numeric_bounds(&sorted_data);
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
        self.draw_numeric_ticks(&mut canvas, &layout);
        
        // Draw bars
        self.draw_numeric_bars(&mut canvas, &sorted_data, &transformer, color);
        
        // Flatten layers and format output
        self.format_output(&canvas, &layout)
    }
    
    fn render_categorical(&self, color: Option<&str>) -> String {
        // Calculate bounds for categorical data
        let bounds = self.calculate_categorical_bounds();
        let layout_engine = LayoutEngine::new(self.width, self.height);
        let layout = layout_engine.calculate_layout(&bounds);
        
        // Create categorical transformer
        let transformer = CategoricalTransformer::new(
            &self.categories,
            bounds,
            self.width,
            self.height,
            layout.margins.clone(),
        );
        
        // Create layered canvas
        let mut canvas = LayeredCanvas::new(self.width, self.height);
        
        // Draw axes and ticks
        self.draw_axes(&mut canvas, &layout);
        self.draw_categorical_ticks(&mut canvas, &layout, &transformer);
        
        // Draw bars
        self.draw_categorical_bars(&mut canvas, &transformer, color);
        
        // Flatten layers and format output
        self.format_output(&canvas, &layout)
    }

    fn draw_numeric_bars(&self, canvas: &mut LayeredCanvas, data: &[NumericCoordinate], transformer: &CoordinateTransformer, color: Option<&str>) {
        let bar_layer = canvas.get_layer(RenderPriority::Lines);
        
        // Calculate baseline (usually y=0, but handle cases where all values are positive/negative)
        let baseline_y = if data.iter().any(|p| p.y < 0.0) && data.iter().any(|p| p.y > 0.0) {
            0.0 // Mixed positive/negative values, use y=0 as baseline
        } else if data.iter().all(|p| p.y >= 0.0) {
            data.iter().map(|p| p.y).fold(f64::INFINITY, f64::min).min(0.0) // All positive, use min or 0
        } else {
            data.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max).max(0.0) // All negative, use max or 0
        };

        for point in data {
            // Transform the data point to screen coordinates
            if let Some(screen_point) = transformer.data_to_screen(*point) {
                // Transform baseline to screen coordinates
                let baseline_point = NumericCoordinate::new(point.x, baseline_y);
                if let Some(baseline_screen) = transformer.data_to_screen(baseline_point) {
                    
                    // Calculate bar dimensions
                    let bar_top = screen_point.row.min(baseline_screen.row);
                    let bar_bottom = screen_point.row.max(baseline_screen.row);
                    let bar_height = if bar_bottom > bar_top { bar_bottom - bar_top } else { 1 };
                    
                    // Calculate bar position (center around the x coordinate)
                    let bar_left = if screen_point.col >= self.bar_width / 2 {
                        screen_point.col - self.bar_width / 2
                    } else {
                        0
                    };
                    
                    // Draw the bar
                    for col in bar_left..bar_left + self.bar_width {
                        for row in bar_top..bar_top + bar_height {
                            bar_layer.draw_point_with_color(col, row, self.bar_char, color);
                        }
                    }
                }
            }
        }
    }

    fn calculate_numeric_bounds(&self, data: &[NumericCoordinate]) -> DataBounds {
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
        
        // For bar charts, we often want to include 0 in the y-range
        let actual_min_y = min_y.min(0.0);
        let actual_max_y = max_y.max(0.0);
        
        // Add padding
        let x_range = max_x - min_x;
        let y_range = actual_max_y - actual_min_y;
        let x_padding = if x_range > 0.0 { x_range * 0.1 } else { 1.0 };
        let y_padding = if y_range > 0.0 { y_range * 0.1 } else { 1.0 };
        
        DataBounds {
            min_x: min_x - x_padding,
            max_x: max_x + x_padding,
            min_y: actual_min_y - y_padding,
            max_y: actual_max_y + y_padding,
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

    fn draw_numeric_ticks(&self, canvas: &mut LayeredCanvas, layout: &crate::layout::Layout) {
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
    
    fn calculate_categorical_bounds(&self) -> DataBounds {
        if self.categories.is_empty() || self.data.is_empty() {
            return DataBounds {
                min_x: 0.0,
                max_x: 1.0,
                min_y: 0.0,
                max_y: 1.0,
            };
        }

        // X bounds are determined by category count
        let min_x = 0.0;
        let max_x = (self.categories.len() - 1) as f64;
        
        // Y bounds from data values
        let min_y = self.data.iter().map(|p| p.y()).fold(f64::INFINITY, f64::min);
        let max_y = self.data.iter().map(|p| p.y()).fold(f64::NEG_INFINITY, f64::max);
        
        // For bar charts, we often want to include 0 in the y-range
        let actual_min_y = min_y.min(0.0);
        let actual_max_y = max_y.max(0.0);
        
        // Add padding
        let x_range = max_x - min_x;
        let y_range = actual_max_y - actual_min_y;
        let x_padding = if x_range > 0.0 { x_range * 0.1 } else { 1.0 };
        let y_padding = if y_range > 0.0 { y_range * 0.1 } else { 1.0 };
        
        DataBounds {
            min_x: min_x - x_padding,
            max_x: max_x + x_padding,
            min_y: actual_min_y - y_padding,
            max_y: actual_max_y + y_padding,
        }
    }
    
    fn draw_categorical_bars(&self, canvas: &mut LayeredCanvas, transformer: &CategoricalTransformer, color: Option<&str>) {
        let bar_layer = canvas.get_layer(RenderPriority::Lines);
        
        // Calculate baseline (usually y=0)
        let baseline_y = if self.data.iter().any(|p| p.y() < 0.0) && self.data.iter().any(|p| p.y() > 0.0) {
            0.0 // Mixed positive/negative values, use y=0 as baseline
        } else if self.data.iter().all(|p| p.y() >= 0.0) {
            self.data.iter().map(|p| p.y()).fold(f64::INFINITY, f64::min).min(0.0) // All positive, use min or 0
        } else {
            self.data.iter().map(|p| p.y()).fold(f64::NEG_INFINITY, f64::max).max(0.0) // All negative, use max or 0
        };

        for point in &self.data {
            // Transform the data point to screen coordinates
            if let Some(screen_point) = transformer.data_to_screen(point) {
                // Create baseline point for the same category
                let baseline_point = match point {
                    DataPoint::Categorical(category, _) => DataPoint::Categorical(category.clone(), baseline_y),
                    DataPoint::Numeric(x, _) => DataPoint::Numeric(*x, baseline_y),
                };
                
                if let Some(baseline_screen) = transformer.data_to_screen(&baseline_point) {
                    // Calculate bar dimensions
                    let bar_top = screen_point.row.min(baseline_screen.row);
                    let bar_bottom = screen_point.row.max(baseline_screen.row);
                    let bar_height = if bar_bottom > bar_top { bar_bottom - bar_top } else { 1 };
                    
                    // Calculate bar position (center around the x coordinate)
                    let bar_left = if screen_point.col >= self.bar_width / 2 {
                        screen_point.col - self.bar_width / 2
                    } else {
                        0
                    };
                    
                    // Draw the bar
                    for col in bar_left..bar_left + self.bar_width {
                        for row in bar_top..bar_top + bar_height {
                            bar_layer.draw_point_with_color(col, row, self.bar_char, color);
                        }
                    }
                }
            }
        }
    }
    
    fn draw_categorical_ticks(&self, canvas: &mut LayeredCanvas, layout: &crate::layout::Layout, transformer: &CategoricalTransformer) {
        let area = &layout.plot_area;
        
        // Generate categorical tick positions
        let positions: Vec<f64> = self.categories.iter()
            .filter_map(|cat| transformer.get_category_position(cat))
            .collect();
            
        let tick_generator = CategoricalTickGenerator::default();
        let ticks = tick_generator.generate_categorical_ticks(&self.categories, &positions);
        
        // Draw X ticks
        {
            let axes_layer = canvas.get_layer(RenderPriority::Axes);
            for tick in &ticks {
                // Convert data position to screen position
                let coord_point = NumericCoordinate::new(tick.value, 0.0);
                let coord_transformer = CoordinateTransformer::new(
                    self.calculate_categorical_bounds(),
                    self.width,
                    self.height,
                    layout.margins.clone(),
                );
                if let Some(screen_point) = coord_transformer.data_to_screen(coord_point) {
                    axes_layer.draw_point(screen_point.col, area.top + area.height, '┬');
                }
            }
            
            // Draw Y ticks (same as numeric)
            for (row, _tick) in &layout.y_ticks {
                axes_layer.draw_point(area.left.saturating_sub(1), *row, '┤');
            }
        }
        
        // Draw X tick labels
        {
            let label_layer = canvas.get_layer(RenderPriority::Labels);
            for tick in &ticks {
                // Convert data position to screen position
                let coord_point = NumericCoordinate::new(tick.value, 0.0);
                let coord_transformer = CoordinateTransformer::new(
                    self.calculate_categorical_bounds(),
                    self.width,
                    self.height,
                    layout.margins.clone(),
                );
                if let Some(screen_point) = coord_transformer.data_to_screen(coord_point) {
                    let label_start = screen_point.col.saturating_sub(tick.label.len() / 2);
                    label_layer.draw_text(label_start, area.top + area.height + 1, &tick.label);
                }
            }
        }
    }
    
    fn format_output(&self, canvas: &LayeredCanvas, layout: &crate::layout::Layout) -> String {
        let final_canvas = canvas.flatten();
        let mut output = String::new();
        output.push_str(&self.title);
        output.push_str("\n\n");
        output.push_str(&self.y_label);
        output.push('\n');
        output.push_str(&self.render_with_y_labels(&final_canvas, layout));
        output.push_str(&crate::layout::format_x_axis_label(&self.x_label, layout));
        
        output
    }
}

pub fn render_bar_chart(dataset: &Dataset, title: &str, bar_char: char, bar_width: usize, color: Option<&str>) -> String {
    let plot = BarChart::new(dataset, title, 80, 24)
        .with_bar_char(bar_char)
        .with_bar_width(bar_width);
    plot.render(color)
}