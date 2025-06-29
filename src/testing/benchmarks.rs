use crate::testing::generators::*;
use std::collections::HashMap;

pub fn large_timeseries(size: usize) -> Vec<(f64, f64)> {
    sine_wave(size, 1.0, 1.0, 0.0)
}

pub fn wide_dataset(rows: usize, cols: usize) -> Vec<Vec<f64>> {
    let mut dataset = Vec::with_capacity(cols);
    
    for col in 0..cols {
        let mut column = Vec::with_capacity(rows);
        let frequency = (col + 1) as f64 * 0.1;
        let phase = col as f64 * std::f64::consts::PI / 4.0;
        
        for row in 0..rows {
            let x = row as f64 / 10.0;
            let y = (frequency * x + phase).sin();
            column.push(y);
        }
        
        dataset.push(column);
    }
    
    dataset
}

pub struct StreamingGenerator {
    current_index: usize,
    rate_hz: f64,
    amplitude: f64,
    frequency: f64,
}

impl StreamingGenerator {
    pub fn new(rate_hz: f64) -> Self {
        Self {
            current_index: 0,
            rate_hz,
            amplitude: 1.0,
            frequency: 1.0,
        }
    }
    
    pub fn with_parameters(rate_hz: f64, amplitude: f64, frequency: f64) -> Self {
        Self {
            current_index: 0,
            rate_hz,
            amplitude,
            frequency,
        }
    }
}

impl Iterator for StreamingGenerator {
    type Item = (f64, f64);
    
    fn next(&mut self) -> Option<Self::Item> {
        let t = self.current_index as f64 / self.rate_hz;
        let x = t;
        let y = self.amplitude * (self.frequency * 2.0 * std::f64::consts::PI * t).sin();
        
        self.current_index += 1;
        Some((x, y))
    }
}

pub fn streaming_generator(rate_hz: f64) -> StreamingGenerator {
    StreamingGenerator::new(rate_hz)
}

pub struct BenchmarkDatasets;

impl BenchmarkDatasets {
    pub fn small_linear() -> Vec<(f64, f64)> {
        linear_data(100, 1.0, 0.0, 0.1)
    }
    
    pub fn medium_sine() -> Vec<(f64, f64)> {
        sine_wave(1000, 1.0, 1.0, 0.0)
    }
    
    pub fn large_complex() -> Vec<(f64, f64)> {
        multi_sine_wave(10000, &[
            (1.0, 1.0, 0.0),
            (0.5, 2.0, std::f64::consts::PI / 4.0),
            (0.25, 4.0, std::f64::consts::PI / 2.0),
        ])
    }
    
    pub fn memory_stress(size: usize) -> Vec<(f64, f64)> {
        linear_data(size, 1.0, 0.0, 1.0)
    }
    
    pub fn high_frequency_noise(size: usize) -> Vec<(f64, f64)> {
        noise_data(size, 1.0)
    }
    
    pub fn mixed_patterns(size: usize) -> Vec<(f64, f64)> {
        let mut data = Vec::with_capacity(size);
        let section_size = size / 4;
        
        // Linear section
        let linear = linear_data(section_size, 2.0, 0.0, 0.1);
        data.extend(linear);
        
        // Sine section
        let sine = sine_wave(section_size, 1.0, 1.0, 0.0);
        data.extend(sine.into_iter().map(|(x, y)| (x + section_size as f64, y)));
        
        // Exponential section
        let exp = exponential_data(section_size, 0.1, 0.1, 0.05);
        data.extend(exp.into_iter().map(|(x, y)| (x + 2.0 * section_size as f64, y)));
        
        // Noise section
        let noise = noise_data(section_size, 0.5);
        data.extend(noise.into_iter().map(|(x, y)| (x + 3.0 * section_size as f64, y)));
        
        data
    }
}

pub struct PerformanceTestData {
    pub name: String,
    pub data: Vec<(f64, f64)>,
    pub expected_parse_time_ms: u64,
    pub expected_render_time_ms: u64,
}

impl PerformanceTestData {
    pub fn new(name: String, data: Vec<(f64, f64)>, parse_time: u64, render_time: u64) -> Self {
        Self {
            name,
            data,
            expected_parse_time_ms: parse_time,
            expected_render_time_ms: render_time,
        }
    }
}

pub fn benchmark_datasets() -> Vec<PerformanceTestData> {
    vec![
        PerformanceTestData::new(
            "Small Linear (100 points)".to_string(),
            BenchmarkDatasets::small_linear(),
            1,  // 1ms parse
            5,  // 5ms render
        ),
        PerformanceTestData::new(
            "Medium Sine (1K points)".to_string(),
            BenchmarkDatasets::medium_sine(),
            5,   // 5ms parse
            20,  // 20ms render
        ),
        PerformanceTestData::new(
            "Large Complex (10K points)".to_string(),
            BenchmarkDatasets::large_complex(),
            50,  // 50ms parse
            100, // 100ms render
        ),
        PerformanceTestData::new(
            "Memory Stress (100K points)".to_string(),
            BenchmarkDatasets::memory_stress(100_000),
            200, // 200ms parse
            500, // 500ms render
        ),
    ]
}

pub struct EdgeCaseData;

