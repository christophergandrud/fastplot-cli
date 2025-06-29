# FastPlot-CLI: Rust Implementation Plan
*A high-performance terminal plotting tool - YouPlot reimplemented in Rust*

## Project Overview

**Crate Name:** `fastplot-cli`  
**Binary Name:** `fastplot` (with `fp` alias)  
**Purpose:** Reimplementation of YouPlot in Rust with significant performance improvements  
**Target:** Drop-in replacement for YouPlot with identical CLI interface but 10-100x faster performance

## Key Value Propositions

- **Performance**: 10-100x faster than YouPlot for large datasets
- **Memory Efficiency**: Lower memory footprint through Rust's zero-cost abstractions
- **Single Binary**: No runtime dependencies (unlike Ruby gems)
- **Compatibility**: Identical CLI interface to YouPlot for easy migration
- **Streaming**: Better real-time/progressive plotting capabilities

---

## High-Level Architecture

### Core Components

1. **CLI Interface** - Using `clap` for argument parsing
2. **Data Input Layer** - Handle stdin/file input with various delimiters  
3. **Data Processing Engine** - Parse and transform data into plottable formats
4. **Unicode Plot Renderer** - Draw plots using Unicode characters
5. **Output Manager** - Handle stdout/stderr output options
6. **Performance Monitoring** - Built-in benchmarking and profiling

---

## Implementation Phases

### Phase 0: Foundation & Testing Infrastructure (Week 1)

#### Project Structure
```
fastplot-cli/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs              # CLI entry point
│   ├── cli.rs              # Argument parsing with clap
│   ├── data/
│   │   ├── mod.rs          # Data processing module
│   │   ├── parser.rs       # CSV/TSV/delimited data parsing
│   │   └── types.rs        # Data structures (Series, DataFrame)
│   ├── plot/
│   │   ├── mod.rs          # Plot module
│   │   ├── canvas.rs       # Unicode canvas for drawing
│   │   ├── bar.rs          # Bar plot implementation
│   │   ├── line.rs         # Line plot implementation
│   │   ├── scatter.rs      # Scatter plot implementation
│   │   ├── histogram.rs    # Histogram implementation
│   │   ├── density.rs      # Density plot implementation
│   │   └── boxplot.rs      # Box plot implementation
│   ├── testing/
│   │   ├── mod.rs          # Test data module
│   │   ├── generators.rs   # Data generation functions
│   │   ├── datasets.rs     # Predefined test datasets
│   │   └── benchmarks.rs   # Performance test data
│   ├── config.rs           # Configuration management
│   └── performance.rs      # Performance monitoring utilities
├── examples/
│   ├── basic_plots.rs      # Example usage
│   └── performance.rs      # Performance demos
├── benches/
│   ├── data_parsing.rs     # Parsing benchmarks
│   ├── plot_rendering.rs   # Rendering benchmarks
│   └── end_to_end.rs       # Full pipeline benchmarks
└── tests/
    ├── integration.rs      # Integration tests
    ├── compatibility.rs    # YouPlot compatibility tests
    └── performance.rs      # Performance regression tests
```

#### Key Dependencies
```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
csv = "1.3"
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
crossterm = "0.27"           # Terminal manipulation
anyhow = "1.0"               # Error handling
tokio = { version = "1.0", features = ["full"] }  # Async stdin
unicode-width = "0.1"        # Unicode character width handling
rand = "0.8"                 # Test data generation
chrono = { version = "0.4", features = ["serde"] }  # Date/time handling

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }  # Benchmarking
tempfile = "3.0"             # Temporary files for testing
assert_cmd = "2.0"           # CLI testing
predicates = "3.0"           # Test assertions
```

### Phase 1: Core Data Structures & Parsing (Week 2)

