use crate::axis::AxisConfig;
use crate::data::{DataPoint, Dataset};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    grid: Vec<Vec<char>>,
    x_axis: AxisConfig,
    y_axis: AxisConfig,
    left_margin: usize,
}

impl Canvas {
    pub fn new(dataset: &Dataset, title: &str) -> Self {
        let x_axis = AxisConfig::from_data(&dataset.points, 'x');
        let y_axis = AxisConfig::from_data(&dataset.points, 'y');
        
        // Calculate left margin based on y-axis labels
        let max_y_label_width = y_axis.get_ticks().iter()
            .map(|&tick| format_tick(tick).len())
            .max()
            .unwrap_or(3);
        let left_margin = max_y_label_width + 1; // +1 for tick symbol
        
        let width = left_margin + 25; // Plot area width
        let height = 12; // Plot area height
        
        let mut grid = vec![vec![' '; width]; height];
        
        let mut canvas = Self {
            width,
            height,
            grid,
            x_axis,
            y_axis,
            left_margin,
        };
        
        canvas.draw_title(title);
        canvas.draw_axis_labels(&dataset.x_label, &dataset.y_label);
        canvas.draw_axes();
        canvas
    }
    
    pub fn plot_point(&mut self, point: &DataPoint, symbol: char) {
        let plot_width = self.width - self.left_margin;
        let plot_height = self.height - 2; // -2 for x-axis and spacing
        
        let x_pos = self.x_axis.data_to_position(point.x, plot_width);
        let y_pos = self.y_axis.data_to_position(point.y, plot_height);
        
        // Convert to grid coordinates (y inverted, add margins)
        let grid_x = self.left_margin + x_pos;
        let grid_y = plot_height - 1 - y_pos + 1; // +1 for title spacing
        
        if grid_x < self.width && grid_y < self.height - 2 {
            self.grid[grid_y][grid_x] = symbol;
        }
    }
    
    fn draw_title(&mut self, title: &str) {
        // Title goes above the plot area
        // We'll handle this in the render method
    }
    
    fn draw_axis_labels(&mut self, x_label: &str, y_label: &str) {
        // Labels will be handled in render method
    }
    
    fn draw_axes(&mut self) {
        let plot_height = self.height - 2;
        let plot_width = self.width - self.left_margin;
        
        // Draw y-axis ticks and labels
        let y_ticks = self.y_axis.get_ticks();
        for &tick in &y_ticks {
            let y_pos = self.y_axis.data_to_position(tick, plot_height);
            let grid_y = plot_height - 1 - y_pos + 1;
            
            if grid_y < self.height - 2 {
                // Place tick symbol
                self.grid[grid_y][self.left_margin - 1] = '┼';
            }
        }
        
        // Draw vertical line for y-axis
        for y in 2..self.height - 2 {
            if self.grid[y][self.left_margin - 1] != '┼' {
                self.grid[y][self.left_margin - 1] = '│';
            }
        }
        
        // Draw x-axis line with ticks
        let x_axis_row = self.height - 2;
        let x_ticks = self.x_axis.get_ticks();
        
        // Draw horizontal line
        for x in self.left_margin..self.width {
            self.grid[x_axis_row][x] = '─';
        }
        
        // Draw x-axis ticks
        for &tick in &x_ticks {
            let x_pos = self.x_axis.data_to_position(tick, plot_width);
            let grid_x = self.left_margin + x_pos;
            
            if grid_x < self.width {
                self.grid[x_axis_row][grid_x] = '┼';
            }
        }
    }
    
    pub fn render(&self, title: &str, x_label: &str, y_label: &str) -> String {
        let mut output = String::new();
        
        // Title
        output.push_str(title);
        output.push_str("\n\n");
        
        // Y-axis label
        output.push_str(y_label);
        output.push('\n');
        
        // Plot area with y-axis labels
        let y_ticks = self.y_axis.get_ticks();
        let plot_height = self.height - 2;
        
        for (i, row) in self.grid.iter().enumerate().take(self.height - 2).skip(1) {
            // Find corresponding y tick for this row
            let grid_y = i;
            let y_pos = plot_height - 1 - grid_y + 1;
            let data_y = y_pos as f64 / (plot_height - 1) as f64;
            let actual_y = self.y_axis.min + data_y * (self.y_axis.max - self.y_axis.min);
            
            // Check if this row has a y-tick
            let tick_label = y_ticks.iter()
                .find(|&&tick| {
                    let tick_pos = self.y_axis.data_to_position(tick, plot_height);
                    let tick_grid_y = plot_height - 1 - tick_pos + 1;
                    tick_grid_y == grid_y
                })
                .map(|&tick| format_tick(tick));
            
            if let Some(label) = tick_label {
                output.push_str(&format!("{:>width$} ", label, width = self.left_margin - 1));
            } else {
                output.push_str(&format!("{:>width$} ", "", width = self.left_margin - 1));
            }
            
            // Plot content
            for &ch in row.iter().skip(self.left_margin) {
                output.push(ch);
            }
            output.push('\n');
        }
        
        // X-axis row
        output.push_str(&format!("{:>width$} ", "", width = self.left_margin - 1));
        for &ch in self.grid[self.height - 2].iter().skip(self.left_margin) {
            output.push(ch);
        }
        output.push('\n');
        
        // X-axis labels
        let x_ticks = self.x_axis.get_ticks();
        let mut x_label_line = format!("{:>width$} ", "", width = self.left_margin - 1);
        
        for (i, &tick) in x_ticks.iter().enumerate() {
            let label = format_tick(tick);
            if i == 0 {
                x_label_line.push_str(&label);
            } else {
                // Calculate spacing to next label
                let spacing = (self.width - self.left_margin) / (x_ticks.len() - 1);
                let padding = spacing.saturating_sub(label.len());
                x_label_line.push_str(&" ".repeat(padding));
                x_label_line.push_str(&label);
            }
        }
        output.push_str(&x_label_line);
        output.push('\n');
        
        // X-axis label centered
        let x_label_padding = (self.width - x_label.len()) / 2;
        output.push_str(&format!("{:>width$}{}", "", x_label, width = x_label_padding));
        
        output
    }
}

fn format_tick(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        format!("{:.1}", value)
    }
}