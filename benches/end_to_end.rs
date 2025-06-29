use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use fastplot_cli::data::{FastParser, DataFormat, PlotConfig};
use fastplot_cli::plot::{Plot, LinePlot, BarPlot, ScatterPlot};
use std::io::Cursor;

fn generate_synthetic_data(rows: usize, format: &str) -> String {
    let mut data = String::new();
    
    match format {
        "timeseries" => {
            data.push_str("time,value\n");
            for i in 0..rows {
                data.push_str(&format!("{},{}\n", i, (i as f64 / 10.0).sin()));
            }
        },
        "stock_prices" => {
            data.push_str("date,open,high,low,close,volume\n");
            let mut price = 100.0;
            for i in 0..rows {
                let change = (i as f64 * 0.1).sin() * 2.0;
                price += change;
                let high = price + (i as f64 * 0.05).cos().abs();
                let low = price - (i as f64 * 0.05).sin().abs();
                data.push_str(&format!("2024-01-{:02},{:.2},{:.2},{:.2},{:.2},{}\n", 
                    (i % 30) + 1, price, high, low, price + change * 0.5, (i + 1) * 1000));
            }
        },
        "scatter" => {
            data.push_str("x,y\n");
            for i in 0..rows {
                let x = i as f64 / 100.0;
                let y = x * x + (x * 10.0).sin() * 0.1;
                data.push_str(&format!("{:.3},{:.3}\n", x, y));
            }
        },
        _ => {
            data.push_str("x,y\n");
            for i in 0..rows {
                data.push_str(&format!("{},{}\n", i, i * 2));
            }
        }
    }
    
    data
}

fn bench_full_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_pipeline");
    let data_sizes = [100, 1000, 10000];
    
    for size in data_sizes {
        // Timeseries data
        let timeseries_data = generate_synthetic_data(size, "timeseries");
        group.bench_with_input(
            BenchmarkId::new("timeseries_line", size),
            &timeseries_data,
            |b, data| {
                b.iter(|| {
                    // Parse data
                    let parser = FastParser::new(',', true);
                    let df = parser.parse_string(black_box(data)).unwrap();
                    
                    // Create plot config
                    let config = PlotConfig {
                        width: 80,
                        height: 20,
                        title: Some("Timeseries".to_string()),
                        ..Default::default()
                    };
                    
                    // Render plot
                    let plot = LinePlot;
                    plot.render(&df, &config).unwrap()
                })
            },
        );
        
        // Scatter plot data
        let scatter_data = generate_synthetic_data(size, "scatter");
        group.bench_with_input(
            BenchmarkId::new("scatter_plot", size),
            &scatter_data,
            |b, data| {
                b.iter(|| {
                    let parser = FastParser::new(',', true);
                    let df = parser.parse_string(black_box(data)).unwrap();
                    
                    let config = PlotConfig {
                        width: 80,
                        height: 20,
                        title: Some("Scatter".to_string()),
                        ..Default::default()
                    };
                    
                    let plot = ScatterPlot;
                    plot.render(&df, &config).unwrap()
                })
            },
        );
    }
    
    group.finish();
}

fn bench_different_formats(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_formats");
    let size = 1000;
    
    // CSV format
    let csv_data = generate_synthetic_data(size, "timeseries");
    group.bench_function("csv_format", |b| {
        b.iter(|| {
            let parser = FastParser::new(',', true);
            let df = parser.parse_string(black_box(&csv_data)).unwrap();
            let config = PlotConfig::default();
            let plot = LinePlot;
            plot.render(&df, &config).unwrap()
        })
    });
    
    // TSV format
    let tsv_data = csv_data.replace(',', "\t");
    group.bench_function("tsv_format", |b| {
        b.iter(|| {
            let parser = FastParser::new('\t', true);
            let df = parser.parse_string(black_box(&tsv_data)).unwrap();
            let config = PlotConfig::default();
            let plot = LinePlot;
            plot.render(&df, &config).unwrap()
        })
    });
    
    group.finish();
}

fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    group.sample_size(10); // Fewer samples for memory-intensive tests
    
    let sizes = [1000, 10000, 100000];
    
    for size in sizes {
        let data = generate_synthetic_data(size, "timeseries");
        
        group.bench_with_input(
            BenchmarkId::new("large_dataset", size),
            &data,
            |b, data| {
                b.iter(|| {
                    let parser = FastParser::new(',', true);
                    let df = parser.parse_string(black_box(data)).unwrap();
                    
                    // Force multiple allocations to test memory efficiency
                    let config = PlotConfig {
                        width: 200,
                        height: 50,
                        title: Some(format!("Large Dataset {} points", size)),
                        ..Default::default()
                    };
                    
                    let plot = LinePlot;
                    let result = plot.render(&df, &config).unwrap();
                    
                    // Ensure the result is actually used
                    black_box(result.len())
                })
            },
        );
    }
    
    group.finish();
}

fn bench_streaming_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("streaming_simulation");
    
    // Simulate processing data in chunks
    let chunk_sizes = [100, 500, 1000];
    let total_size = 10000;
    
    for chunk_size in chunk_sizes {
        group.bench_with_input(
            BenchmarkId::new("chunked_processing", chunk_size),
            &chunk_size,
            |b, &chunk_size| {
                b.iter(|| {
                    let mut results = Vec::new();
                    
                    for chunk_start in (0..total_size).step_by(chunk_size) {
                        let chunk_end = (chunk_start + chunk_size).min(total_size);
                        let chunk_data = generate_synthetic_data(chunk_end - chunk_start, "timeseries");
                        
                        let parser = FastParser::new(',', true);
                        let df = parser.parse_string(black_box(&chunk_data)).unwrap();
                        
                        let config = PlotConfig {
                            width: 40,
                            height: 10,
                            ..Default::default()
                        };
                        
                        let plot = LinePlot;
                        let result = plot.render(&df, &config).unwrap();
                        results.push(result);
                    }
                    
                    black_box(results.len())
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_full_pipeline,
    bench_different_formats,
    bench_memory_usage,
    bench_streaming_simulation
);
criterion_main!(benches);