#### Data Types
```rust
// Basic data structures
#[derive(Debug, Clone)]
pub struct Series {
    pub name: String,
    pub data: Vec<f64>,
}

#[derive(Debug)]
pub struct DataFrame {
    pub columns: Vec<Series>,
    pub headers: Option<Vec<String>>,
}

// Plot configuration
#[derive(Debug, Clone)]
pub struct PlotConfig {
    pub width: usize,
    pub height: usize,
    pub title: Option<String>,
    pub xlabel: Option<String>,
    pub ylabel: Option<String>,
    pub delimiter: char,
    pub has_header: bool,
    pub format: DataFormat,
    pub xlim: Option<(f64, f64)>,
    pub ylim: Option<(f64, f64)>,
    pub color: Option<String>,
    pub symbol: Option<char>,
}

#[derive(Debug, Clone)]
pub enum DataFormat {
    XY,    // First col X, second col Y
    XYY,   // First col X, remaining cols are Y series
    XYXY,  // Alternating X,Y pairs
    YX,    // First col Y, second col X (swapped)
}
```

#### High-Performance CSV Parser
```rust
pub struct FastParser {
    delimiter: u8,
    buffer_size: usize,
}

impl FastParser {
    pub fn parse_stream<R: Read>(&self, reader: R) -> Result<DataFrame> {
        // Optimized streaming parser with minimal allocations
        // Use SIMD where possible for delimiter detection
        // Pre-allocate vectors based on estimated row count
    }
}
```

### Phase 2: Unicode Canvas & Rendering Engine (Week 3)

#### Canvas System
```rust
pub struct Canvas {
    width: usize,
    height: usize,
    buffer: Vec<Vec<char>>,
    x_range: (f64, f64),
    y_range: (f64, f64),
}

impl Canvas {
    pub fn plot_point(&mut self, x: f64, y: f64, symbol: char) {
        // Convert data coordinates to canvas coordinates
        // Handle Unicode width properly
        // Optimize for sparse data
    }
    
    pub fn draw_axis(&mut self) {
        // Draw X and Y axes with proper scaling
        // Smart tick mark placement
    }
    
    pub fn render(&self) -> String {
        // Convert buffer to string for output
        // Minimize string allocations
    }
}
```

### Phase 3: Plot Types Implementation (Week 4)

#### Core Plot Types
- **Bar Chart**: Horizontal and vertical bars
- **Line Plot**: Single and multi-series lines  
- **Scatter Plot**: Point plotting with different symbols
- **Histogram**: Binned data visualization
- **Density Plot**: Kernel density estimation
- **Box Plot**: Statistical distribution visualization

### Phase 4: Advanced Features (Week 5)

#### Progressive Mode
```rust
// For streaming data like YouPlot's --progress flag
pub async fn process_streaming_data<R: AsyncRead>(
    reader: R,
    plot_type: PlotType,
    config: PlotConfig,
) -> Result<()> {
    // Handle real-time data plotting
    // Update plot as new data arrives
    // Optimize for minimal screen flicker
}
```

#### Configuration System
- YAML configuration files
- Environment variable support
- Command-line option precedence

### Phase 5: Performance Optimization (Week 6)

#### Key Optimizations
- SIMD operations for data processing
- Memory pool allocation for frequent operations
- Lazy evaluation for large datasets
- Parallel processing for multi-series data
- Cache-friendly data structures

### Phase 6: Testing & Compatibility (Week 7)

#### Compatibility Testing
- Identical output verification against YouPlot
- Edge case handling
- Unicode rendering consistency
- Color support matching

### Phase 7: Documentation & Polish (Week 8)

#### Final Polish
- Comprehensive documentation
- Performance benchmarking reports
- Migration guide from YouPlot
- Package publishing preparation

---

## Test Data Infrastructure

### Simulated Data Generators

#### Mathematical Functions
```rust
pub mod generators {
    // Linear data with configurable noise
    pub fn linear_data(n: usize, slope: f64, intercept: f64, noise: f64) -> Vec<(f64, f64)>
    
    // Sine wave with amplitude, frequency, phase
    pub fn sine_wave(n: usize, amplitude: f64, frequency: f64, phase: f64) -> Vec<(f64, f64)>
    
    // Exponential growth/decay
    pub fn exponential_data(n: usize, base: f64, rate: f64, noise: f64) -> Vec<(f64, f64)>
    
    // Random walk
    pub fn random_walk(n: usize, step_size: f64) -> Vec<(f64, f64)>
    
    // Normal distribution samples
    pub fn normal_distribution(n: usize, mean: f64, std_dev: f64) -> Vec<f64>
}
```

