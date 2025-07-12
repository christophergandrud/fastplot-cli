use crate::coordinates::{DataBounds, Margins};
use crate::ticks::{Tick, TickGenerator};

#[derive(Debug, Clone)]
pub struct Layout {
    pub margins: Margins,
    pub x_ticks: Vec<(usize, Tick)>,
    pub y_ticks: Vec<(usize, Tick)>,
    pub plot_area: PlotArea,
}

#[derive(Debug, Clone)]
pub struct PlotArea {
    pub left: usize,
    pub top: usize,
    pub width: usize,
    pub height: usize,
}

pub struct LayoutEngine {
    canvas_width: usize,
    canvas_height: usize,
    show_labels: bool,
    label_padding: usize,
}

impl LayoutEngine {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            canvas_width: width,
            canvas_height: height,
            show_labels: true,
            label_padding: 1,
        }
    }

    #[allow(dead_code)]
    pub fn with_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    pub fn calculate_layout(&self, data_bounds: &DataBounds) -> Layout {
        let tick_gen = TickGenerator::default();
        
        let x_ticks = tick_gen.generate_ticks(data_bounds.min_x, data_bounds.max_x);
        let y_ticks = tick_gen.generate_ticks(data_bounds.min_y, data_bounds.max_y);
        
        let margins = self.calculate_margins(&x_ticks, &y_ticks);
        
        let plot_area = PlotArea {
            left: margins.left,
            top: margins.top,
            width: self.canvas_width.saturating_sub(margins.left + margins.right),
            height: self.canvas_height.saturating_sub(margins.top + margins.bottom),
        };
        
        let x_tick_positions = self.position_x_ticks(&x_ticks, data_bounds, &plot_area);
        let y_tick_positions = self.position_y_ticks(&y_ticks, data_bounds, &plot_area);
        
        Layout {
            margins,
            x_ticks: x_tick_positions,
            y_ticks: y_tick_positions,
            plot_area,
        }
    }

    fn calculate_margins(&self, x_ticks: &[Tick], y_ticks: &[Tick]) -> Margins {
        if !self.show_labels {
            return Margins { 
                left: 2, 
                right: 2, 
                top: 1, 
                bottom: 1 
            };
        }
        
        let max_y_label_width = y_ticks.iter()
            .map(|t| t.label.len())
            .max()
            .unwrap_or(3);
        
        let max_x_label_width = x_ticks.iter()
            .map(|t| t.label.len())
            .max()
            .unwrap_or(1);
        
        Margins {
            left: max_y_label_width + self.label_padding + 1,
            right: (max_x_label_width / 2).max(1),
            top: 1,
            bottom: 2 + self.label_padding,
        }
    }

    fn position_x_ticks(&self, ticks: &[Tick], bounds: &DataBounds, area: &PlotArea) -> Vec<(usize, Tick)> {
        if area.width == 0 {
            return Vec::new();
        }

        let x_range = bounds.max_x - bounds.min_x;
        if x_range == 0.0 {
            return Vec::new();
        }

        ticks.iter().filter_map(|tick| {
            let norm = (tick.value - bounds.min_x) / x_range;
            if norm >= 0.0 && norm <= 1.0 {
                let col = area.left + (norm * area.width as f64).round() as usize;
                if col <= area.left + area.width {
                    Some((col, tick.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        }).collect()
    }

    fn position_y_ticks(&self, ticks: &[Tick], bounds: &DataBounds, area: &PlotArea) -> Vec<(usize, Tick)> {
        if area.height == 0 {
            return Vec::new();
        }

        let y_range = bounds.max_y - bounds.min_y;
        if y_range == 0.0 {
            return Vec::new();
        }

        ticks.iter().filter_map(|tick| {
            let norm = (tick.value - bounds.min_y) / y_range;
            if norm >= 0.0 && norm <= 1.0 {
                let row = area.top + ((1.0 - norm) * area.height as f64).round() as usize;
                if row >= area.top && row <= area.top + area.height {
                    Some((row, tick.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_calculation() {
        let bounds = DataBounds {
            min_x: 0.0,
            max_x: 10.0,
            min_y: 0.0,
            max_y: 100.0,
        };
        
        let engine = LayoutEngine::new(80, 24);
        let layout = engine.calculate_layout(&bounds);
        
        assert!(layout.plot_area.width > 0);
        assert!(layout.plot_area.height > 0);
        assert!(!layout.x_ticks.is_empty());
        assert!(!layout.y_ticks.is_empty());
    }

    #[test]
    fn test_margin_calculation() {
        let bounds = DataBounds {
            min_x: -1000.0,
            max_x: 1000.0,
            min_y: -1000.0,
            max_y: 1000.0,
        };
        
        let engine = LayoutEngine::new(80, 24);
        let layout = engine.calculate_layout(&bounds);
        
        assert!(layout.margins.left >= 4);
    }
}