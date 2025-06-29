mod cli;
mod data;
mod plot;
mod config;
mod testing;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    
    println!("FastPlot CLI - Phase 1 Implementation");
    println!("Args: {:?}", args);
    
    Ok(())
}