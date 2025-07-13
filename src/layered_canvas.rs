use colored::Colorize;

/// Priority levels for rendering (higher overwrites lower)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderPriority {
    Axes = 2,
    Lines = 3,
    Points = 4,
    Labels = 5,
}

pub struct LayeredCanvas {
    width: usize,
    height: usize,
    layers: Vec<(RenderPriority, Canvas)>,
}

impl LayeredCanvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            layers: vec![],
        }
    }

    pub fn get_layer(&mut self, priority: RenderPriority) -> &mut Canvas {
        // Find or create layer
        let idx = self.layers.iter().position(|(p, _)| *p == priority);
        
        match idx {
            Some(i) => &mut self.layers[i].1,
            None => {
                self.layers.push((priority, Canvas::new(self.width, self.height)));
                self.layers.sort_by_key(|(p, _)| *p);
                let idx = self.layers.iter().position(|(p, _)| *p == priority).unwrap();
                &mut self.layers[idx].1
            }
        }
    }

    pub fn flatten(&self) -> Canvas {
        let mut result = Canvas::new(self.width, self.height);
        
        // Apply layers in order of priority
        for (_, layer) in &self.layers {
            for row in 0..self.height {
                for col in 0..self.width {
                    let ch = layer.buffer[row][col];
                    let color = &layer.colors[row][col];
                    if ch != ' ' {
                        result.buffer[row][col] = ch;
                        result.colors[row][col] = color.clone();
                    }
                }
            }
        }
        
        result
    }
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Vec<char>>,
    pub colors: Vec<Vec<Option<String>>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![vec![' '; width]; height],
            colors: vec![vec![None; width]; height],
        }
    }

    pub fn draw_point(&mut self, col: usize, row: usize, ch: char) {
        if col < self.width && row < self.height {
            self.buffer[row][col] = ch;
        }
    }

    pub fn draw_point_with_color(&mut self, col: usize, row: usize, ch: char, color: Option<&str>) {
        if col < self.width && row < self.height {
            self.buffer[row][col] = ch;
            self.colors[row][col] = color.map(|s| s.to_string());
        }
    }

    pub fn draw_text(&mut self, col: usize, row: usize, text: &str) {
        for (i, ch) in text.chars().enumerate() {
            if col + i < self.width && row < self.height {
                self.buffer[row][col + i] = ch;
            }
        }
    }

    pub fn draw_line(&mut self, row: usize, start_col: usize, end_col: usize, ch: char) {
        if row < self.height {
            let start = start_col.min(end_col);
            let end = start_col.max(end_col);
            for col in start..=end {
                if col < self.width {
                    self.buffer[row][col] = ch;
                }
            }
        }
    }

    pub fn draw_vertical_line(&mut self, col: usize, start_row: usize, end_row: usize, ch: char) {
        if col < self.width {
            let start = start_row.min(end_row);
            let end = start_row.max(end_row);
            for row in start..=end {
                if row < self.height {
                    self.buffer[row][col] = ch;
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        
        for row in 0..self.height {
            let mut line = String::new();
            for (col, ch) in self.buffer[row].iter().enumerate() {
                if let Some(color_str) = &self.colors[row][col] {
                    if let Some(colored_char) = apply_color(*ch, color_str) {
                        line.push_str(&colored_char);
                    } else {
                        line.push(*ch);
                    }
                } else {
                    line.push(*ch);
                }
            }
            output.push_str(line.trim_end());
            output.push('\n');
        }
        
        output
    }
}

fn apply_color(ch: char, color_str: &str) -> Option<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layered_canvas_creation() {
        let canvas = LayeredCanvas::new(10, 5);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 5);
        assert!(canvas.layers.is_empty());
    }

    #[test]
    fn test_layer_ordering() {
        let mut canvas = LayeredCanvas::new(10, 5);
        
        // Get layers in different order
        canvas.get_layer(RenderPriority::Points);
        canvas.get_layer(RenderPriority::Axes);
        canvas.get_layer(RenderPriority::Lines);
        
        // Layers should be sorted by priority
        for i in 1..canvas.layers.len() {
            assert!(canvas.layers[i-1].0 <= canvas.layers[i].0);
        }
    }

    #[test]
    fn test_layer_overwrite() {
        let mut canvas = LayeredCanvas::new(10, 5);
        
        // Draw on axes layer
        let axes_layer = canvas.get_layer(RenderPriority::Axes);
        axes_layer.draw_point(2, 2, 'A');
        
        // Draw on points layer (higher priority)
        let points_layer = canvas.get_layer(RenderPriority::Points);
        points_layer.draw_point(2, 2, 'P');
        
        // Flatten and check that points layer overwrites axes
        let result = canvas.flatten();
        assert_eq!(result.buffer[2][2], 'P');
    }

    #[test]
    fn test_canvas_drawing() {
        let mut canvas = Canvas::new(5, 3);
        
        canvas.draw_point(1, 1, 'X');
        canvas.draw_text(0, 0, "Hi");
        canvas.draw_line(2, 0, 4, '-');
        
        assert_eq!(canvas.buffer[1][1], 'X');
        assert_eq!(canvas.buffer[0][0], 'H');
        assert_eq!(canvas.buffer[0][1], 'i');
        assert_eq!(canvas.buffer[2][0], '-');
        assert_eq!(canvas.buffer[2][4], '-');
    }
}