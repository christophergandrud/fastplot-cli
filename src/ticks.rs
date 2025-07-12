#[derive(Debug, Clone)]
pub struct Tick {
    pub value: f64,
    pub label: String,
    #[allow(dead_code)]
    pub is_major: bool,
}

pub struct TickGenerator {
    #[allow(dead_code)]
    min_ticks: usize,
    max_ticks: usize,
}

impl Default for TickGenerator {
    fn default() -> Self {
        Self {
            min_ticks: 4,
            max_ticks: 8,
        }
    }
}

impl TickGenerator {
    #[allow(dead_code)]
    pub fn new(min_ticks: usize, max_ticks: usize) -> Self {
        Self { min_ticks, max_ticks }
    }

    pub fn generate_ticks(&self, min: f64, max: f64) -> Vec<Tick> {
        if min >= max {
            return vec![
                Tick {
                    value: min,
                    label: self.format_tick_label(min, 1.0),
                    is_major: true,
                }
            ];
        }

        let range = max - min;
        let raw_step = range / (self.max_ticks - 1) as f64;
        let nice_step = self.nice_number(raw_step, false);
        
        let nice_min = (min / nice_step).floor() * nice_step;
        let nice_max = (max / nice_step).ceil() * nice_step;
        
        let mut ticks = Vec::new();
        let mut value = nice_min;
        
        while value <= nice_max + nice_step * 0.5 {
            if value >= min - nice_step * 0.1 && value <= max + nice_step * 0.1 {
                ticks.push(Tick {
                    value,
                    label: self.format_tick_label(value, nice_step),
                    is_major: true,
                });
            }
            value += nice_step;
        }
        
        if ticks.is_empty() {
            ticks.push(Tick {
                value: (min + max) / 2.0,
                label: self.format_tick_label((min + max) / 2.0, range),
                is_major: true,
            });
        }
        
        ticks
    }

    fn nice_number(&self, value: f64, round: bool) -> f64 {
        if value == 0.0 {
            return 1.0;
        }
        
        let exponent = value.log10().floor();
        let fraction = value / 10f64.powf(exponent);
        
        let nice_fraction = if round {
            if fraction < 1.5 { 1.0 }
            else if fraction < 3.0 { 2.0 }
            else if fraction < 7.0 { 5.0 }
            else { 10.0 }
        } else {
            if fraction <= 1.0 { 1.0 }
            else if fraction <= 2.0 { 2.0 }
            else if fraction <= 5.0 { 5.0 }
            else { 10.0 }
        };
        
        nice_fraction * 10f64.powf(exponent)
    }

    fn format_tick_label(&self, value: f64, step: f64) -> String {
        if value.abs() < 1e-10 {
            return "0".to_string();
        }

        let precision = if step >= 1.0 {
            0
        } else {
            let log_step = -step.log10();
            (log_step.ceil() as usize).min(6)
        };
        
        if precision == 0 {
            format!("{}", value.round() as i64)
        } else {
            let formatted = format!("{:.prec$}", value, prec = precision);
            formatted.trim_end_matches('0').trim_end_matches('.').to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice_number() {
        let gen = TickGenerator::default();
        
        assert_eq!(gen.nice_number(1.3, false), 2.0);
        assert_eq!(gen.nice_number(2.7, false), 5.0);
        assert_eq!(gen.nice_number(8.2, false), 10.0);
        assert_eq!(gen.nice_number(0.23, false), 0.5);
    }

    #[test]
    fn test_generate_ticks_simple() {
        let gen = TickGenerator::default();
        let ticks = gen.generate_ticks(0.0, 10.0);
        
        assert!(!ticks.is_empty());
        assert!(ticks.len() >= 3);
        assert!(ticks.len() <= 8);
        
        for tick in &ticks {
            assert!(tick.value >= -1.0 && tick.value <= 11.0);
        }
    }

    #[test]
    fn test_generate_ticks_negative() {
        let gen = TickGenerator::default();
        let ticks = gen.generate_ticks(-5.0, 5.0);
        
        assert!(!ticks.is_empty());
        assert!(ticks.iter().any(|t| t.value <= 0.0));
        assert!(ticks.iter().any(|t| t.value >= 0.0));
    }

    #[test]
    fn test_format_tick_label() {
        let gen = TickGenerator::default();
        
        assert_eq!(gen.format_tick_label(5.0, 1.0), "5");
        assert_eq!(gen.format_tick_label(0.0, 0.1), "0");
        assert_eq!(gen.format_tick_label(2.5, 0.5), "2.5");
    }
}