use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::f64::consts::PI;

pub struct DataGenerator {
    rng: StdRng,
}

impl DataGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn with_default_seed() -> Self {
        Self::new(42) // Deterministic seed for reproducible tests
    }
}

impl Default for DataGenerator {
    fn default() -> Self {
        Self::with_default_seed()
    }
}

pub fn linear_data(n: usize, slope: f64, intercept: f64, noise: f64) -> Vec<(f64, f64)> {
    let mut generator = DataGenerator::default();
    let mut data = Vec::with_capacity(n);
    
    for i in 0..n {
        let x = i as f64;
        let y_perfect = slope * x + intercept;
        let noise_val = (generator.rng.gen::<f64>() - 0.5) * 2.0 * noise;
        let y = y_perfect + noise_val;
        data.push((x, y));
    }
    
    data
}

pub fn sine_wave(n: usize, amplitude: f64, frequency: f64, phase: f64) -> Vec<(f64, f64)> {
    let mut data = Vec::with_capacity(n);
    
    for i in 0..n {
        let x = i as f64 / n as f64 * 4.0 * PI; // 4π range
        let y = amplitude * (frequency * x + phase).sin();
        data.push((x, y));
    }
    
    data
}

pub fn cosine_wave(n: usize, amplitude: f64, frequency: f64, phase: f64) -> Vec<(f64, f64)> {
    let mut data = Vec::with_capacity(n);
    
    for i in 0..n {
        let x = i as f64 / n as f64 * 4.0 * PI;
        let y = amplitude * (frequency * x + phase).cos();
        data.push((x, y));
    }
    
    data
}

pub fn exponential_data(n: usize, base: f64, rate: f64, noise: f64) -> Vec<(f64, f64)> {
    let mut generator = DataGenerator::default();
    let mut data = Vec::with_capacity(n);
    
    for i in 0..n {
        let x = i as f64 / 10.0; // Scale x values
        let y_perfect = base * (rate * x).exp();
        let noise_val = (generator.rng.gen::<f64>() - 0.5) * 2.0 * noise;
        let y = y_perfect + noise_val;
        data.push((x, y));
    }
    
    data
}

pub fn logarithmic_data(n: usize, base: f64, scale: f64, noise: f64) -> Vec<(f64, f64)> {
    let mut generator = DataGenerator::default();
    let mut data = Vec::with_capacity(n);
    
    for i in 0..n {
        let x = (i + 1) as f64; // Start from 1 to avoid log(0)
        let y_perfect = base + scale * x.ln();
        let noise_val = (generator.rng.gen::<f64>() - 0.5) * 2.0 * noise;
        let y = y_perfect + noise_val;
        data.push((x, y));
    }
    
    data
}

pub fn quadratic_data(n: usize, a: f64, b: f64, c: f64, noise: f64) -> Vec<(f64, f64)> {
    let mut generator = DataGenerator::default();
    let mut data = Vec::with_capacity(n);
    
    for i in 0..n {
        let x = i as f64 / 10.0 - n as f64 / 20.0; // Center around 0
        let y_perfect = a * x * x + b * x + c;
        let noise_val = (generator.rng.gen::<f64>() - 0.5) * 2.0 * noise;
        let y = y_perfect + noise_val;
        data.push((x, y));
    }
    
    data
}

pub fn random_walk(n: usize, step_size: f64) -> Vec<(f64, f64)> {
    let mut generator = DataGenerator::default();
    let mut data = Vec::with_capacity(n);
    let mut y = 0.0;
    
    for i in 0..n {
        let x = i as f64;
        let step = (generator.rng.gen::<f64>() - 0.5) * 2.0 * step_size;
        y += step;
        data.push((x, y));
    }
    
    data
}

pub fn normal_distribution(n: usize, mean: f64, std_dev: f64) -> Vec<f64> {
    let mut generator = DataGenerator::default();
    let mut data = Vec::with_capacity(n);
    
    for _ in 0..n {
        // Box-Muller transform for normal distribution
        let u1: f64 = generator.rng.gen();
        let u2: f64 = generator.rng.gen();
        let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos();
        let value = mean + std_dev * z0;
        data.push(value);
    }
    
    data
}

pub fn uniform_distribution(n: usize, min: f64, max: f64) -> Vec<f64> {
    let mut generator = DataGenerator::default();
    let mut data = Vec::with_capacity(n);
    
    for _ in 0..n {
        let value = min + (max - min) * generator.rng.gen::<f64>();
        data.push(value);
    }
    
    data
}

