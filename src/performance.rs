use std::time::{Duration, Instant};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub operation: String,
    pub duration: Duration,
    pub memory_used_bytes: Option<usize>,
    pub throughput_items_per_sec: Option<f64>,
    pub timestamp: std::time::SystemTime,
}

impl PerformanceMetrics {
    pub fn new(operation: String, duration: Duration) -> Self {
        Self {
            operation,
            duration,
            memory_used_bytes: None,
            throughput_items_per_sec: None,
            timestamp: std::time::SystemTime::now(),
        }
    }

    pub fn with_memory(mut self, memory_bytes: usize) -> Self {
        self.memory_used_bytes = Some(memory_bytes);
        self
    }

    pub fn with_throughput(mut self, items_processed: usize) -> Self {
        if self.duration.as_secs_f64() > 0.0 {
            self.throughput_items_per_sec = Some(items_processed as f64 / self.duration.as_secs_f64());
        }
        self
    }
}

pub struct PerformanceMonitor {
    metrics: Vec<PerformanceMetrics>,
    active_timers: HashMap<String, Instant>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics: Vec::new(),
            active_timers: HashMap::new(),
        }
    }

    pub fn start_timer(&mut self, operation: &str) {
        self.active_timers.insert(operation.to_string(), Instant::now());
    }

    pub fn end_timer(&mut self, operation: &str) -> Option<Duration> {
        if let Some(start_time) = self.active_timers.remove(operation) {
            let duration = start_time.elapsed();
            let metric = PerformanceMetrics::new(operation.to_string(), duration);
            self.metrics.push(metric);
            Some(duration)
        } else {
            None
        }
    }

    pub fn end_timer_with_throughput(&mut self, operation: &str, items_processed: usize) -> Option<Duration> {
        if let Some(start_time) = self.active_timers.remove(operation) {
            let duration = start_time.elapsed();
            let metric = PerformanceMetrics::new(operation.to_string(), duration)
                .with_throughput(items_processed);
            self.metrics.push(metric);
            Some(duration)
        } else {
            None
        }
    }

    pub fn record_metric(&mut self, metric: PerformanceMetrics) {
        self.metrics.push(metric);
    }

    pub fn get_metrics(&self) -> &[PerformanceMetrics] {
        &self.metrics
    }

    pub fn get_metrics_for_operation(&self, operation: &str) -> Vec<&PerformanceMetrics> {
        self.metrics.iter()
            .filter(|m| m.operation == operation)
            .collect()
    }

    pub fn average_duration_for_operation(&self, operation: &str) -> Option<Duration> {
        let metrics = self.get_metrics_for_operation(operation);
        if metrics.is_empty() {
            return None;
        }

        let total_nanos: u128 = metrics.iter()
            .map(|m| m.duration.as_nanos())
            .sum();
        let average_nanos = total_nanos / metrics.len() as u128;
        
        Some(Duration::from_nanos(average_nanos as u64))
    }

    pub fn clear(&mut self) {
        self.metrics.clear();
        self.active_timers.clear();
    }

    pub fn print_summary(&self) {
        if self.metrics.is_empty() {
            println!("No performance metrics recorded.");
            return;
        }

        println!("Performance Summary");
        println!("==================");

        // Group by operation
        let mut operation_metrics: HashMap<String, Vec<&PerformanceMetrics>> = HashMap::new();
        for metric in &self.metrics {
            operation_metrics.entry(metric.operation.clone())
                .or_insert_with(Vec::new)
                .push(metric);
        }

        for (operation, metrics) in operation_metrics {
            println!("\nOperation: {}", operation);
            println!("  Count: {}", metrics.len());
            
            let durations: Vec<Duration> = metrics.iter().map(|m| m.duration).collect();
            let total_duration: Duration = durations.iter().sum();
            let avg_duration = total_duration / durations.len() as u32;
            let min_duration = durations.iter().min().unwrap();
            let max_duration = durations.iter().max().unwrap();

            println!("  Total time: {:.3?}", total_duration);
            println!("  Average time: {:.3?}", avg_duration);
            println!("  Min time: {:.3?}", min_duration);
            println!("  Max time: {:.3?}", max_duration);

            // Throughput stats
            let throughputs: Vec<f64> = metrics.iter()
                .filter_map(|m| m.throughput_items_per_sec)
                .collect();
            
            if !throughputs.is_empty() {
                let avg_throughput = throughputs.iter().sum::<f64>() / throughputs.len() as f64;
                let max_throughput = throughputs.iter().fold(0.0f64, |a, &b| a.max(b));
                println!("  Average throughput: {:.0} items/sec", avg_throughput);
                println!("  Max throughput: {:.0} items/sec", max_throughput);
            }

            // Memory stats
            let memory_usages: Vec<usize> = metrics.iter()
                .filter_map(|m| m.memory_used_bytes)
                .collect();
            
            if !memory_usages.is_empty() {
                let avg_memory = memory_usages.iter().sum::<usize>() / memory_usages.len();
                let max_memory = memory_usages.iter().max().unwrap();
                println!("  Average memory: {:.2} MB", avg_memory as f64 / 1_048_576.0);
                println!("  Max memory: {:.2} MB", *max_memory as f64 / 1_048_576.0);
            }
        }
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

pub fn time_function<F, R>(operation_name: &str, f: F) -> (R, PerformanceMetrics)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    
    let metric = PerformanceMetrics::new(operation_name.to_string(), duration);
    (result, metric)
}

