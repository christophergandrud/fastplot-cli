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
        
        let grid = vec![vec![' '; width]; height];
        
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
        // Use same coordinate system as draw_axes
        let plot_start_row = 1;
        let plot_end_row = self.height - 2;
        let plot_height = plot_end_row - plot_start_row;
        let plot_width = self.width - self.left_margin;
        
        let x_pos = self.x_axis.data_to_position(point.x, plot_width);
        let y_pos = self.y_axis.data_to_position(point.y, plot_height);
        
        // Convert to grid coordinates (y inverted to match screen coordinates)
        let grid_x = self.left_margin + x_pos;
        let grid_y = plot_start_row + (plot_height - 1 - y_pos);
        
        if grid_x < self.width && grid_y >= plot_start_row && grid_y < plot_end_row {
            self.grid[grid_y][grid_x] = symbol;
        }
    }
    
    fn draw_title(&mut self, _title: &str) {
        // Title goes above the plot area
        // We'll handle this in the render method
    }
    
    fn draw_axis_labels(&mut self, _x_label: &str, _y_label: &str) {
        // Labels will be handled in render method
    }
    
    fn draw_axes(&mut self) {
        // Define plot area boundaries
        let plot_start_row = 1; // Start after title/y-label
        let plot_end_row = self.height - 2; // End before x-axis
        let plot_height = plot_end_row - plot_start_row;
        let plot_width = self.width - self.left_margin;
        
        // Draw y-axis ticks at their correct positions
        let y_ticks = self.y_axis.get_ticks();
        for &tick in &y_ticks {
            let y_pos = self.y_axis.data_to_position(tick, plot_height);
            let grid_y = plot_start_row + (plot_height - 1 - y_pos);
            
            if grid_y >= plot_start_row && grid_y < plot_end_row {
                // Place tick symbol at the y-axis position
                self.grid[grid_y][self.left_margin - 1] = '┼';
            }
        }
        
        // Draw vertical line for y-axis (connecting all plot rows)
        for y in plot_start_row..plot_end_row {
            if self.grid[y][self.left_margin - 1] == ' ' {
                self.grid[y][self.left_margin - 1] = '│';
            }
        }
        
        // Draw x-axis line with ticks
        let x_axis_row = plot_end_row;
        let x_ticks = self.x_axis.get_ticks();
        
        // Draw horizontal line from y-axis to end of plot
        for x in (self.left_margin - 1)..self.width {
            self.grid[x_axis_row][x] = '─';
        }
        
        // Draw x-axis ticks at their correct positions
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
        let plot_start_row = 1;
        let plot_end_row = self.height - 2;
        let plot_height = plot_end_row - plot_start_row;
        
        for (i, row) in self.grid.iter().enumerate().take(plot_end_row).skip(plot_start_row) {
            // Check if this row has a y-tick label
            let tick_label = y_ticks.iter()
                .find(|&&tick| {
                    let tick_pos = self.y_axis.data_to_position(tick, plot_height);
                    let tick_grid_y = plot_start_row + (plot_height - 1 - tick_pos);
                    tick_grid_y == i
                })
                .map(|&tick| format_tick(tick));
            
            if let Some(label) = tick_label {
                output.push_str(&format!("{:>width$} ", label, width = self.left_margin - 1));
            } else {
                output.push_str(&format!("{:>width$} ", "", width = self.left_margin - 1));
            }
            
            // Plot content - render the entire row including y-axis characters
            for &ch in row.iter().skip(self.left_margin - 1) {
                output.push(ch);
            }
            output.push('\n');
        }
        
        // X-axis row
        output.push_str(&format!("{:>width$} ", "", width = self.left_margin - 1));
        for &ch in self.grid[self.height - 2].iter().skip(self.left_margin - 1) {
            output.push(ch);
        }
        output.push('\n');
        
        // X-axis labels - position them to align with tick marks
        let x_ticks = self.x_axis.get_ticks();
        let plot_width = self.width - self.left_margin;
        let mut x_label_line = vec![' '; self.width];
        
        for &tick in &x_ticks {
            let label = format_tick(tick);
            let x_pos = self.x_axis.data_to_position(tick, plot_width);
            let grid_x = self.left_margin + x_pos;
            
            // Center the label on the tick position
            let label_start = grid_x.saturating_sub(label.len() / 2);
            if label_start + label.len() <= self.width {
                for (i, ch) in label.chars().enumerate() {
                    if label_start + i < self.width {
                        x_label_line[label_start + i] = ch;
                    }
                }
            }
        }
        
        // Convert to string and trim trailing spaces
        let x_label_str: String = x_label_line.iter().collect();
        output.push_str(&format!("{:>width$} ", "", width = self.left_margin - 1));
        output.push_str(x_label_str.trim_end());
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