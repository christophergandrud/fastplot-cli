mod data;
mod coordinates;
mod ticks;
mod layout;
mod scatter;

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
        /// Plot symbol
        #[arg(short, long, default_value = "●")]
        symbol: char,
        /// Color for the plot (named color or hex code)
        #[arg(short, long)]
        color: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Scatter { file, title, symbol, color } => {
            let dataset = data::parse_csv(&file)?;
            let output = scatter::render_scatter_plot(&dataset, &title, symbol, color.as_deref());
            print!("{}", output);
        }
    }
    
    Ok(())
}