#### Real-World Simulations
```rust
pub mod datasets {
    // Stock price simulation
    pub fn stock_prices(days: usize, initial_price: f64, volatility: f64) -> Vec<(DateTime<Utc>, f64)>
    
    // Server metrics simulation  
    pub fn server_metrics(hours: usize) -> Vec<(DateTime<Utc>, f64, f64, f64)>
    
    // Sales data simulation
    pub fn sales_data(months: usize) -> Vec<(String, f64, f64)>
    
    // Iris-like classification dataset
    pub fn iris_like_data(samples_per_class: usize) -> Vec<(f64, f64, f64, f64, String)>
}
```

#### Performance Test Data
```rust
pub mod benchmarks {
    // Large dataset generators for performance testing
    pub fn large_timeseries(size: usize) -> Vec<(f64, f64)>
    
    // Memory stress test - wide datasets
    pub fn wide_dataset(rows: usize, cols: usize) -> Vec<Vec<f64>>
    
    // Streaming data simulation
    pub fn streaming_generator(rate_hz: f64) -> impl Iterator<Item = (f64, f64)>
}
```

#### Edge Case Testing
```rust
pub mod edge_cases {
    // Empty dataset, single point, NaN/Infinity values
    // Very large numbers, duplicate x values
    // Extreme aspect ratios, Unicode edge cases
}
```

---

## Performance Benchmarking Strategy

### Benchmark Categories

#### 1. Data Parsing Performance
```rust
// Benchmark different data sizes and formats
- Small datasets (100 rows): CSV, TSV, space-delimited
- Medium datasets (10,000 rows): Multiple columns, headers
- Large datasets (1,000,000 rows): Memory usage, streaming
- Wide datasets (1000 columns): Parse time, memory efficiency
```

#### 2. Plot Rendering Performance  
```rust
// Benchmark rendering speed for different plot types
- Bar charts: Various bar counts (10, 100, 1000, 10000)
- Line plots: Point density, multi-series (1, 5, 10, 50 series)
- Scatter plots: Point count scaling (100 to 1M points)
- Histograms: Bin count optimization (10 to 1000 bins)
```

#### 3. End-to-End Performance
```rust
// Full pipeline benchmarks
- stdin processing speed
- File I/O performance
- Memory usage profiling
- CPU utilization analysis
```

### Comparison Targets

#### Primary Comparison: YouPlot
```bash
# Benchmark identical datasets against YouPlot
fastplot generate --dataset timeseries --size 10000 > test.csv

# Time YouPlot
time uplot line test.csv -d, -H -t "Test Data"

# Time FastPlot  
time fastplot line test.csv -d, -H -t "Test Data"

# Memory usage comparison
/usr/bin/time -v uplot line test.csv -d, -H
/usr/bin/time -v fastplot line test.csv -d, -H
```

#### Secondary Comparisons
- **gnuplot**: Traditional plotting tool
- **plotutils**: GNU plotting utilities  
- **Python matplotlib**: Via command-line scripts
- **R plotting**: Via Rscript commands

### Performance Targets

#### Speed Targets
- **Small datasets (< 1K points)**: 2-5x faster than YouPlot
- **Medium datasets (1K-100K points)**: 10-20x faster than YouPlot
- **Large datasets (> 100K points)**: 50-100x faster than YouPlot
- **Streaming data**: Sub-100ms update latency

#### Memory Targets
- **50% less memory usage** compared to YouPlot for equivalent datasets
- **Constant memory usage** for streaming operations
- **Linear scaling** with data size (no quadratic growth)

#### Startup Time Targets
- **< 10ms cold start** time
- **< 1ms warm start** time
- **No Ruby interpreter overhead**

### Benchmarking Infrastructure

#### Automated Benchmarking
```rust
// Criterion.rs benchmark suite
pub fn bench_data_parsing(c: &mut Criterion) {
    let sizes = [100, 1000, 10000, 100000];
    for size in sizes {
        let data = generate_test_data(size);
        c.bench_function(&format!("parse_{}_rows", size), |b| {
            b.iter(|| parse_csv_data(black_box(&data)))
        });
    }
}

// Memory usage benchmarking
pub fn profile_memory_usage() {
    let sizes = [1000, 10000, 100000, 1000000];
    for size in sizes {
        let start_memory = get_memory_usage();
        let _plot = render_line_plot(generate_timeseries(size));
        let end_memory = get_memory_usage();
        println!("Size: {}, Memory: {} MB", size, (end_memory - start_memory) / 1024 / 1024);
    }
}
```

