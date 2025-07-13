mod data;
mod coordinates;
mod ticks;
mod layout;
mod scatter;
mod line_style;
mod line_drawing;
mod layered_canvas;
mod line_plot;

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
        /// Input CSV file
        file: String,
        /// Plot title
        #[arg(short, long, default_value = "Scatter Plot")]
        title: String,
        /// Point character
        #[arg(short = 'p', long, default_value = "●")]
        point_char: char,
        /// Color for the plot (named color or hex code)
        #[arg(short, long)]
        color: Option<String>,
    },
    Line {
        /// Input CSV file
        file: String,
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
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Scatter { file, title, point_char, color } => {
            let dataset = data::parse_csv(&file)?;
            let output = scatter::render_scatter_plot(&dataset, &title, point_char, color.as_deref());
            print!("{}", output);
        }
        Commands::Line { 
            file, 
            title, 
            style, 
            points_only, 
            lines_only, 
            point_char, 
            line_char, 
            color 
        } => {
            let dataset = data::parse_csv(&file)?;
            
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