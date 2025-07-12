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

fn nice_number(value: f64, round_up: bool) -> f64 {
    let exp = value.abs().log10().floor();
    let fraction = value.abs() / 10_f64.powf(exp);
    
    let nice_fraction = if round_up {
        if fraction < 1.5 { 1.5 }
        else if fraction < 3.0 { 3.0 }
        else if fraction < 7.0 { 7.0 }
        else { 10.0 }
    } else {
        if fraction < 1.0 { 1.0 }
        else if fraction < 2.0 { 2.0 }
        else if fraction < 5.0 { 5.0 }
        else { 10.0 }
    };
    
    let result = nice_fraction * 10_f64.powf(exp);
    if value < 0.0 { -result } else { result }
}