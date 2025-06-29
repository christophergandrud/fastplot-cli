use fastplot_cli::data::{FastParser, PlotConfig};
use fastplot_cli::plot::{Plot, LinePlot, Canvas};
use std::time::{Instant, Duration};

const PERFORMANCE_TIMEOUT: Duration = Duration::from_secs(10);

fn generate_test_data(size: usize) -> String {
    let mut data = String::from("x,y\n");
    for i in 0..size {
        let x = i as f64 / 100.0;
        let y = (x * 2.0 * std::f64::consts::PI).sin();
        data.push_str(&format!("{:.6},{:.6}\n", x, y));
    }
    data
}

#[test]
fn test_parsing_performance_small() {
    // Test parsing performance for small datasets
    let data = generate_test_data(1000);
    let parser = FastParser::new(',', true);
    
    let start = Instant::now();
    let df = parser.parse_string(&data).unwrap();
    let duration = start.elapsed();
    
    assert_eq!(df.num_rows(), 1000);
    assert!(duration < Duration::from_millis(100), 
           "Parsing 1000 rows took too long: {:?}", duration);
    
    // Should process at least 10,000 rows per second
    let rows_per_sec = 1000.0 / duration.as_secs_f64();
    assert!(rows_per_sec > 10_000.0, 
           "Parse rate too slow: {:.0} rows/sec", rows_per_sec);
}

#[test]
fn test_parsing_performance_medium() {
    // Test parsing performance for medium datasets
    let data = generate_test_data(10_000);
    let parser = FastParser::new(',', true);
    
    let start = Instant::now();
    let df = parser.parse_string(&data).unwrap();
    let duration = start.elapsed();
    
    assert_eq!(df.num_rows(), 10_000);
    assert!(duration < Duration::from_millis(500), 
           "Parsing 10,000 rows took too long: {:?}", duration);
    
    // Should process at least 20,000 rows per second
    let rows_per_sec = 10_000.0 / duration.as_secs_f64();
    assert!(rows_per_sec > 20_000.0, 
           "Parse rate too slow: {:.0} rows/sec", rows_per_sec);
}

#[test]
fn test_parsing_performance_large() {
    // Test parsing performance for large datasets
    let data = generate_test_data(100_000);
    let parser = FastParser::new(',', true);
    
    let start = Instant::now();
    let df = parser.parse_string(&data).unwrap();
    let duration = start.elapsed();
    
    assert_eq!(df.num_rows(), 100_000);
    assert!(duration < Duration::from_secs(2), 
           "Parsing 100,000 rows took too long: {:?}", duration);
    
    // Should process at least 50,000 rows per second
    let rows_per_sec = 100_000.0 / duration.as_secs_f64();
    assert!(rows_per_sec > 50_000.0, 
           "Parse rate too slow: {:.0} rows/sec", rows_per_sec);
}

#[test]
fn test_rendering_performance() {
    // Test plot rendering performance
    let data = generate_test_data(10_000);
    let parser = FastParser::new(',', true);
    let df = parser.parse_string(&data).unwrap();
    
    let config = PlotConfig {
        width: 100,
        height: 30,
        title: Some("Performance Test".to_string()),
        ..Default::default()
    };
    
    let plot = LinePlot;
    
    let start = Instant::now();
    let result = plot.render(&df, &config).unwrap();
    let duration = start.elapsed();
    
    assert!(!result.is_empty());
    assert!(duration < Duration::from_millis(200), 
           "Rendering took too long: {:?}", duration);
    
    // Should render at least 50,000 points per second
    let points_per_sec = 10_000.0 / duration.as_secs_f64();
    assert!(points_per_sec > 50_000.0, 
           "Render rate too slow: {:.0} points/sec", points_per_sec);
}

#[test]
fn test_canvas_performance() {
    // Test canvas operations performance
    let mut canvas = Canvas::new(100, 30);
    canvas.set_ranges((0.0, 1000.0), (-1.0, 1.0));
    
    let start = Instant::now();
    
    // Plot 1000 points
    for i in 0..1000 {
        let x = i as f64;
        let y = (x / 100.0).sin();
        canvas.plot_point(x, y, '*');
    }
    
    let plot_duration = start.elapsed();
    
    // Render the canvas
    let render_start = Instant::now();
    let output = canvas.render();
    let render_duration = render_start.elapsed();
    
    assert!(!output.is_empty());
    assert!(plot_duration < Duration::from_millis(50), 
           "Plotting points took too long: {:?}", plot_duration);
    assert!(render_duration < Duration::from_millis(10), 
           "Canvas rendering took too long: {:?}", render_duration);
}

