use clap::{Parser, Subcommand};
use crate::data::DataFormat;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(name = "fastplot")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug, Clone)]
pub struct PlotOptions {
    #[arg(short = 'd', long = "delimiter", default_value = "\t", help = "Field delimiter")]
    pub delimiter: char,

    #[arg(short = 'H', long = "header", help = "First line contains headers")]
    pub has_header: bool,

    #[arg(short = 'o', long = "output", help = "Output file (default: stderr)")]
    pub output: Option<PathBuf>,

    #[arg(short = 'O', long = "pass-data", help = "Pass input data to stdout")]
    pub pass_data: bool,

    #[arg(short = 't', long = "title", help = "Plot title")]
    pub title: Option<String>,

    #[arg(short = 'w', long = "width", default_value = "80", help = "Plot width in characters")]
    pub width: usize,

    #[arg(long = "height", default_value = "20", help = "Plot height in characters")]
    pub height: usize,

    #[arg(short = 'c', long = "color", help = "Plot color")]
    pub color: Option<String>,

    #[arg(long = "symbol", help = "Plot symbol character")]
    pub symbol: Option<char>,

    #[arg(long = "fmt", default_value = "xy", help = "Data format (xy, xyy, xyxy, yx)")]
    pub format: DataFormat,

    #[arg(long = "xlim", help = "X-axis limits (min,max)", value_parser = parse_limits)]
    pub xlim: Option<(f64, f64)>,

    #[arg(long = "ylim", help = "Y-axis limits (min,max)", value_parser = parse_limits)]
    pub ylim: Option<(f64, f64)>,

    #[arg(long = "xlabel", help = "X-axis label")]
    pub xlabel: Option<String>,

    #[arg(long = "ylabel", help = "Y-axis label")]
    pub ylabel: Option<String>,

    #[arg(long = "progress", help = "Progressive/streaming mode")]
    pub progress: bool,

    #[arg(long = "benchmark", help = "Show performance metrics")]
    pub benchmark: bool,

    #[arg(long = "profile", help = "Enable memory profiling")]
    pub profile: bool,

    #[arg(help = "Input data file (default: stdin)")]
    pub input: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Horizontal bar chart")]
    Bar {
        #[command(flatten)]
        options: PlotOptions,
    },
    #[command(about = "Histogram")]
    Hist {
        #[command(flatten)]
        options: PlotOptions,
    },
    #[command(about = "Line chart")]
    Line {
        #[command(flatten)]
        options: PlotOptions,
    },
    #[command(about = "Multi-series line chart")]
    Lines {
        #[command(flatten)]
        options: PlotOptions,
    },
    #[command(about = "Scatter plot")]
    Scatter {
        #[command(flatten)]
        options: PlotOptions,
    },
    #[command(about = "Density plot")]
    Density {
        #[command(flatten)]
        options: PlotOptions,
    },
    #[command(about = "Box plot")]
    Boxplot {
        #[command(flatten)]
        options: PlotOptions,
    },
    #[command(about = "Count-based bar chart")]
    Count {
        #[command(flatten)]
        options: PlotOptions,
    },
    #[command(about = "Performance testing")]
    Benchmark {
        #[arg(long = "size", default_value = "1000", help = "Dataset size")]
        size: usize,
        #[arg(long = "plot-type", default_value = "line", help = "Plot type to benchmark")]
        plot_type: String,
    },
    #[command(about = "Generate test data")]
    Generate {
        #[arg(long = "dataset", default_value = "sine", help = "Dataset type")]
        dataset: String,
        #[arg(long = "size", default_value = "100", help = "Dataset size")]
        size: usize,
    },
}

fn parse_limits(s: &str) -> Result<(f64, f64), String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err("Limits must be in format 'min,max'".to_string());
    }
    
    let min: f64 = parts[0].parse().map_err(|_| "Invalid minimum value")?;
    let max: f64 = parts[1].parse().map_err(|_| "Invalid maximum value")?;
    
    if min >= max {
        return Err("Minimum must be less than maximum".to_string());
    }
    
    Ok((min, max))
}