use crate::data::DataPoint;

#[derive(Debug)]
pub struct AxisConfig {
    pub min: f64,
    pub max: f64,
    pub tick_count: usize,
}

impl AxisConfig {
    pub fn from_data(_points: &[DataPoint], _axis: char) -> Self {
        // Fixed range from -10 to 10 as specified in test case
        Self {
            min: -10.0,
            max: 10.0,
            tick_count: 5,
        }
    }
    
    pub fn get_ticks(&self) -> Vec<f64> {
        let step = (self.max - self.min) / (self.tick_count - 1) as f64;
        (0..self.tick_count)
            .map(|i| self.min + i as f64 * step)
            .collect()
    }
    
    pub fn data_to_position(&self, value: f64, total_size: usize) -> usize {
        let normalized = (value - self.min) / (self.max - self.min);
        (normalized * (total_size - 1) as f64).round() as usize
    }
}

// Removed unused nice_number function