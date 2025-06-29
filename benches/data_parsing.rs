use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use fastplot_cli::data::{FastParser, DataFormat};
use std::hint::black_box as hint_black_box;

fn generate_csv_data(rows: usize, cols: usize) -> String {
    let mut data = String::new();
    
    // Add header
    for col in 0..cols {
        if col > 0 {
            data.push(',');
        }
        data.push_str(&format!("col_{}", col));
    }
    data.push('\n');
    
    // Add data rows
    for row in 0..rows {
        for col in 0..cols {
            if col > 0 {
                data.push(',');
            }
            data.push_str(&format!("{}.{}", row, col));
        }
        data.push('\n');
    }
    
    data
}

fn generate_tsv_data(rows: usize, cols: usize) -> String {
    generate_csv_data(rows, cols).replace(',', "\t")
}

fn bench_csv_parsing(c: &mut Criterion) {
    let sizes = [100, 1000, 10000];
    let mut group = c.benchmark_group("csv_parsing");
    
    for size in sizes {
        let data = generate_csv_data(size, 2);
        
        group.bench_with_input(
            BenchmarkId::new("csv_small", size),
            &data,
            |b, data| {
                let parser = FastParser::new(',', true);
                b.iter(|| parser.parse_string(black_box(data)).unwrap())
            },
        );
    }
    
    group.finish();
}

fn bench_tsv_parsing(c: &mut Criterion) {
    let sizes = [100, 1000, 10000];
    let mut group = c.benchmark_group("tsv_parsing");
    
    for size in sizes {
        let data = generate_tsv_data(size, 2);
        
        group.bench_with_input(
            BenchmarkId::new("tsv_small", size),
            &data,
            |b, data| {
                let parser = FastParser::new('\t', true);
                b.iter(|| parser.parse_string(black_box(data)).unwrap())
            },
        );
    }
    
    group.finish();
}

fn bench_large_datasets(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_datasets");
    group.sample_size(10); // Fewer samples for large datasets
    
    // Wide dataset (many columns)
    let wide_data = generate_csv_data(1000, 50);
    group.bench_function("wide_dataset_1000x50", |b| {
        let parser = FastParser::new(',', true);
        b.iter(|| parser.parse_string(black_box(&wide_data)).unwrap())
    });
    
    // Long dataset (many rows)
    let long_data = generate_csv_data(100000, 2);
    group.bench_function("long_dataset_100000x2", |b| {
        let parser = FastParser::new(',', true);
        b.iter(|| parser.parse_string(black_box(&long_data)).unwrap())
    });
    
    group.finish();
}

fn bench_format_conversion(c: &mut Criterion) {
    let data = generate_csv_data(1000, 2);
    let mut group = c.benchmark_group("format_conversion");
    
    group.bench_function("xy_format", |b| {
        let parser = FastParser::new(',', true);
        b.iter(|| {
            parser.parse_with_format(
                hint_black_box(data.as_bytes()), 
                black_box(DataFormat::XY)
            ).unwrap()
        })
    });
    
    group.bench_function("yx_format", |b| {
        let parser = FastParser::new(',', true);
        b.iter(|| {
            parser.parse_with_format(
                hint_black_box(data.as_bytes()), 
                black_box(DataFormat::YX)
            ).unwrap()
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_csv_parsing,
    bench_tsv_parsing,
    bench_large_datasets,
    bench_format_conversion
);
criterion_main!(benches);