#### Continuous Performance Testing
```yaml
# GitHub Actions performance CI
name: Performance Benchmarks
on: [push, pull_request]
jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - name: Benchmark vs YouPlot
        run: |
          # Install YouPlot
          gem install youplot
          
          # Run comparative benchmarks
          ./scripts/benchmark_comparison.sh
          
      - name: Performance Regression Check
        run: |
          cargo bench -- --output-format json | tee bench_results.json
          python scripts/check_performance_regression.py
```

### Benchmark Reporting

#### Performance Dashboard
- Real-time performance metrics
- Historical performance trends
- Comparison charts vs YouPlot
- Memory usage visualization
- Platform-specific performance data

#### Benchmark Reports
```markdown
## Performance Comparison Report

### Dataset: 100,000 point time series
- **YouPlot**: 2.3 seconds, 45 MB memory
- **FastPlot**: 0.12 seconds, 8 MB memory  
- **Improvement**: 19x faster, 5.6x less memory

### Dataset: 1,000,000 point scatter plot
- **YouPlot**: 28.5 seconds, 180 MB memory
- **FastPlot**: 0.31 seconds, 12 MB memory
- **Improvement**: 92x faster, 15x less memory
```

---

## CLI Interface Design

### Command Structure (YouPlot Compatible)
```bash
fastplot <command> [options] [data.tsv]

# Commands (identical to YouPlot)
fastplot bar          # Horizontal bar chart
fastplot hist         # Histogram  
fastplot line         # Line chart
fastplot lines        # Multi-series line chart
fastplot scatter      # Scatter plot
fastplot density      # Density plot
fastplot boxplot      # Box plot
fastplot count        # Count-based bar chart

# Aliases
fp <command>          # Short alias
```

### Options (YouPlot Compatible)
```bash
# Input/Output
-d, --delimiter CHAR  # Field delimiter (default: tab)
-H, --header         # First line contains headers
-o, --output FILE    # Output file (default: stderr)
-O, --pass-data      # Pass input data to stdout

# Plot appearance  
-t, --title TITLE    # Plot title
-w, --width WIDTH    # Plot width in characters
-h, --height HEIGHT  # Plot height in characters
-c, --color COLOR    # Plot color
--symbol CHAR        # Plot symbol character

# Data format
--fmt FORMAT         # Data format (xy, xyy, xyxy, yx)
--xlim MIN,MAX       # X-axis limits
--ylim MIN,MAX       # Y-axis limits

# Performance extensions
--progress           # Progressive/streaming mode
--benchmark         # Show performance metrics
--profile           # Enable memory profiling
```

### FastPlot-Specific Extensions
```bash
# Performance and debugging
fastplot benchmark --size 10000 --plot-type line    # Performance testing
fastplot generate --dataset sine --size 1000        # Test data generation
fastplot profile data.csv                           # Memory usage analysis
fastplot compare youplot data.csv                   # Direct comparison with YouPlot

# Advanced features
fastplot multi data1.csv data2.csv data3.csv        # Multiple file plotting
fastplot watch data.csv                             # File watching mode
fastplot server --port 8080                         # Web interface (future)
```

---

## Quality Assurance Strategy

### Testing Strategy

#### Unit Tests
- Data parsing accuracy
- Plot rendering correctness  
- Configuration handling
- Error conditions

#### Integration Tests
- End-to-end pipeline testing
- CLI interface validation
- File format compatibility
- Output format verification

#### Compatibility Tests
```rust
#[test]
fn test_youplot_output_compatibility() {
    // Generate identical test data
    let test_data = generate_sine_wave(100);
    
    // Run YouPlot
    let youplot_output = run_youplot(&test_data, "line");
    
    // Run FastPlot
    let fastplot_output = run_fastplot(&test_data, "line");
    
    // Compare outputs (allowing for minor differences)
    assert_plots_equivalent(youplot_output, fastplot_output);
}
```

