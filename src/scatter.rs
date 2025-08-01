use crate::coordinates::{DataBounds, CoordinateTransformer};
use crate::layout::LayoutEngine;
use crate::data::{Dataset, DataPoint};
use crate::color;

pub struct ScatterPlot {
    width: usize,
    height: usize,
    data: Vec<DataPoint>,
    title: String,
    x_label: String,
    y_label: String,
}

impl ScatterPlot {
    pub fn new(dataset: &Dataset, title: &str, width: usize, height: usize) -> Self {
        let data = dataset.points.clone();
        
        Self {
            width,
            height,
            data,
            title: title.to_string(),
            x_label: dataset.x_label.clone(),
            y_label: dataset.y_label.clone(),
        }
    }

    pub fn render(&self, symbol: char, color: Option<&str>) -> String {
        if self.data.is_empty() {
            return format!("{}\n\nNo data to plot\n", self.title);
        }

        let bounds = DataBounds::from_numeric_data_points(&self.data);
        let layout_engine = LayoutEngine::new(self.width, self.height);
        let layout = layout_engine.calculate_layout(&bounds);
        
        let transformer = CoordinateTransformer::new(
            bounds,
            self.width,
            self.height,
            layout.margins.clone(),
        );
        
        let mut canvas = CharCanvas::new(self.width, self.height);
        
        self.draw_axes(&mut canvas, &layout);
        self.draw_ticks_and_labels(&mut canvas, &layout);
        
        for point in &self.data {
            if let Some(screen_pt) = transformer.transform_data_point(point) {
                canvas.set_char(screen_pt.col, screen_pt.row, symbol, color);
            }
        }
        
        let mut output = String::new();
        output.push_str(&self.title);
        output.push_str("\n\n");
        output.push_str(&self.y_label);
        output.push('\n');
        output.push_str(&canvas.render_with_y_labels(&layout));
        output.push_str(&crate::layout::format_x_axis_label(&self.x_label, &layout));
        
        output
    }

    fn draw_axes(&self, canvas: &mut CharCanvas, layout: &crate::layout::Layout) {
        let area = &layout.plot_area;
        
        let axis_col = area.left.saturating_sub(1);
        let axis_row = area.top + area.height;
        
        for row in area.top..=area.top + area.height {
            canvas.set_char_simple(axis_col, row, '│');
        }
        
        for col in axis_col..=area.left + area.width {
            canvas.set_char_simple(col, axis_row, '─');
        }
        
        canvas.set_char_simple(axis_col, axis_row, '└');
    }

    fn draw_ticks_and_labels(&self, canvas: &mut CharCanvas, layout: &crate::layout::Layout) {
        for (col, tick) in &layout.x_ticks {
            let axis_row = layout.plot_area.top + layout.plot_area.height;
            canvas.set_char_simple(*col, axis_row, '┬');
            
            let label_row = axis_row + 1;
            let label_start = col.saturating_sub(tick.label.len() / 2);
            canvas.set_string(label_start, label_row, &tick.label);
        }
        
        for (row, _tick) in &layout.y_ticks {
            let axis_col = layout.plot_area.left.saturating_sub(1);
            canvas.set_char_simple(axis_col, *row, '┤');
        }
    }
}

struct CharCanvas {
    width: usize,
    height: usize,
    buffer: Vec<Vec<char>>,
    colors: Vec<Vec<Option<String>>>,
}

impl CharCanvas {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![vec![' '; width]; height],
            colors: vec![vec![None; width]; height],
        }
    }

    fn set_char(&mut self, col: usize, row: usize, ch: char, color: Option<&str>) {
        if col < self.width && row < self.height {
            self.buffer[row][col] = ch;
            self.colors[row][col] = color.map(|s| s.to_string());
        }
    }

    fn set_char_simple(&mut self, col: usize, row: usize, ch: char) {
        if col < self.width && row < self.height {
            self.buffer[row][col] = ch;
        }
    }

    fn set_string(&mut self, col: usize, row: usize, s: &str) {
        for (i, ch) in s.chars().enumerate() {
            self.set_char_simple(col + i, row, ch);
        }
    }

    fn render_with_y_labels(&self, layout: &crate::layout::Layout) -> String {
        let mut output = String::new();
        
        // Calculate the maximum width of y-axis labels for consistent alignment
        let max_label_width = layout.y_ticks.iter()
            .map(|(_, tick)| tick.label.len())
            .max()
            .unwrap_or(0);
        
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
            
            let mut line = String::new();
            for (col, ch) in self.buffer[row].iter().enumerate() {
                if let Some(color_str) = &self.colors[row][col] {
                    if let Some(colored_char) = color::apply_color(*ch, color_str) {
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
        
        // Add x-axis labels below the plot
        let x_axis_row = plot_end + 1;
        if x_axis_row < self.height {
            output.push_str(&" ".repeat(max_label_width + 1));
            let x_label_line: String = self.buffer[x_axis_row].iter().collect();
            output.push_str(x_label_line.trim_end());
            output.push('\n');
        }
        
        output
    }
}


pub fn render_scatter_plot(dataset: &Dataset, title: &str, symbol: char, color: Option<&str>) -> String {
    let plot = ScatterPlot::new(dataset, title, 80, 24);
    plot.render(symbol, color)
}