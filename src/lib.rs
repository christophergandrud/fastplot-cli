pub mod cli;
pub mod data;
pub mod plot;
pub mod config;
pub mod testing;

// Re-export commonly used items
pub use data::{DataFrame, Series, PlotConfig, DataFormat};
pub use plot::Canvas;