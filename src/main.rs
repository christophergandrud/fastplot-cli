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

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "fastplot")]
#[command(about = "A fast terminal plotting tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
    Line {
        /// Data source: CSV file path or function expression (e.g., "data.csv" or "function:sin(x)")
        source: String,
        /// Plot title
        #[arg(short, long, default_value = "Line Plot")]
        title: String,
        /// Line style (default, ascii, smooth, dashed)
        #[arg(short = 'S', long, default_value = "default")]
        style: String,
        /// Show only points (no lines)
        #[arg(long)]
        points_only: bool,
        /// Show only lines (no points)
        #[arg(long)]
        lines_only: bool,
        /// Point character
        #[arg(short = 'p', long)]
        point_char: Option<char>,
        /// Line character
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
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Scatter { source, title, point_char, color, range, points } => {
            let dataset = data::parse_data_source(&source, range.as_deref(), Some(points))?;
            let output = scatter::render_scatter_plot(&dataset, &title, point_char, color.as_deref());
            print!("{}", output);
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
            points
        } => {
            let dataset = data::parse_data_source(&source, range.as_deref(), Some(points))?;
            
            // Create line style based on arguments
            let mut line_style = match style.as_str() {
                "ascii" => line_style::LineStyle::with_ascii(),
                "smooth" => line_style::LineStyle::with_unicode_smooth(),
                "dashed" => line_style::LineStyle::with_dashed(),
                _ => line_style::LineStyle::default(),
            };
            
            // Override with specific options
            if points_only {
                line_style = line_style::LineStyle::points_only();
            } else if lines_only {
                line_style = line_style::LineStyle::lines_only();
            }
            
            // Override characters if specified
            if let Some(pc) = point_char {
                line_style.point_char = pc;
            }
            if let Some(lc) = line_char {
                line_style.line_char = lc;
            }
            
            let output = line_plot::render_line_plot(&dataset, &title, line_style, color.as_deref());
            print!("{}", output);
        }
    }
    
    Ok(())
}

