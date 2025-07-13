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
    Function {
        /// Mathematical expression to plot (e.g., "sin(x)", "x^2 + 2*x + 1")
        expression: String,
        /// Plot title
        #[arg(short, long)]
        title: Option<String>,
        /// X range as min:max (e.g., "-5:5")
        #[arg(short, long)]
        range: Option<String>,
        /// Number of points to evaluate
        #[arg(short, long, default_value = "200")]
        points: usize,
        /// Line style (default, ascii, smooth, dashed)
        #[arg(short = 'S', long, default_value = "default")]
        style: String,
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
        Commands::Function { 
            expression, 
            title, 
            range, 
            points, 
            style, 
            color 
        } => {
            // Parse range or use intelligent default
            let (x_min, x_max) = if let Some(range_str) = range {
                parse_range(&range_str)?
            } else {
                function::detect_range(&expression)
            };
            
            // Create function and generate dataset
            let func = function::Function::new(&expression);
            let dataset = func.generate_dataset(x_min, x_max, Some(points))?;
            
            // Determine title
            let plot_title = title.unwrap_or_else(|| format!("f(x) = {}", expression));
            
            // Create line style
            let line_style = match style.as_str() {
                "ascii" => line_style::LineStyle::with_ascii(),
                "smooth" => line_style::LineStyle::with_unicode_smooth(),
                "dashed" => line_style::LineStyle::with_dashed(),
                _ => line_style::LineStyle::default(),
            };
            
            let output = line_plot::render_line_plot(&dataset, &plot_title, line_style, color.as_deref());
            print!("{}", output);
        }
    }
    
    Ok(())
}

fn parse_range(range_str: &str) -> Result<(f64, f64)> {
    let parts: Vec<&str> = range_str.split(':').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Range must be in format 'min:max', got: {}", range_str));
    }
    
    let min: f64 = parts[0].parse()
        .map_err(|_| anyhow::anyhow!("Invalid minimum value: {}", parts[0]))?;
    let max: f64 = parts[1].parse()
        .map_err(|_| anyhow::anyhow!("Invalid maximum value: {}", parts[1]))?;
    
    if min >= max {
        return Err(anyhow::anyhow!("Minimum value must be less than maximum value"));
    }
    
    Ok((min, max))
}