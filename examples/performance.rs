use fastplot_cli::data::{FastParser, PlotConfig};
use fastplot_cli::plot::{Plot, LinePlot, Canvas};
use std::time::{Instant, Duration};
use anyhow::Result;

fn generate_large_dataset(size: usize) -> String {
    let mut data = String::from("x,y\n");
    for i in 0..size {
        let x = i as f64 / 100.0;
        let y = (x * 2.0 * std::f64::consts::PI).sin() + (x * 5.0).cos() * 0.3;
        data.push_str(&format!("{:.6},{:.6}\n", x, y));
    }
    data
}

fn time_operation<F, R>(name: &str, operation: F) -> Result<(R, Duration)>
where
    F: FnOnce() -> Result<R>,
{
    let start = Instant::now();
    let result = operation()?;
    let duration = start.elapsed();
    println!("{}: {:.2?}", name, duration);
    Ok((result, duration))
}

fn main() -> Result<()> {
    println!("FastPlot CLI - Performance Examples");
    println!("===================================\n");

    // Test 1: Parsing Performance
    println!("Test 1: Data Parsing Performance");
    println!("---------------------------------");
    
    let sizes = [1_000, 10_000, 100_000];
    for size in sizes {
        println!("\nDataset size: {} points", size);
        let data = generate_large_dataset(size);
        
        let parser = FastParser::new(',', true);
        let (df, parse_time) = time_operation("  Parse CSV", || {
            parser.parse_string(&data)
        })?;
        
        println!("  Parsed {} columns, {} rows", df.num_columns(), df.num_rows());
        println!("  Parse rate: {:.0} points/sec", size as f64 / parse_time.as_secs_f64());
    }

    // Test 2: Rendering Performance
    println!("\n\nTest 2: Plot Rendering Performance");
    println!("----------------------------------");
    
    for size in [1_000, 10_000, 50_000] {
        println!("\nDataset size: {} points", size);
        let data = generate_large_dataset(size);
        let parser = FastParser::new(',', true);
        let df = parser.parse_string(&data)?;
        
        let config = PlotConfig {
            width: 120,
            height: 30,
            title: Some(format!("Performance Test - {} points", size)),
            ..Default::default()
        };
        
        let plot = LinePlot;
        let (result, render_time) = time_operation("  Render plot", || {
            plot.render(&df, &config)
        })?;
        
        println!("  Output size: {} characters", result.len());
        println!("  Render rate: {:.0} points/sec", size as f64 / render_time.as_secs_f64());
    }

    // Test 3: Canvas Performance
    println!("\n\nTest 3: Canvas Operations Performance");
    println!("-------------------------------------");
    
    for (width, height) in [(80, 20), (160, 40), (240, 60)] {
        println!("\nCanvas size: {}x{}", width, height);
        
        let (mut canvas, create_time) = time_operation("  Create canvas", || {
            Ok(Canvas::new(width, height))
        })?;
        
        canvas.set_ranges((0.0, 1000.0), (-2.0, 2.0));
        
        let points = 10_000;
        let ((), plot_time) = time_operation("  Plot points", || {
            for i in 0..points {
                let x = i as f64;
                let y = (x / 100.0).sin();
                canvas.plot_point(x, y, '•');
            }
            Ok(())
        })?;
        
        let (output, render_time) = time_operation("  Render canvas", || {
            Ok(canvas.render())
        })?;
        
        println!("  Points plotted: {}", points);
        println!("  Plot rate: {:.0} points/sec", points as f64 / plot_time.as_secs_f64());
        println!("  Output size: {} chars", output.len());
    }

    // Test 4: Memory Usage Simulation
    println!("\n\nTest 4: Memory Usage Profile");
    println!("-----------------------------");
    
    for size in [10_000, 100_000, 500_000] {
        println!("\nProcessing {} points:", size);
        
        // Simulate memory-intensive operations
        let data = generate_large_dataset(size);
        let data_size = data.len();
        println!("  Input data size: {:.2} MB", data_size as f64 / 1_024_000.0);
        
        let parser = FastParser::new(',', true);
        let df = parser.parse_string(&data)?;
        
        // Estimate memory usage
        let estimated_memory = df.num_columns() * df.num_rows() * 8; // 8 bytes per f64
        println!("  DataFrame memory: {:.2} MB", estimated_memory as f64 / 1_024_000.0);
        
        // Multiple renders to test memory stability
        let config = PlotConfig {
            width: 100,
            height: 25,
            ..Default::default()
        };
        
        let plot = LinePlot;
        let start = Instant::now();
        
        for i in 0..5 {
            let result = plot.render(&df, &config)?;
            if i == 0 {
                println!("  First render size: {} chars", result.len());
            }
        }
        
        let avg_time = start.elapsed() / 5;
        println!("  Average render time: {:.2?}", avg_time);
        println!("  Memory efficiency: {:.1} points/MB", 
            size as f64 / (estimated_memory as f64 / 1_024_000.0));
    }

    // Test 5: Comparison Baseline
    println!("\n\nTest 5: Performance Summary");
    println!("---------------------------");
    
    let test_size = 50_000;
    let data = generate_large_dataset(test_size);
    
    // Full pipeline timing
    let start = Instant::now();
    
    let parser = FastParser::new(',', true);
    let df = parser.parse_string(&data)?;
    let parse_end = Instant::now();
    
    let config = PlotConfig {
        width: 100,
        height: 30,
        title: Some("Performance Baseline".to_string()),
        ..Default::default()
    };
    
    let plot = LinePlot;
    let result = plot.render(&df, &config)?;
    let render_end = Instant::now();
    
    let total_time = render_end.duration_since(start);
    let parse_time = parse_end.duration_since(start);
    let render_time = render_end.duration_since(parse_end);
    
    println!("Dataset: {} points", test_size);
    println!("Parse time: {:.2?} ({:.1}%)", parse_time, 
        parse_time.as_secs_f64() / total_time.as_secs_f64() * 100.0);
    println!("Render time: {:.2?} ({:.1}%)", render_time,
        render_time.as_secs_f64() / total_time.as_secs_f64() * 100.0);
    println!("Total time: {:.2?}", total_time);
    println!("Overall rate: {:.0} points/sec", test_size as f64 / total_time.as_secs_f64());
    println!("Output size: {} characters", result.len());

    println!("\nPerformance testing completed!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_examples() {
        // Test with smaller datasets to avoid long test times
        let data = generate_large_dataset(100);
        assert!(data.len() > 0);
        
        let parser = FastParser::new(',', true);
        let df = parser.parse_string(&data).unwrap();
        assert_eq!(df.num_rows(), 100);
        assert_eq!(df.num_columns(), 2);
    }

    #[test]
    fn test_timing_function() {
        let (result, duration) = time_operation("test", || Ok(42)).unwrap();
        assert_eq!(result, 42);
        assert!(duration.as_nanos() > 0);
    }
}