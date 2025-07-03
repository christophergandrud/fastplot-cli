# Rust CLI Plotting Improvements Guide

## Problem Summary

Your current plot shows two main issues:
1. **X-axis misalignment**: Labels show "0 0 1 1 2" instead of proper spacing
2. **Staircase effect**: Lines between points look stepped rather than smooth

```
   3.2 ┤                                                                      
       ┤                                                                      
       ┤                                                                  ∙∙∙◆
   2.7 ┤                                                            ∙∙∙∙∙∙∙   
       ┤                                                      ∙∙∙∙∙∙∙         
       ┤                                                 ∙∙∙∙∙∙               
   2.2 ┤                                           ∙∙∙∙∙∙∙                    
       ┤                                     ∙∙∙∙∙∙∙                          
       ┤                               ∙∙∙◆∙∙∙                                
   1.7 ┤                        ∙∙∙∙∙∙∙∙                                      
       ┤                 ∙∙∙∙∙∙∙∙                                             
       ┤          ∙∙∙∙∙∙∙∙                                                    
   1.1 ┤   ∙∙∙∙∙∙∙∙                                                           
       ┤◆∙∙∙                                                                  
       ┤                                                                      
     └───────────────────────────────────┬──────────────────────────────────
      0                0                1                1                2
```

## Solution 1: Fix X-Axis Alignment

### A. Calculate Proper Label Spacing

```rust
use std::fmt::Write;

struct AxisLabels {
    labels: Vec<String>,
    positions: Vec<usize>,
}

impl AxisLabels {
    fn new(min: f64, max: f64, width: usize, label_count: usize) -> Self {
        let range = max - min;
        let label_interval = range / (label_count - 1) as f64;
        
        let mut labels = Vec::new();
        let mut positions = Vec::new();
        
        for i in 0..label_count {
            let value = min + i as f64 * label_interval;
            labels.push(format!("{:.1}", value));
            
            // Calculate position in terminal columns
            let normalized = i as f64 / (label_count - 1) as f64;
            let position = (normalized * (width - 1) as f64).round() as usize;
            positions.push(position);
        }
        
        Self { labels, positions }
    }
    
    fn render(&self, width: usize) -> String {
        let mut axis_line = vec![' '; width];
        
        for (label, &pos) in self.labels.iter().zip(&self.positions) {
            // Ensure labels don't overlap
            let label_start = pos.saturating_sub(label.len() / 2);
            let label_end = (label_start + label.len()).min(width);
            
            for (i, ch) in label.chars().enumerate() {
                if label_start + i < label_end {
                    axis_line[label_start + i] = ch;
                }
            }
        }
        
        axis_line.into_iter().collect()
    }
}
```

### B. Coordinate Mapping Functions

```rust
/// Map x-coordinate to terminal column
fn x_to_col(x: f64, x_min: f64, x_max: f64, width: usize) -> usize {
    let normalized = (x - x_min) / (x_max - x_min);
    (normalized * (width - 1) as f64).round().max(0.0).min((width - 1) as f64) as usize
}

/// Map y-coordinate to terminal row (inverted because terminal rows increase downward)
fn y_to_row(y: f64, y_min: f64, y_max: f64, height: usize) -> usize {
    let normalized = (y - y_min) / (y_max - y_min);
    let row = ((1.0 - normalized) * (height - 1) as f64).round();
    row.max(0.0).min((height - 1) as f64) as usize
}
```

## Solution 2: Smooth Line Interpolation

### A. Bresenham's Line Algorithm

```rust
/// Generate all points along a line between two points
fn bresenham_line(p1: (usize, usize), p2: (usize, usize)) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    
    let dx = (p2.0 as i32 - p1.0 as i32).abs();
    let dy = (p2.1 as i32 - p1.1 as i32).abs();
    let sx = if p1.0 < p2.0 { 1 } else { -1 };
    let sy = if p1.1 < p2.1 { 1 } else { -1 };
    
    let mut err = dx - dy;
    let mut x = p1.0 as i32;
    let mut y = p1.1 as i32;
    
    loop {
        points.push((x as usize, y as usize));
        
        if x == p2.0 as i32 && y == p2.1 as i32 {
            break;
        }
        
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
    
    points
}
```

### B. Unicode Characters for Smoother Lines

```rust
/// Select appropriate line character based on direction
fn get_line_char(dx: i32, dy: i32) -> char {
    match (dx.signum(), dy.signum()) {
        (0, 0) => '·',    // Point
        (_, 0) => '─',    // Horizontal
        (0, _) => '│',    // Vertical
        (1, -1) | (-1, 1) => '╱',  // Diagonal /
        (1, 1) | (-1, -1) => '╲',  // Diagonal \
        _ => '·',
    }
}

/// For sub-character resolution using Braille patterns
const BRAILLE_DOTS: [[u16; 4]; 2] = [
    [0x2801, 0x2802, 0x2804, 0x2840],  // ⠁ ⠂ ⠄ ⡀
    [0x2808, 0x2810, 0x2820, 0x2880],  // ⠈ ⠐ ⠠ ⢀
];

fn get_braille_char(x_fraction: f64, y_fraction: f64) -> char {
    let x_idx = (x_fraction * 2.0).min(1.9) as usize;
    let y_idx = (y_fraction * 4.0).min(3.9) as usize;
    char::from_u32(BRAILLE_DOTS[x_idx][y_idx] as u32).unwrap_or('·')
}
```

## Solution 3: Complete Plot Implementation

