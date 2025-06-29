use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use fastplot_cli::plot::{Canvas, Plot, LinePlot, BarPlot, ScatterPlot};
use fastplot_cli::data::{DataFrame, Series, PlotConfig};

fn generate_test_dataframe(size: usize) -> DataFrame {
    let mut df = DataFrame::new();
    
    let x_data: Vec<f64> = (0..size).map(|i| i as f64).collect();
    let y_data: Vec<f64> = (0..size).map(|i| (i as f64).sin()).collect();
    
    df.add_column(Series::new("x".to_string(), x_data));
    df.add_column(Series::new("y".to_string(), y_data));
    
    df
}

fn generate_multi_series_dataframe(size: usize, series_count: usize) -> DataFrame {
    let mut df = DataFrame::new();
    
    let x_data: Vec<f64> = (0..size).map(|i| i as f64).collect();
    df.add_column(Series::new("x".to_string(), x_data));
    
    for series_idx in 0..series_count {
        let y_data: Vec<f64> = (0..size)
            .map(|i| ((i as f64) * (series_idx + 1) as f64 / 10.0).sin())
            .collect();
        df.add_column(Series::new(format!("series_{}", series_idx), y_data));
    }
    
    df
}

fn bench_canvas_creation(c: &mut Criterion) {
    let sizes = [(80, 20), (120, 30), (160, 40)];
    let mut group = c.benchmark_group("canvas_creation");
    
    for (width, height) in sizes {
        group.bench_with_input(
            BenchmarkId::new("canvas", format!("{}x{}", width, height)),
            &(width, height),
            |b, &(w, h)| {
                b.iter(|| Canvas::new(black_box(w), black_box(h)))
            },
        );
    }
    
    group.finish();
}

fn bench_canvas_plotting(c: &mut Criterion) {
    let mut group = c.benchmark_group("canvas_plotting");
    let mut canvas = Canvas::new(80, 20);
    canvas.set_ranges((0.0, 100.0), (-1.0, 1.0));
    
    let point_counts = [100, 1000, 10000];
    
    for count in point_counts {
        group.bench_with_input(
            BenchmarkId::new("plot_points", count),
            &count,
            |b, &count| {
                b.iter(|| {
                    let mut canvas = Canvas::new(80, 20);
                    canvas.set_ranges((0.0, count as f64), (-1.0, 1.0));
                    for i in 0..count {
                        let x = i as f64;
                        let y = (x / 10.0).sin();
                        canvas.plot_point(black_box(x), black_box(y), black_box('*'));
                    }
                })
            },
        );
    }
    
    group.finish();
}

fn bench_canvas_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("canvas_rendering");
    
    let sizes = [(80, 20), (120, 30), (200, 50)];
    
    for (width, height) in sizes {
        let mut canvas = Canvas::new(width, height);
        canvas.set_ranges((0.0, 100.0), (-1.0, 1.0));
        
        // Fill canvas with some data
        for i in 0..1000 {
            let x = i as f64 / 10.0;
            let y = x.sin();
            canvas.plot_point(x, y, '*');
        }
        canvas.draw_axis();
        
        group.bench_with_input(
            BenchmarkId::new("render", format!("{}x{}", width, height)),
            &canvas,
            |b, canvas| {
                b.iter(|| canvas.render())
            },
        );
    }
    
    group.finish();
}

fn bench_plot_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("plot_types");
    let data_sizes = [100, 1000, 5000];
    let config = PlotConfig::default();
    
    for size in data_sizes {
        let df = generate_test_dataframe(size);
        
        // Line plot
        group.bench_with_input(
            BenchmarkId::new("line_plot", size),
            &df,
            |b, df| {
                let plot = LinePlot;
                b.iter(|| plot.render(black_box(df), black_box(&config)).unwrap())
            },
        );
        
        // Bar plot
        group.bench_with_input(
            BenchmarkId::new("bar_plot", size),
            &df,
            |b, df| {
                let plot = BarPlot;
                b.iter(|| plot.render(black_box(df), black_box(&config)).unwrap())
            },
        );
        
        // Scatter plot
        group.bench_with_input(
            BenchmarkId::new("scatter_plot", size),
            &df,
            |b, df| {
                let plot = ScatterPlot;
                b.iter(|| plot.render(black_box(df), black_box(&config)).unwrap())
            },
        );
    }
    
    group.finish();
}

fn bench_multi_series_plots(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_series");
    let config = PlotConfig::default();
    
    let series_counts = [1, 5, 10];
    let data_size = 1000;
    
    for series_count in series_counts {
        let df = generate_multi_series_dataframe(data_size, series_count);
        
        group.bench_with_input(
            BenchmarkId::new("multi_line", series_count),
            &df,
            |b, df| {
                let plot = LinePlot;
                b.iter(|| plot.render(black_box(df), black_box(&config)).unwrap())
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_canvas_creation,
    bench_canvas_plotting,
    bench_canvas_rendering,
    bench_plot_types,
    bench_multi_series_plots
);
criterion_main!(benches);