#[test]
fn test_memory_efficiency() {
    // Test that memory usage is reasonable
    let sizes = [1_000, 10_000, 50_000];
    
    for size in sizes {
        let data = generate_test_data(size);
        let parser = FastParser::new(',', true);
        
        // Parse data
        let df = parser.parse_string(&data).unwrap();
        
        // Estimate memory usage (rough calculation)
        // Each f64 is 8 bytes, plus overhead
        let expected_memory = size * 2 * 8; // 2 columns, 8 bytes per f64
        let data_memory = df.columns.iter()
            .map(|col| col.data.len() * 8)
            .sum::<usize>();
        
        // Memory usage should be close to expected (within 50% overhead for structures)
        assert!(data_memory <= expected_memory * 3 / 2, 
               "Memory usage too high for {} rows: {} bytes vs expected {}",
               size, data_memory, expected_memory);
    }
}

#[test]
fn test_repeated_operations_performance() {
    // Test that repeated operations maintain performance (no memory leaks)
    let data = generate_test_data(1000);
    let parser = FastParser::new(',', true);
    let config = PlotConfig::default();
    let plot = LinePlot;
    
    let mut durations = Vec::new();
    
    // Perform operation 10 times
    for _ in 0..10 {
        let start = Instant::now();
        
        let df = parser.parse_string(&data).unwrap();
        let _result = plot.render(&df, &config).unwrap();
        
        durations.push(start.elapsed());
    }
    
    // Check that performance doesn't degrade significantly
    let first_duration = durations[0];
    let last_duration = durations[9];
    
    // Last operation should not be more than 2x slower than first
    assert!(last_duration < first_duration * 2, 
           "Performance degraded: first={:?}, last={:?}", 
           first_duration, last_duration);
    
    // Average should be reasonable
    let avg_duration: Duration = durations.iter().sum::<Duration>() / durations.len() as u32;
    assert!(avg_duration < Duration::from_millis(50), 
           "Average operation time too slow: {:?}", avg_duration);
}

#[test]
fn test_concurrent_operations() {
    // Test that operations can be performed concurrently without issues
    use std::thread;
    
    let handles: Vec<_> = (0..4).map(|i| {
        thread::spawn(move || {
            let data = generate_test_data(1000);
            let parser = FastParser::new(',', true);
            let config = PlotConfig {
                title: Some(format!("Thread {}", i)),
                ..Default::default()
            };
            let plot = LinePlot;
            
            let start = Instant::now();
            let df = parser.parse_string(&data).unwrap();
            let result = plot.render(&df, &config).unwrap();
            let duration = start.elapsed();
            
            (result.len(), duration)
        })
    }).collect();
    
    // Collect results
    let results: Vec<_> = handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    
    // All threads should complete successfully
    assert_eq!(results.len(), 4);
    
    // All should have reasonable performance
    for (output_len, duration) in results {
        assert!(output_len > 0);
        assert!(duration < Duration::from_millis(100), 
               "Concurrent operation took too long: {:?}", duration);
    }
}

#[test]
fn test_startup_performance() {
    // Test that initial operations are fast (no slow initialization)
    let data = generate_test_data(100);
    
    // Time the very first operation
    let start = Instant::now();
    let parser = FastParser::new(',', true);
    let df = parser.parse_string(&data).unwrap();
    let config = PlotConfig::default();
    let plot = LinePlot;
    let _result = plot.render(&df, &config).unwrap();
    let duration = start.elapsed();
    
    // First operation should be fast (no slow lazy initialization)
    assert!(duration < Duration::from_millis(50), 
           "Startup operation took too long: {:?}", duration);
}

#[test]
fn test_scalability() {
    // Test that performance scales reasonably with data size
    let sizes = [100, 1000, 10000];
    let mut parse_times = Vec::new();
    
    for size in sizes {
        let data = generate_test_data(size);
        let parser = FastParser::new(',', true);
        
        let start = Instant::now();
        let _df = parser.parse_string(&data).unwrap();
        let duration = start.elapsed();
        
        parse_times.push((size, duration));
    }
    
    // Performance should scale roughly linearly
    // 10x more data should not take more than 20x the time
    let (size1, time1) = parse_times[0];
    let (size3, time3) = parse_times[2];
    
    let size_ratio = size3 as f64 / size1 as f64;
    let time_ratio = time3.as_secs_f64() / time1.as_secs_f64();
    
    assert!(time_ratio < size_ratio * 2.0, 
           "Performance scaling too poor: {}x size increase caused {}x time increase",
           size_ratio, time_ratio);
}