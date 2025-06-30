mod cli;
mod data;
mod plot;
mod config;
mod testing;

use anyhow::{Result, anyhow};
use clap::Parser;
use std::io::{self, Read, BufReader};
use std::fs::File;

use data::PlotConfig;
use plot::{BarChart, LinePlot, ScatterPlot, Histogram, DensityPlot, BoxPlot, KernelType};

fn main() -> Result<()> {
    let args = cli::Args::parse();
    
    match &args.command {
        cli::Commands::Line { options } |
        cli::Commands::Lines { options } |
        cli::Commands::Bar { options } |
        cli::Commands::Scatter { options } |
        cli::Commands::Hist { options } |
        cli::Commands::Density { options } |
        cli::Commands::Boxplot { options } |
        cli::Commands::Count { options } => {
            // Read input data
            let input_data = read_input_data(options)?;
            
            // Parse data based on delimiter and format
            let dataframe = parse_input_data(&input_data, options)?;
            
            // Create plot configuration
            let config = create_plot_config(options);
            
            // Generate plot based on command
            let plot_output = generate_plot(&args.command, &dataframe, &config)?;
            
            // Output the plot
            println!("{}", plot_output);
        }
        cli::Commands::Benchmark { size: _, plot_type: _ } => {
            return Err(anyhow!("Benchmark command not yet implemented"));
        }
        cli::Commands::Generate { dataset: _, size: _ } => {
            return Err(anyhow!("Generate command not yet implemented"));
        }
    }
    
    Ok(())
}

fn read_input_data(options: &cli::PlotOptions) -> Result<String> {
    match &options.input {
        Some(path) => {
            let file = File::open(path)?;
            let mut reader = BufReader::new(file);
            let mut content = String::new();
            reader.read_to_string(&mut content)?;
            Ok(content)
        }
        None => {
            let mut content = String::new();
            io::stdin().read_to_string(&mut content)?;
            Ok(content)
        }
    }
}

fn parse_input_data(data: &str, options: &cli::PlotOptions) -> Result<data::DataFrame> {
    let parser = data::parser::FastParser::new(options.delimiter, options.has_header);
    parser.parse_string_with_auto_detect(data)
}

fn create_plot_config(options: &cli::PlotOptions) -> PlotConfig {
    PlotConfig {
        width: options.width,
        height: options.height,
        title: options.title.clone(),
        xlabel: options.xlabel.clone(),
        ylabel: options.ylabel.clone(),
        delimiter: options.delimiter,
        has_header: options.has_header,
        format: options.format.clone(),
        xlim: options.xlim,
        ylim: options.ylim,
        color: options.color.clone(),
        symbol: options.symbol,
    }
}

fn generate_plot(command: &cli::Commands, dataframe: &data::DataFrame, config: &PlotConfig) -> Result<String> {
    match command {
        cli::Commands::Line { .. } => {
            let chart = LinePlot::single();
            chart.render(dataframe, config)
        }
        cli::Commands::Lines { .. } => {
            let chart = LinePlot::multi();
            chart.render(dataframe, config)
        }
        cli::Commands::Bar { .. } => {
            let chart = BarChart::vertical();
            chart.render(dataframe, config)
        }
        cli::Commands::Scatter { .. } => {
            let chart = ScatterPlot::default();
            chart.render(dataframe, config)
        }
        cli::Commands::Hist { .. } => {
            let chart = Histogram::auto_bins();
            chart.render(dataframe, config)
        }
        cli::Commands::Density { .. } => {
            let chart = DensityPlot::auto_bandwidth()
                .with_kernel(KernelType::Gaussian)
                .with_resolution(200);
            chart.render(dataframe, config)
        }
        cli::Commands::Boxplot { .. } => {
            let chart = BoxPlot::vertical();
            chart.render(dataframe, config)
        }
        cli::Commands::Count { .. } => {
            // For count plots, use histogram
            let chart = Histogram::auto_bins();
            chart.render(dataframe, config)
        }
        cli::Commands::Benchmark { size: _, plot_type: _ } => {
            Err(anyhow!("Benchmark command not yet implemented"))
        }
        cli::Commands::Generate { dataset: _, size: _ } => {
            Err(anyhow!("Generate command not yet implemented"))
        }
    }
}