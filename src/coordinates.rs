use crate::data::DataPoint as OldDataPoint;

#[derive(Debug, Clone, Copy)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
}

impl From<OldDataPoint> for DataPoint {
    fn from(old: OldDataPoint) -> Self {
        Self { x: old.x, y: old.y }
    }
}

impl From<DataPoint> for OldDataPoint {
    fn from(new: DataPoint) -> Self {
        Self { x: new.x, y: new.y }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScreenPoint {
    pub col: usize,
    pub row: usize,
}

#[derive(Debug, Clone)]
pub struct DataBounds {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}

impl DataBounds {
    pub fn from_points(points: &[DataPoint]) -> Self {
        if points.is_empty() {
            return Self {
                min_x: -10.0,
                max_x: 10.0,
                min_y: -10.0,
                max_y: 10.0,
            };
        }

        let min_x = points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
        let max_x = points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
        let min_y = points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
        let max_y = points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);

        let x_range = max_x - min_x;
        let y_range = max_y - min_y;
        let x_padding = if x_range > 0.0 { x_range * 0.1 } else { 1.0 };
        let y_padding = if y_range > 0.0 { y_range * 0.1 } else { 1.0 };

        Self {
            min_x: min_x - x_padding,
            max_x: max_x + x_padding,
            min_y: min_y - y_padding,
            max_y: max_y + y_padding,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Margins {
    pub left: usize,
    pub right: usize,
    pub top: usize,
    pub bottom: usize,
}

impl Default for Margins {
    fn default() -> Self {
        Self {
            left: 4,
            right: 2,
            top: 1,
            bottom: 2,
        }
    }
}

pub struct CoordinateTransformer {
    data_bounds: DataBounds,
    screen_width: usize,
    screen_height: usize,
    margins: Margins,
}

impl CoordinateTransformer {
    pub fn new(bounds: DataBounds, width: usize, height: usize, margins: Margins) -> Self {
        Self {
            data_bounds: bounds,
            screen_width: width,
            screen_height: height,
            margins,
        }
    }

    pub fn data_to_screen(&self, point: DataPoint) -> Option<ScreenPoint> {
        let plot_width = self.screen_width.saturating_sub(self.margins.left + self.margins.right);
        let plot_height = self.screen_height.saturating_sub(self.margins.top + self.margins.bottom);

        if plot_width == 0 || plot_height == 0 {
            return None;
        }

        let x_range = self.data_bounds.max_x - self.data_bounds.min_x;
        let y_range = self.data_bounds.max_y - self.data_bounds.min_y;

        if x_range == 0.0 || y_range == 0.0 {
            return None;
        }

        let norm_x = (point.x - self.data_bounds.min_x) / x_range;
        let norm_y = (point.y - self.data_bounds.min_y) / y_range;

        if norm_x < 0.0 || norm_x > 1.0 || norm_y < 0.0 || norm_y > 1.0 {
            return None;
        }

        let col = self.margins.left + (norm_x * plot_width as f64).round() as usize;
        let row = self.margins.top + ((1.0 - norm_y) * plot_height as f64).round() as usize;

        if col < self.screen_width && row < self.screen_height {
            Some(ScreenPoint { col, row })
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn screen_to_data(&self, point: ScreenPoint) -> DataPoint {
        let plot_width = self.screen_width - self.margins.left - self.margins.right;
        let plot_height = self.screen_height - self.margins.top - self.margins.bottom;

        let norm_x = (point.col.saturating_sub(self.margins.left)) as f64 / plot_width as f64;
        let norm_y = 1.0 - ((point.row.saturating_sub(self.margins.top)) as f64 / plot_height as f64);

        DataPoint {
            x: self.data_bounds.min_x + norm_x * (self.data_bounds.max_x - self.data_bounds.min_x),
            y: self.data_bounds.min_y + norm_y * (self.data_bounds.max_y - self.data_bounds.min_y),
        }
    }

    #[allow(dead_code)]
    pub fn get_plot_area(&self) -> (usize, usize, usize, usize) {
        let left = self.margins.left;
        let top = self.margins.top;
        let width = self.screen_width.saturating_sub(self.margins.left + self.margins.right);
        let height = self.screen_height.saturating_sub(self.margins.top + self.margins.bottom);
        (left, top, width, height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_bounds_from_points() {
        let points = vec![
            DataPoint { x: 1.0, y: 2.0 },
            DataPoint { x: 5.0, y: 8.0 },
            DataPoint { x: 3.0, y: 4.0 },
        ];
        
        let bounds = DataBounds::from_points(&points);
        
        assert!(bounds.min_x < 1.0);
        assert!(bounds.max_x > 5.0);
        assert!(bounds.min_y < 2.0);
        assert!(bounds.max_y > 8.0);
    }

    #[test]
    fn test_coordinate_transformation() {
        let bounds = DataBounds {
            min_x: 0.0,
            max_x: 10.0,
            min_y: 0.0,
            max_y: 100.0,
        };
        let margins = Margins::default();
        let transformer = CoordinateTransformer::new(bounds, 80, 24, margins);
        
        let data_pt = DataPoint { x: 5.0, y: 50.0 };
        let screen_pt = transformer.data_to_screen(data_pt).unwrap();
        
        assert!(screen_pt.col > margins.left);
        assert!(screen_pt.col < 80 - margins.right);
        assert!(screen_pt.row > margins.top);
        assert!(screen_pt.row < 24 - margins.bottom);
    }

    #[test]
    fn test_coordinate_roundtrip() {
        let bounds = DataBounds {
            min_x: -5.0,
            max_x: 15.0,
            min_y: -10.0,
            max_y: 90.0,
        };
        let margins = Margins::default();
        let transformer = CoordinateTransformer::new(bounds, 80, 24, margins);
        
        let original = DataPoint { x: 7.5, y: 25.0 };
        if let Some(screen_pt) = transformer.data_to_screen(original) {
            let recovered = transformer.screen_to_data(screen_pt);
            
            assert!((original.x - recovered.x).abs() < 1.0);
            assert!((original.y - recovered.y).abs() < 5.0);
        }
    }

    #[test]
    fn test_bounds_checking() {
        let bounds = DataBounds {
            min_x: 0.0,
            max_x: 10.0,
            min_y: 0.0,
            max_y: 10.0,
        };
        let margins = Margins::default();
        let transformer = CoordinateTransformer::new(bounds, 80, 24, margins);
        
        let out_of_bounds = DataPoint { x: 15.0, y: 5.0 };
        assert!(transformer.data_to_screen(out_of_bounds).is_none());
        
        let in_bounds = DataPoint { x: 5.0, y: 5.0 };
        assert!(transformer.data_to_screen(in_bounds).is_some());
    }
}