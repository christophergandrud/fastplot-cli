#![allow(dead_code)]

use unicode_width::UnicodeWidthChar;
use std::fmt;
use crossterm::style::{Color, Stylize};

#[derive(Debug, Clone)]
pub struct Canvas {
    width: usize,
    height: usize,
    buffer: Vec<Vec<char>>,
    color_buffer: Vec<Vec<Option<Color>>>,
    x_range: (f64, f64),
    y_range: (f64, f64),
    title: Option<String>,
    xlabel: Option<String>,
    ylabel: Option<String>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![vec![' '; width]; height];
        let color_buffer = vec![vec![None; width]; height];
        Self {
            width,
            height,
            buffer,
            color_buffer,
            x_range: (0.0, 1.0),
            y_range: (0.0, 1.0),
            title: None,
            xlabel: None,
            ylabel: None,
        }
    }

    pub fn with_labels(
        width: usize, 
        height: usize, 
        title: Option<String>, 
        xlabel: Option<String>, 
        ylabel: Option<String>
    ) -> Self {
        let mut canvas = Self::new(width, height);
        canvas.title = title;
        canvas.xlabel = xlabel;
        canvas.ylabel = ylabel;
        canvas
    }

    pub fn set_ranges(&mut self, x_range: (f64, f64), y_range: (f64, f64)) {
        self.x_range = x_range;
        self.y_range = y_range;
    }

    pub fn plot_point(&mut self, x: f64, y: f64, symbol: char) {
        self.plot_point_with_color(x, y, symbol, None);
    }

    pub fn plot_point_with_color(&mut self, x: f64, y: f64, symbol: char, color: Option<Color>) {
        let canvas_x = self.data_to_canvas_x(x);
        let canvas_y = self.data_to_canvas_y(y);

        if canvas_x < self.width && canvas_y < self.height {
            // Handle Unicode width properly
            let char_width = symbol.width().unwrap_or(1);
            if canvas_x + char_width <= self.width {
                self.buffer[canvas_y][canvas_x] = symbol;
                self.color_buffer[canvas_y][canvas_x] = color;
                // Clear additional cells for wide characters
                for i in 1..char_width {
                    if canvas_x + i < self.width {
                        self.buffer[canvas_y][canvas_x + i] = ' ';
                        self.color_buffer[canvas_y][canvas_x + i] = None;
                    }
                }
            }
        }
    }

    pub fn plot_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, symbol: char) {
        self.plot_line_with_color(x1, y1, x2, y2, symbol, None);
    }

    pub fn plot_line_with_color(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, symbol: char, color: Option<Color>) {
        let cx1 = self.data_to_canvas_x(x1) as i32;
        let cy1 = self.data_to_canvas_y(y1) as i32;
        let cx2 = self.data_to_canvas_x(x2) as i32;
        let cy2 = self.data_to_canvas_y(y2) as i32;

        // Bresenham's line algorithm
        let dx = (cx2 - cx1).abs();
        let dy = -(cy2 - cy1).abs();
        let sx = if cx1 < cx2 { 1 } else { -1 };
        let sy = if cy1 < cy2 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = cx1;
        let mut y = cy1;

        loop {
            if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                self.buffer[y as usize][x as usize] = symbol;
                self.color_buffer[y as usize][x as usize] = color;
            }

            if x == cx2 && y == cy2 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn draw_axis(&mut self) {
        self.draw_axes_with_ticks(5, 5);
    }

    pub fn draw_axes_with_ticks(&mut self, x_ticks: usize, y_ticks: usize) {
        // Draw Y axis (left side)
        for y in 0..self.height {
            if y < self.buffer.len() && !self.buffer[y].is_empty() {
                self.buffer[y][0] = '│';
            }
        }

        // Draw X axis (bottom)
        if let Some(bottom_row) = self.buffer.last_mut() {
            for x in 0..self.width.min(bottom_row.len()) {
                bottom_row[x] = '─';
            }
            if !bottom_row.is_empty() {
                bottom_row[0] = '└';
            }
        }

        // Add X axis tick marks - align with data positioning
        if x_ticks > 0 && self.height > 0 {
            let y_pos = self.height - 1;
            let data_width = self.width.saturating_sub(1); // Available data width (excluding Y-axis column)
            
            for i in 0..x_ticks {
                // Match data positioning: start from column 1, distribute evenly across data width
                let x_pos = if x_ticks <= 1 {
                    1 + data_width / 2 // Center position
                } else {
                    1 + (i * (data_width - 1)) / (x_ticks - 1)
                };
                
                if x_pos < self.width && y_pos < self.buffer.len() {
                    self.buffer[y_pos][x_pos] = '┴';
                }
            }
        }

        // Add Y axis tick marks
        if y_ticks > 0 && !self.buffer.is_empty() {
            for i in 1..=y_ticks {
                let y_pos = (i * (self.height - 1)) / (y_ticks + 1);
                if y_pos < self.height && !self.buffer[y_pos].is_empty() {
                    self.buffer[y_pos][0] = '├';
                }
            }
        }
    }

    pub fn clear(&mut self) {
        for row in &mut self.buffer {
            for cell in row {
                *cell = ' ';
            }
        }
        for row in &mut self.color_buffer {
            for cell in row {
                *cell = None;
            }
        }
    }

    pub fn render(&self) -> String {
        self.render_colored(false)
    }

    pub fn render_colored(&self, use_colors: bool) -> String {
        let mut result = String::new();
        
        // Add title if present
        if let Some(ref title) = self.title {
            let padding = if title.len() < self.width {
                (self.width - title.len()) / 2
            } else {
                0
            };
            result.push_str(&" ".repeat(padding));
            if use_colors {
                result.push_str(&title.clone().bold().to_string());
            } else {
                result.push_str(title);
            }
            result.push('\n');
            result.push('\n');
        }
        
        // Render the main canvas buffer with colors
        for (row_idx, row) in self.buffer.iter().enumerate() {
            for (col_idx, &ch) in row.iter().enumerate() {
                if use_colors {
                    if let Some(color) = self.color_buffer[row_idx][col_idx] {
                        result.push_str(&ch.to_string().with(color).to_string());
                    } else {
                        result.push(ch);
                    }
                } else {
                    result.push(ch);
                }
            }
            result.push('\n');
        }
        
        // Add x-label if present
        if let Some(ref xlabel) = self.xlabel {
            let padding = if xlabel.len() < self.width {
                (self.width - xlabel.len()) / 2
            } else {
                0
            };
            result.push_str(&" ".repeat(padding));
            result.push_str(xlabel);
            result.push('\n');
        }
        
        result
    }

    pub fn render_with_labels(&self, show_values: bool) -> String {
        let mut result = String::new();
        
        // Add title if present
        if let Some(ref title) = self.title {
            let padding = if title.len() < self.width {
                (self.width - title.len()) / 2
            } else {
                0
            };
            result.push_str(&" ".repeat(padding));
            result.push_str(title);
            result.push('\n');
            result.push('\n');
        }
        
        // Y-axis label (rotated, positioned on the left)
        let mut canvas_lines: Vec<String> = Vec::new();
        
        for row in &self.buffer {
            let line: String = row.iter().collect();
            canvas_lines.push(line);
        }
        
        // Add Y-axis values if requested
        if show_values {
            let y_step = (self.y_range.1 - self.y_range.0) / (self.height as f64 - 1.0);
            for (i, line) in canvas_lines.iter().enumerate() {
                let y_value = self.y_range.1 - (i as f64 * y_step);
                result.push_str(&format!("{:8.2} {}\n", y_value, line));
            }
            
            // Add X-axis values
            if self.width > 10 {
                result.push_str("         ");
                let x_step = (self.x_range.1 - self.x_range.0) / (self.width as f64 - 1.0);
                for i in (0..self.width).step_by(self.width / 5) {
                    let x_value = self.x_range.0 + (i as f64 * x_step);
                    result.push_str(&format!("{:8.2} ", x_value));
                }
                result.push('\n');
            }
        } else {
            for line in canvas_lines {
                result.push_str(&line);
                result.push('\n');
            }
        }
        
        // Add x-label if present
        if let Some(ref xlabel) = self.xlabel {
            let padding = if xlabel.len() < self.width {
                (self.width - xlabel.len()) / 2
            } else {
                0
            };
            result.push_str(&" ".repeat(padding));
            result.push_str(xlabel);
            result.push('\n');
        }
        
        result
    }

    fn data_to_canvas_x(&self, x: f64) -> usize {
        let normalized = (x - self.x_range.0) / (self.x_range.1 - self.x_range.0);
        // Reserve column 0 for Y-axis, map data to columns 1 through width-1
        let canvas_x = 1 + ((normalized * (self.width as f64 - 2.0)).round() as usize);
        canvas_x.min(self.width - 1)
    }

    fn data_to_canvas_y(&self, y: f64) -> usize {
        let normalized = (y - self.y_range.0) / (self.y_range.1 - self.y_range.0);
        let inverted = 1.0 - normalized; // Invert Y axis (top = high values)
        ((inverted * (self.height as f64 - 1.0)).round() as usize).min(self.height - 1)
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn get_ranges(&self) -> ((f64, f64), (f64, f64)) {
        (self.x_range, self.y_range)
    }

    pub fn is_point_in_bounds(&self, x: f64, y: f64) -> bool {
        x >= self.x_range.0 && x <= self.x_range.1 && 
        y >= self.y_range.0 && y <= self.y_range.1
    }

    pub fn fill_area(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, symbol: char) {
        self.fill_area_with_color(x1, y1, x2, y2, symbol, None);
    }

    pub fn fill_area_with_color(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, symbol: char, color: Option<Color>) {
        let cx1 = self.data_to_canvas_x(x1.min(x2));
        let cy1 = self.data_to_canvas_y(y1.max(y2));
        let cx2 = self.data_to_canvas_x(x1.max(x2));
        let cy2 = self.data_to_canvas_y(y1.min(y2));

        for y in cy1..=cy2.min(self.height - 1) {
            for x in cx1..=cx2.min(self.width - 1) {
                if y < self.buffer.len() && x < self.buffer[y].len() {
                    self.buffer[y][x] = symbol;
                    self.color_buffer[y][x] = color;
                }
            }
        }
    }

    // Getter methods for private fields
    pub fn get_width(&self) -> usize { self.width }
    pub fn get_height(&self) -> usize { self.height }
    pub fn get_x_range(&self) -> (f64, f64) { self.x_range }
    pub fn get_y_range(&self) -> (f64, f64) { self.y_range }
    pub fn get_title(&self) -> &Option<String> { &self.title }
    pub fn get_xlabel(&self) -> &Option<String> { &self.xlabel }
    pub fn get_buffer(&self) -> &Vec<Vec<char>> { &self.buffer }
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}