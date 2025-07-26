use crate::data::DataPoint;
use std::collections::HashMap;

/// Simple coordinate pair for numeric positioning
/// This is used internally for coordinate transformations after type resolution
#[derive(Debug, Clone, Copy)]
pub struct NumericCoordinate {
    pub x: f64,
    pub y: f64,
}

impl NumericCoordinate {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    /// Create bounds from numeric coordinates only
    /// Categorical data should use specialized bounds calculation
    pub fn from_numeric_coordinates(coords: &[NumericCoordinate]) -> Self {
        if coords.is_empty() {
            return Self {
                min_x: -10.0,
                max_x: 10.0,
                min_y: -10.0,
                max_y: 10.0,
            };
        }

        let min_x = coords.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
        let max_x = coords.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
        let min_y = coords.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
        let max_y = coords.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);

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

    /// Create bounds from DataPoint enum, filtering to numeric data only
    /// This preserves type safety by explicitly handling only numeric data
    pub fn from_numeric_data_points(points: &[DataPoint]) -> Self {
        let numeric_coords: Vec<NumericCoordinate> = points
            .iter()
            .filter_map(|p| match p {
                DataPoint::Numeric(x, y) => Some(NumericCoordinate::new(*x, *y)),
                DataPoint::Categorical(_, _) => None, // Skip categorical data
            })
            .collect();
        
        Self::from_numeric_coordinates(&numeric_coords)
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

    /// Convert DataPoint to NumericCoordinate for transformation
    /// This preserves type information by only working with numeric data
    pub fn data_point_to_coordinate(&self, point: &DataPoint) -> Option<NumericCoordinate> {
        match point {
            DataPoint::Numeric(x, y) => Some(NumericCoordinate::new(*x, *y)),
            DataPoint::Categorical(_, _) => None, // Categorical data needs special handling
        }
    }

    /// Transform DataPoint to ScreenPoint (for numeric data only)
    pub fn transform_data_point(&self, point: &DataPoint) -> Option<ScreenPoint> {
        let coord = self.data_point_to_coordinate(point)?;
        self.data_to_screen(coord)
    }

    pub fn data_to_screen(&self, point: NumericCoordinate) -> Option<ScreenPoint> {
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
    pub fn screen_to_data(&self, point: ScreenPoint) -> NumericCoordinate {
        let plot_width = self.screen_width - self.margins.left - self.margins.right;
        let plot_height = self.screen_height - self.margins.top - self.margins.bottom;

        let norm_x = (point.col.saturating_sub(self.margins.left)) as f64 / plot_width as f64;
        let norm_y = 1.0 - ((point.row.saturating_sub(self.margins.top)) as f64 / plot_height as f64);

        NumericCoordinate::new(
            self.data_bounds.min_x + norm_x * (self.data_bounds.max_x - self.data_bounds.min_x),
            self.data_bounds.min_y + norm_y * (self.data_bounds.max_y - self.data_bounds.min_y),
        )
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

pub struct CategoricalTransformer {
    category_map: HashMap<String, f64>,
    data_bounds: DataBounds,
    screen_width: usize,
    screen_height: usize,
    margins: Margins,
}

impl CategoricalTransformer {
    pub fn new(categories: &[String], data_bounds: DataBounds, width: usize, height: usize, margins: Margins) -> Self {
        let mut category_map = HashMap::new();
        
        // Map categories to evenly spaced positions
        if !categories.is_empty() {
            let x_min = data_bounds.min_x;
            let x_max = data_bounds.max_x;
            let step = if categories.len() > 1 {
                (x_max - x_min) / (categories.len() - 1) as f64
            } else {
                0.0
            };
            
            for (i, category) in categories.iter().enumerate() {
                let position = x_min + (i as f64 * step);
                category_map.insert(category.clone(), position);
            }
        }
        
        Self {
            category_map,
            data_bounds,
            screen_width: width,
            screen_height: height,
            margins,
        }
    }
    
    pub fn data_to_screen(&self, point: &DataPoint) -> Option<ScreenPoint> {
        let x_pos = match point {
            DataPoint::Numeric(x, _) => *x,
            DataPoint::Categorical(category, _) => {
                *self.category_map.get(category)?
            }
        };
        
        let y_pos = point.y();
        
        let numeric_coord = NumericCoordinate::new(x_pos, y_pos);
        let transformer = CoordinateTransformer::new(self.data_bounds.clone(), self.screen_width, self.screen_height, self.margins);
        transformer.data_to_screen(numeric_coord)
    }
    
    pub fn get_category_position(&self, category: &str) -> Option<f64> {
        self.category_map.get(category).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_bounds_from_points() {
        let coords = vec![
            NumericCoordinate::new(1.0, 2.0),
            NumericCoordinate::new(5.0, 8.0),
            NumericCoordinate::new(3.0, 4.0),
        ];
        
        let bounds = DataBounds::from_numeric_coordinates(&coords);
        
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
        
        let data_coord = NumericCoordinate::new(5.0, 50.0);
        let screen_pt = transformer.data_to_screen(data_coord).unwrap();
        
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
        
        let original = NumericCoordinate::new(7.5, 25.0);
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
        
        let out_of_bounds = NumericCoordinate::new(15.0, 5.0);
        assert!(transformer.data_to_screen(out_of_bounds).is_none());
        
        let in_bounds = NumericCoordinate::new(5.0, 5.0);
        assert!(transformer.data_to_screen(in_bounds).is_some());
    }
}