impl EdgeCaseData {
    pub fn empty_dataset() -> Vec<(f64, f64)> {
        vec![]
    }
    
    pub fn single_point() -> Vec<(f64, f64)> {
        vec![(0.0, 0.0)]
    }
    
    pub fn two_points() -> Vec<(f64, f64)> {
        vec![(0.0, 0.0), (1.0, 1.0)]
    }
    
    pub fn large_numbers() -> Vec<(f64, f64)> {
        vec![
            (1e6, 2e6),
            (1e7, 2e7),
            (1e8, 2e8),
        ]
    }
    
    pub fn small_numbers() -> Vec<(f64, f64)> {
        vec![
            (1e-6, 2e-6),
            (1e-7, 2e-7),
            (1e-8, 2e-8),
        ]
    }
    
    pub fn negative_numbers() -> Vec<(f64, f64)> {
        vec![
            (-1.0, -2.0),
            (-10.0, -20.0),
            (-100.0, -200.0),
        ]
    }
    
    pub fn mixed_signs() -> Vec<(f64, f64)> {
        vec![
            (-1.0, 1.0),
            (1.0, -1.0),
            (-1.0, -1.0),
            (1.0, 1.0),
        ]
    }
    
    pub fn duplicate_x_values() -> Vec<(f64, f64)> {
        vec![
            (1.0, 1.0),
            (1.0, 2.0),
            (1.0, 3.0),
            (2.0, 4.0),
        ]
    }
    
    pub fn extreme_aspect_ratio() -> Vec<(f64, f64)> {
        // Very wide range in X, narrow in Y
        linear_data(1000, 0.001, 0.0, 0.0001)
    }
    
    pub fn all_same_y() -> Vec<(f64, f64)> {
        (0..100).map(|i| (i as f64, 5.0)).collect()
    }
    
    pub fn all_same_x() -> Vec<(f64, f64)> {
        (0..100).map(|i| (5.0, i as f64)).collect()
    }
}

pub fn unicode_test_data() -> Vec<String> {
    vec![
        "Basic ASCII".to_string(),
        "Émojis: 📊📈📉".to_string(),
        "Greek: αβγδε".to_string(),
        "Math: ∑∏∫∆∇".to_string(),
        "Arrows: ←→↑↓".to_string(),
        "Box: ┌┐└┘│─".to_string(),
        "Wide chars: 中文字符".to_string(),
    ]
}

pub struct MemoryTestData {
    pub size: usize,
    pub data: Vec<(f64, f64)>,
}

impl MemoryTestData {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            data: BenchmarkDatasets::memory_stress(size),
        }
    }
    
    pub fn estimated_memory_mb(&self) -> f64 {
        // Rough estimate: each point is 2 f64s (16 bytes) + overhead
        (self.size * 16) as f64 / 1_048_576.0
    }
}

pub fn memory_test_sizes() -> Vec<MemoryTestData> {
    vec![
        MemoryTestData::new(1_000),      // ~16 KB
        MemoryTestData::new(10_000),     // ~160 KB
        MemoryTestData::new(100_000),    // ~1.6 MB
        MemoryTestData::new(1_000_000),  // ~16 MB
    ]
}

pub struct ConcurrencyTestData {
    pub thread_count: usize,
    pub data_per_thread: Vec<(f64, f64)>,
}

impl ConcurrencyTestData {
    pub fn new(thread_count: usize, points_per_thread: usize) -> Self {
        Self {
            thread_count,
            data_per_thread: sine_wave(points_per_thread, 1.0, 1.0, 0.0),
        }
    }
}

pub fn concurrency_test_scenarios() -> Vec<ConcurrencyTestData> {
    vec![
        ConcurrencyTestData::new(2, 1000),
        ConcurrencyTestData::new(4, 1000),
        ConcurrencyTestData::new(8, 1000),
        ConcurrencyTestData::new(16, 1000),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_large_timeseries() {
        let data = large_timeseries(1000);
        assert_eq!(data.len(), 1000);
    }

    #[test]
    fn test_wide_dataset() {
        let data = wide_dataset(100, 10);
        assert_eq!(data.len(), 10); // 10 columns
        assert_eq!(data[0].len(), 100); // 100 rows
    }

    #[test]
    fn test_streaming_generator() {
        let mut gen = streaming_generator(10.0);
        let first = gen.next().unwrap();
        let second = gen.next().unwrap();
        
        assert_eq!(first.0, 0.0);
        assert_eq!(second.0, 0.1); // 1/10 second later
    }

    #[test]
    fn test_benchmark_datasets() {
        let datasets = benchmark_datasets();
        assert!(!datasets.is_empty());
        
        for dataset in datasets {
            assert!(!dataset.name.is_empty());
            assert!(!dataset.data.is_empty());
        }
    }

    #[test]
    fn test_edge_cases() {
        assert!(EdgeCaseData::empty_dataset().is_empty());
        assert_eq!(EdgeCaseData::single_point().len(), 1);
        assert_eq!(EdgeCaseData::two_points().len(), 2);
    }

    #[test]
    fn test_memory_test_data() {
        let mem_data = MemoryTestData::new(1000);
        assert_eq!(mem_data.size, 1000);
        assert_eq!(mem_data.data.len(), 1000);
        assert!(mem_data.estimated_memory_mb() > 0.0);
    }
}