#### Performance Tests
```rust
#[test]
fn test_performance_targets() {
    let large_dataset = generate_timeseries(100_000);
    
    let start = Instant::now();
    let _plot = render_line_plot(large_dataset);
    let duration = start.elapsed();
    
    // Should be significantly faster than YouPlot baseline
    assert!(duration < Duration::from_millis(500));
}
```

### Error Handling
- Graceful handling of malformed data
- Clear error messages
- Consistent exit codes
- Helpful suggestions for common mistakes

---

## Documentation Strategy

### User Documentation
- **README.md**: Quick start and basic usage
- **CLI Reference**: Complete command documentation  
- **Migration Guide**: Converting from YouPlot
- **Performance Guide**: Optimization tips
- **Examples**: Common use cases and recipes

### Developer Documentation
- **Architecture Overview**: System design
- **API Documentation**: Internal interfaces
- **Contributing Guide**: Development setup
- **Benchmark Guide**: Performance testing
- **Release Process**: Publishing workflow

### Performance Documentation
- **Benchmark Reports**: Regular performance comparisons
- **Optimization Guide**: Performance tuning techniques
- **Memory Usage Analysis**: Memory efficiency documentation
- **Platform Performance**: OS-specific performance characteristics

---

## Deployment & Distribution

### Package Distribution
```toml
# Cargo.toml for release
[package]
name = "fastplot-cli"
version = "1.0.0"
description = "A fast terminal plotting tool - YouPlot reimplemented in Rust"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
repository = "https://github.com/yourusername/fastplot-cli"
documentation = "https://docs.rs/fastplot-cli"
categories = ["command-line-utilities", "visualization", "science"]
keywords = ["plotting", "terminal", "cli", "visualization", "youplot"]

[[bin]]
name = "fastplot"
path = "src/main.rs"

[[bin]]  
name = "fp"
path = "src/main.rs"
```

### Installation Methods
```bash
# Cargo (primary)
cargo install fastplot-cli

# Homebrew (macOS/Linux)
brew install fastplot

# APT (Ubuntu/Debian)
apt install fastplot

# Scoop (Windows)  
scoop install fastplot

# Pre-compiled binaries
wget https://github.com/user/fastplot-cli/releases/download/v1.0.0/fastplot-linux-amd64
```

### Cross-Platform Support
- Linux (x86_64, ARM64)
- macOS (x86_64, ARM64/M1)
- Windows (x86_64)
- FreeBSD, OpenBSD
- Static linking for maximum compatibility

---

## Success Metrics

### Performance Metrics
- **10-100x speed improvement** over YouPlot
- **50%+ memory reduction** compared to YouPlot
- **Sub-second rendering** for datasets up to 1M points
- **<100ms latency** for streaming updates

### Adoption Metrics
- **1000+ downloads** in first month
- **Positive community feedback** on Reddit/HN
- **Documentation completeness**: 100% API coverage
- **Test coverage**: >90% code coverage

### Quality Metrics
- **Zero performance regressions** in CI
- **100% YouPlot compatibility** for core features
- **Cross-platform support** for major OS
- **Memory leak free** operation

---

## Future Roadmap

### Version 1.0: Core Compatibility
- Complete YouPlot feature parity
- Performance optimizations
- Comprehensive testing
- Documentation

### Version 1.1: Enhanced Performance
- SIMD optimizations
- Parallel processing
- Streaming improvements
- Memory optimizations

### Version 1.2: Extended Features
- Additional plot types
- Interactive mode
- Configuration enhancements
- Plugin system

### Version 2.0: Advanced Features
- Web interface
- Real-time dashboards
- Custom plot types
- API integration

---

## Getting Started

### Development Setup
```bash
# Clone repository
git clone https://github.com/yourusername/fastplot-cli
cd fastplot-cli

# Install dependencies
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench

# Generate test data and compare with YouPlot
cargo run -- generate --dataset sine --size 1000 | \
  tee >(uplot line -t "YouPlot") | \
  cargo run -- line -t "FastPlot"
```

### Contributing
1. **Performance First**: All changes must maintain or improve performance
2. **Compatibility**: Maintain YouPlot CLI compatibility
3. **Testing**: Include benchmarks for performance-critical code
4. **Documentation**: Update docs for user-facing changes

---

*This plan represents a comprehensive approach to creating a high-performance, compatible alternative to YouPlot that showcases Rust's strengths in system programming and performance optimization.*