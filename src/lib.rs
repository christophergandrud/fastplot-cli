pub mod cli;
pub mod data;
pub mod plot;
pub mod config;
pub mod testing;
pub mod performance;

// Re-export commonly used items
pub use data::{FastParser, DataFrame, Series, PlotConfig, DataFormat, parse_csv_data, parse_file_data};
pub use plot::{Plot, LinePlot, BarPlot, ScatterPlot, Histogram, Canvas};
pub use performance::{PerformanceMonitor, MemoryProfiler, BenchmarkRunner, time_function};