```rust
pub struct TerminalPlot {
    width: usize,
    height: usize,
    canvas: Vec<Vec<char>>,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

impl TerminalPlot {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            canvas: vec![vec![' '; width]; height],
            x_min: 0.0,
            x_max: 1.0,
            y_min: 0.0,
            y_max: 1.0,
        }
    }
    
    pub fn plot_line(&mut self, data: &[(f64, f64)]) {
        if data.is_empty() {
            return;
        }
        
        // Calculate bounds
        self.x_min = data.iter().map(|(x, _)| *x).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        self.x_max = data.iter().map(|(x, _)| *x).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        self.y_min = data.iter().map(|(_, y)| *y).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        self.y_max = data.iter().map(|(_, y)| *y).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        
        // Add padding
        let x_padding = (self.x_max - self.x_min) * 0.1;
        let y_padding = (self.y_max - self.y_min) * 0.1;
        self.x_min -= x_padding;
        self.x_max += x_padding;
        self.y_min -= y_padding;
        self.y_max += y_padding;
        
        // Convert to canvas coordinates
        let canvas_points: Vec<(usize, usize)> = data.iter()
            .map(|(x, y)| {
                let col = x_to_col(*x, self.x_min, self.x_max, self.width);
                let row = y_to_row(*y, self.y_min, self.y_max, self.height);
                (col, row)
            })
            .collect();
        
        // Draw smooth lines between points
        for window in canvas_points.windows(2) {
            let line_points = bresenham_line(window[0], window[1]);
            for (x, y) in line_points {
                if x < self.width && y < self.height {
                    self.canvas[y][x] = '─';
                }
            }
        }
        
        // Draw data points on top
        for (i, (x, y)) in canvas_points.iter().enumerate() {
            if *x < self.width && *y < self.height {
                // Mark first and last points differently
                self.canvas[*y][*x] = if i == 0 || i == canvas_points.len() - 1 {
                    '◆'
                } else {
                    '●'
                };
            }
        }
    }
    
    pub fn render(&self) -> String {
        let mut output = String::new();
        
        // Y-axis labels
        let y_labels = 5;
        let y_interval = (self.y_max - self.y_min) / (y_labels - 1) as f64;
        
        for (row_idx, row) in self.canvas.iter().enumerate() {
            // Add Y-axis label every few rows
            if row_idx % (self.height / y_labels) == 0 {
                let y_value = self.y_max - (row_idx as f64 / self.height as f64) * (self.y_max - self.y_min);
                write!(&mut output, "{:6.1} ┤", y_value).unwrap();
            } else {
                write!(&mut output, "       ┤").unwrap();
            }
            
            // Draw canvas row
            for &ch in row {
                output.push(ch);
            }
            output.push('\n');
        }
        
        // X-axis
        output.push_str("     └");
        output.push_str(&"─".repeat(self.width));
        output.push('\n');
        
        // X-axis labels
        let x_labels = AxisLabels::new(self.x_min, self.x_max, self.width, 5);
        output.push_str("      ");
        output.push_str(&x_labels.render(self.width));
        
        output
    }
}
```

## Solution 4: Using External Libraries

### Option A: textplots-rs

```toml
[dependencies]
textplots = "0.8"
```

```rust
use textplots::{Chart, Plot, Shape};

fn plot_with_textplots(data: &[(f64, f64)]) {
    let mut chart = Chart::new(180, 60, 0.0, 10.0);
    chart
        .lineplot(&Shape::Lines(data))
        .nice();
    println!("{}", chart);
}
```

### Option B: drawille-rs (Braille canvas)

```toml
[dependencies]
drawille = "0.3"
```

```rust
use drawille::Canvas;

fn plot_with_drawille(data: &[(f64, f64)]) {
    let mut canvas = Canvas::new(80, 40);
    
    // Scale data to canvas dimensions
    let x_scale = 80.0 / (data.iter().map(|(x, _)| x).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());
    let y_scale = 40.0 / (data.iter().map(|(_, y)| y).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());
    
    // Draw lines
    for window in data.windows(2) {
        let (x1, y1) = window[0];
        let (x2, y2) = window[1];
        canvas.line(
            (x1 * x_scale) as u32,
            (y1 * y_scale) as u32,
            (x2 * x_scale) as u32,
            (y2 * y_scale) as u32,
        );
    }
    
    println!("{}", canvas.frame());
}
```

## Testing Your Implementation

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_smooth_plot() {
        let mut plot = TerminalPlot::new(70, 20);
        
        // Generate test data
        let data: Vec<(f64, f64)> = (0..20)
            .map(|i| {
                let x = i as f64 / 10.0;
                let y = (x * 2.0).sin() + 2.0;
                (x, y)
            })
            .collect();
        
        plot.plot_line(&data);
        let output = plot.render();
        
        // Check that output doesn't have duplicate x-axis labels
        assert!(!output.contains("0 0"));
        
        // Check that we have smooth lines (multiple line characters)
        let line_chars = output.chars().filter(|&c| c == '─').count();
        assert!(line_chars > data.len());
    }
}
```

## Performance Considerations

1. **Pre-allocate buffers**: Create the canvas once and reuse it
2. **Avoid string concatenation**: Use `String::with_capacity()` or `write!()`
3. **Cache calculations**: Store scaled coordinates if plotting the same data multiple times
4. **Use integer arithmetic**: When possible, work with integer coordinates

## Example Usage

```rust
fn main() {
    let mut plot = TerminalPlot::new(70, 20);
    
    // Your data
    let data = vec![
        (0.0, 1.0),
        (0.5, 1.5),
        (1.0, 1.8),
        (1.5, 2.3),
        (2.0, 3.0),
    ];
    
    plot.plot_line(&data);
    println!("{}", plot.render());
}
```

This should produce a much smoother plot with properly aligned axes!