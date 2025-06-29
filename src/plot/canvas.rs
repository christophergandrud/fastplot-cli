use unicode_width::UnicodeWidthChar;

pub struct Canvas {
    width: usize,
    height: usize,
    buffer: Vec<Vec<char>>,
    x_range: (f64, f64),
    y_range: (f64, f64),
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![vec![' '; width]; height];
        Self {
            width,
            height,
            buffer,
            x_range: (0.0, 1.0),
            y_range: (0.0, 1.0),
        }
    }

    pub fn set_ranges(&mut self, x_range: (f64, f64), y_range: (f64, f64)) {
        self.x_range = x_range;
        self.y_range = y_range;
    }

    pub fn plot_point(&mut self, x: f64, y: f64, symbol: char) {
        let canvas_x = self.data_to_canvas_x(x);
        let canvas_y = self.data_to_canvas_y(y);

        if canvas_x < self.width && canvas_y < self.height {
            self.buffer[canvas_y][canvas_x] = symbol;
        }
    }

    pub fn draw_axis(&mut self) {
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
    }

    pub fn render(&self) -> String {
        let mut result = String::with_capacity(self.height * (self.width + 1));
        
        for row in &self.buffer {
            for &ch in row {
                result.push(ch);
            }
            result.push('\n');
        }
        
        result
    }

    fn data_to_canvas_x(&self, x: f64) -> usize {
        let normalized = (x - self.x_range.0) / (self.x_range.1 - self.x_range.0);
        ((normalized * (self.width as f64 - 1.0)).round() as usize).min(self.width - 1)
    }

    fn data_to_canvas_y(&self, y: f64) -> usize {
        let normalized = (y - self.y_range.0) / (self.y_range.1 - self.y_range.0);
        let inverted = 1.0 - normalized; // Invert Y axis (top = high values)
        ((inverted * (self.height as f64 - 1.0)).round() as usize).min(self.height - 1)
    }
}