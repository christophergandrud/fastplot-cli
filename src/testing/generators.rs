use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::f64::consts::PI;

// Test data generators - marked as dead_code since they're only used in tests/examples

#[allow(dead_code)]
pub fn linear_data(n: usize, slope: f64, intercept: f64, noise: f64) -> Vec<(f64, f64)> {
    let mut rng = StdRng::seed_from_u64(42);
    let mut data = Vec::with_capacity(n);
    
    for i in 0..n {
        let x = i as f64;
        let y_perfect = slope * x + intercept;
        let noise_val = (rng.random::<f64>() - 0.5) * 2.0 * noise;
        let y = y_perfect + noise_val;
        data.push((x, y));
    }
    
    data
}

#[allow(dead_code)]
pub fn sine_wave(n: usize, amplitude: f64, frequency: f64, phase: f64) -> Vec<(f64, f64)> {
    let mut data = Vec::with_capacity(n);
    
    for i in 0..n {
        let x = i as f64 / n as f64 * 4.0 * PI;
        let y = amplitude * (frequency * x + phase).sin();
        data.push((x, y));
    }
    
    data
}

#[allow(dead_code)]
pub fn random_walk(n: usize, step_size: f64) -> Vec<(f64, f64)> {
    let mut rng = StdRng::seed_from_u64(42);
    let mut data = Vec::with_capacity(n);
    let mut y = 0.0;
    
    for i in 0..n {
        let x = i as f64;
        let step = (rng.random::<f64>() - 0.5) * 2.0 * step_size;
        y += step;
        data.push((x, y));
    }
    
    data
}

#[allow(dead_code)]
pub fn normal_distribution(n: usize, mean: f64, std_dev: f64) -> Vec<f64> {
    let mut rng = StdRng::seed_from_u64(42);
    let mut data = Vec::with_capacity(n);
    
    for _ in 0..n {
        // Box-Muller transform for normal distribution
        let u1: f64 = rng.random();
        let u2: f64 = rng.random();
        let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos();
        data.push(mean + std_dev * z0);
    }
    
    data
}

#[allow(dead_code)]
pub fn test_points(n: usize) -> Vec<(f64, f64)> {
    (0..n).map(|i| (i as f64, (i * 2) as f64)).collect()
}