pub fn multi_sine_wave(n: usize, components: &[(f64, f64, f64)]) -> Vec<(f64, f64)> {
    let mut data = Vec::with_capacity(n);
    
    for i in 0..n {
        let x = i as f64 / n as f64 * 4.0 * PI;
        let mut y = 0.0;
        
        for &(amplitude, frequency, phase) in components {
            y += amplitude * (frequency * x + phase).sin();
        }
        
        data.push((x, y));
    }
    
    data
}

pub fn damped_oscillation(n: usize, amplitude: f64, frequency: f64, decay: f64) -> Vec<(f64, f64)> {
    let mut data = Vec::with_capacity(n);
    
    for i in 0..n {
        let x = i as f64 / 10.0;
        let damping = (-decay * x).exp();
        let y = amplitude * damping * (frequency * x).sin();
        data.push((x, y));
    }
    
    data
}

pub fn spiral_data(n: usize, radius_growth: f64, turns: f64) -> Vec<(f64, f64)> {
    let mut data = Vec::with_capacity(n);
    
    for i in 0..n {
        let t = i as f64 / n as f64 * turns * 2.0 * PI;
        let r = radius_growth * t;
        let x = r * t.cos();
        let y = r * t.sin();
        data.push((x, y));
    }
    
    data
}

pub fn step_function(n: usize, step_width: usize, step_height: f64) -> Vec<(f64, f64)> {
    let mut data = Vec::with_capacity(n);
    
    for i in 0..n {
        let x = i as f64;
        let step_number = (i / step_width) as f64;
        let y = step_number * step_height;
        data.push((x, y));
    }
    
    data
}

pub fn sawtooth_wave(n: usize, amplitude: f64, period: f64) -> Vec<(f64, f64)> {
    let mut data = Vec::with_capacity(n);
    
    for i in 0..n {
        let x = i as f64;
        let phase = (x / period) % 1.0;
        let y = amplitude * (2.0 * phase - 1.0);
        data.push((x, y));
    }
    
    data
}

pub fn noise_data(n: usize, noise_level: f64) -> Vec<(f64, f64)> {
    let mut generator = DataGenerator::default();
    let mut data = Vec::with_capacity(n);
    
    for i in 0..n {
        let x = i as f64;
        let y = (generator.rng.gen::<f64>() - 0.5) * 2.0 * noise_level;
        data.push((x, y));
    }
    
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_data() {
        let data = linear_data(100, 2.0, 1.0, 0.1);
        assert_eq!(data.len(), 100);
        
        // Check first point is approximately y = 2*0 + 1 = 1
        assert!((data[0].1 - 1.0).abs() < 0.5); // Allow for noise
        
        // Check that it's roughly linear
        let slope = (data[50].1 - data[0].1) / (data[50].0 - data[0].0);
        assert!((slope - 2.0).abs() < 0.5); // Should be close to 2.0
    }

    #[test]
    fn test_sine_wave() {
        let data = sine_wave(100, 1.0, 1.0, 0.0);
        assert_eq!(data.len(), 100);
        
        // Sine wave should start near 0
        assert!(data[0].1.abs() < 0.1);
        
        // Should have values between -1 and 1
        for (_, y) in &data {
            assert!(y.abs() <= 1.1); // Allow small margin for floating point
        }
    }

    #[test]
    fn test_normal_distribution() {
        let data = normal_distribution(1000, 0.0, 1.0);
        assert_eq!(data.len(), 1000);
        
        // Calculate sample mean and std dev
        let mean: f64 = data.iter().sum::<f64>() / data.len() as f64;
        let variance: f64 = data.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / data.len() as f64;
        let std_dev = variance.sqrt();
        
        // Should be close to N(0,1)
        assert!(mean.abs() < 0.1);
        assert!((std_dev - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_random_walk() {
        let data = random_walk(100, 0.1);
        assert_eq!(data.len(), 100);
        
        // First point should have x=0
        assert_eq!(data[0].0, 0.0);
        // Y value will vary due to random step
        
        // X values should be sequential
        for i in 1..data.len() {
            assert_eq!(data[i].0, i as f64);
        }
    }

    #[test]
    fn test_deterministic_output() {
        // Same seed should produce same results
        let data1 = linear_data(50, 1.0, 0.0, 0.1);
        let data2 = linear_data(50, 1.0, 0.0, 0.1);
        
        for i in 0..50 {
            assert_eq!(data1[i].0, data2[i].0);
            assert_eq!(data1[i].1, data2[i].1);
        }
    }
}