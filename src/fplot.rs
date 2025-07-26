// fplot binary - short alias for fastplot
// This eliminates the duplicate binary warning while maintaining both command names

mod data;
mod coordinates;
mod ticks;
mod layout;
mod scatter;
mod line_style;
mod line_drawing;
mod layered_canvas;
mod line_plot;
mod function;
mod bar_chart;
mod color;
mod plot_config;

use clap::{Parser, Subcommand};
use anyhow::Result;
use plot_config::{PlotConfig, PlotType, PlotCommand};

#[derive(Parser)]
#[command(name = "fplot")]
#[command(about = "A fast terminal plotting tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create scatter plots from data files or functions
    Scatter {
        /// Data source: CSV file path or function expression (e.g., "data.csv" or "function:x^2")
        source: String,
        /// Plot title
        #[arg(short, long, default_value = "Scatter Plot")]
        title: String,
        /// Point character
        #[arg(short = 'p', long, default_value = "●")]
        point_char: char,
        /// Color for the plot (named color or hex code)
        #[arg(short, long)]
        color: Option<String>,
        /// X range for functions as min:max (e.g., "-5:5")
        #[arg(short, long)]
        range: Option<String>,
        /// Number of points to evaluate for functions
        #[arg(long, default_value = "200")]
        points: usize,
    },
    /// Create line plots with various styling options
    Line {
        /// Data source: CSV file path or function expression (e.g., "data.csv" or "function:sin(x)")
        source: String,
        /// Plot title
        #[arg(short, long, default_value = "Line Plot")]
        title: String,
        /// Line style (default, ascii, smooth, dashed)
        #[arg(short = 'S', long, default_value = "default")]
        style: String,
        /// Show only points, no connecting lines
        #[arg(long)]
        points_only: bool,
        /// Show only lines, no data points
        #[arg(long)]
        lines_only: bool,
        /// Custom point character (overrides style preset)
        #[arg(short = 'p', long)]
        point_char: Option<char>,
        /// Custom line character (overrides style preset)
        #[arg(short = 'l', long)]
        line_char: Option<char>,
        /// Color for the plot (named color or hex code)
        #[arg(short, long)]
        color: Option<String>,
        /// X range for functions as min:max (e.g., "-5:5")
        #[arg(short, long)]
        range: Option<String>,
        /// Number of points to evaluate for functions
        #[arg(long, default_value = "200")]
        points: usize,
    },
    /// Create bar charts for categorical or numeric data
    Bar {
        /// Data source: CSV file path 
        source: String,
        /// Plot title
        #[arg(short, long, default_value = "Bar Chart")]
        title: String,
        /// Character to use for bars
        #[arg(short = 'b', long, default_value = "█")]
        bar_char: char,
        /// Bar width in characters
        #[arg(short = 'w', long, default_value = "1")]
        bar_width: usize,
        /// Color for the plot (named color or hex code)
        #[arg(short, long)]
        color: Option<String>,
        /// X range for functions as min:max (e.g., "-5:5")
        #[arg(short, long)]
        range: Option<String>,
        /// Number of points to evaluate for functions
        #[arg(long, default_value = "200")]
        points: usize,
        /// Custom category order (comma-separated)
        #[arg(long)]
        category_order: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Convert CLI arguments to unified plot command - this provides the deep module interface
    let plot_command = match cli.command {
        Commands::Scatter { source, title, point_char, color, range, points } => {
            let config = PlotConfig::new(source)
                .with_title(title)
                .with_color(color)
                .with_range(range)
                .with_points(points);
            
            let plot_type = PlotType::scatter()
                .with_point_char(point_char);
            
            PlotCommand::new(config, plot_type)
        }
        Commands::Line { 
            source, 
            title, 
            style, 
            points_only, 
            lines_only, 
            point_char, 
            line_char, 
            color,
            range,
            points,
        } => {
            let config = PlotConfig::new(source)
                .with_title(title)
                .with_color(color)
                .with_range(range)
                .with_points(points);
            
            // Create line style based on arguments
            let line_style = match style.as_str() {
                "ascii" => line_style::LineStyle::with_ascii(),
                "smooth" => line_style::LineStyle::with_unicode_smooth(),
                "dashed" => line_style::LineStyle::with_dashed(),
                _ => line_style::LineStyle::default(),
            };
            
            let plot_type = PlotType::line()
                .with_line_style(line_style)
                .with_points_only(points_only)
                .with_lines_only(lines_only)
                .with_line_point_char(point_char)
                .with_line_char(line_char);
            
            PlotCommand::new(config, plot_type)
        }
        Commands::Bar { 
            source, 
            title, 
            bar_char, 
            bar_width, 
            color,
            range,
            points,
            category_order,
        } => {
            let config = PlotConfig::new(source)
                .with_title(title)
                .with_color(color)
                .with_range(range)
                .with_points(points);
            
            let category_order_vec = category_order.map(|order| {
                order.split(',').map(|s| s.trim().to_string()).collect()
            });
            
            let plot_type = PlotType::bar()
                .with_bar_char(bar_char)
                .with_bar_width(bar_width)
                .with_category_order(category_order_vec);
            
            PlotCommand::new(config, plot_type)
        }
    };
    
    // Execute the command - single point of execution
    let output = plot_command.execute()?;
    println!("{}", output);
    
    Ok(())
}