pub fn time_function_with_throughput<F, R>(
    operation_name: &str, 
    items_processed: usize,
    f: F
) -> (R, PerformanceMetrics)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    
    let metric = PerformanceMetrics::new(operation_name.to_string(), duration)
        .with_throughput(items_processed);
    (result, metric)
}

pub struct MemoryProfiler {
    peak_usage: usize,
    current_usage: usize,
    allocations: HashMap<String, usize>,
}

impl MemoryProfiler {
    pub fn new() -> Self {
        Self {
            peak_usage: 0,
            current_usage: 0,
            allocations: HashMap::new(),
        }
    }

    pub fn record_allocation(&mut self, category: &str, bytes: usize) {
        self.current_usage += bytes;
        self.peak_usage = self.peak_usage.max(self.current_usage);
        *self.allocations.entry(category.to_string()).or_insert(0) += bytes;
    }

    pub fn record_deallocation(&mut self, bytes: usize) {
        self.current_usage = self.current_usage.saturating_sub(bytes);
    }

    pub fn peak_usage_mb(&self) -> f64 {
        self.peak_usage as f64 / 1_048_576.0
    }

    pub fn current_usage_mb(&self) -> f64 {
        self.current_usage as f64 / 1_048_576.0
    }

    pub fn print_summary(&self) {
        println!("Memory Usage Summary");
        println!("===================");
        println!("Peak usage: {:.2} MB", self.peak_usage_mb());
        println!("Current usage: {:.2} MB", self.current_usage_mb());
        
        if !self.allocations.is_empty() {
            println!("\nAllocations by category:");
            let mut categories: Vec<_> = self.allocations.iter().collect();
            categories.sort_by(|a, b| b.1.cmp(a.1)); // Sort by size descending
            
            for (category, bytes) in categories {
                println!("  {}: {:.2} MB", category, *bytes as f64 / 1_048_576.0);
            }
        }
    }
}

impl Default for MemoryProfiler {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BenchmarkRunner {
    monitor: PerformanceMonitor,
    memory_profiler: MemoryProfiler,
}

impl BenchmarkRunner {
    pub fn new() -> Self {
        Self {
            monitor: PerformanceMonitor::new(),
            memory_profiler: MemoryProfiler::new(),
        }
    }

    pub fn benchmark<F, R>(&mut self, name: &str, iterations: usize, f: F) -> Vec<R>
    where
        F: Fn() -> R,
    {
        let mut results = Vec::with_capacity(iterations);
        
        for i in 0..iterations {
            let iteration_name = format!("{}_iter_{}", name, i);
            self.monitor.start_timer(&iteration_name);
            
            let result = f();
            results.push(result);
            
            self.monitor.end_timer(&iteration_name);
        }
        
        results
    }

    pub fn benchmark_with_warmup<F, R>(&mut self, name: &str, warmup: usize, iterations: usize, f: F) -> Vec<R>
    where
        F: Fn() -> R,
    {
        // Warmup iterations (not measured)
        for _ in 0..warmup {
            let _ = f();
        }
        
        // Measured iterations
        self.benchmark(name, iterations, f)
    }

    pub fn get_monitor(&self) -> &PerformanceMonitor {
        &self.monitor
    }

    pub fn get_memory_profiler(&self) -> &MemoryProfiler {
        &self.memory_profiler
    }

    pub fn print_report(&self) {
        self.monitor.print_summary();
        println!();
        self.memory_profiler.print_summary();
    }
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_performance_monitor() {
        let mut monitor = PerformanceMonitor::new();
        
        monitor.start_timer("test_operation");
        sleep(Duration::from_millis(10));
        let duration = monitor.end_timer("test_operation");
        
        assert!(duration.is_some());
        assert!(duration.unwrap() >= Duration::from_millis(10));
        assert_eq!(monitor.get_metrics().len(), 1);
    }

    #[test]
    fn test_time_function() {
        let (result, metric) = time_function("test_fn", || {
            sleep(Duration::from_millis(1));
            42
        });
        
        assert_eq!(result, 42);
        assert_eq!(metric.operation, "test_fn");
        assert!(metric.duration >= Duration::from_millis(1));
    }

    #[test]
    fn test_throughput_calculation() {
        let metric = PerformanceMetrics::new("test".to_string(), Duration::from_secs(1))
            .with_throughput(1000);
        
        assert_eq!(metric.throughput_items_per_sec, Some(1000.0));
    }

    #[test]
    fn test_memory_profiler() {
        let mut profiler = MemoryProfiler::new();
        
        profiler.record_allocation("test", 1_048_576); // 1 MB
        assert_eq!(profiler.current_usage_mb(), 1.0);
        assert_eq!(profiler.peak_usage_mb(), 1.0);
        
        profiler.record_deallocation(524_288); // 0.5 MB
        assert_eq!(profiler.current_usage_mb(), 0.5);
        assert_eq!(profiler.peak_usage_mb(), 1.0); // Peak should remain
    }

    #[test]
    fn test_benchmark_runner() {
        let mut runner = BenchmarkRunner::new();
        
        let results = runner.benchmark("test_bench", 3, || {
            sleep(Duration::from_millis(1));
            42
        });
        
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|&x| x == 42));
        assert!(runner.get_monitor().get_metrics().len() >= 3);
    }
}