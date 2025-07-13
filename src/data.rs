use anyhow::{Result, anyhow};
use csv::ReaderBuilder;
use std::fs::File;
use crate::function;

#[derive(Debug, Clone)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Dataset {
    pub points: Vec<DataPoint>,
    pub x_label: String,
    pub y_label: String,
}

/// Parse data source which can be either a CSV file path or a function expression
pub fn parse_data_source(source: &str, range: Option<&str>, points: Option<usize>) -> Result<Dataset> {
    if source.starts_with("function:") {
        // Parse function expression
        let expression = source.strip_prefix("function:").unwrap();
        parse_function_expression(expression, range, points)
    } else {
        // Parse as CSV file
        parse_csv(source)
    }
}

/// Parse function expression and generate dataset
pub fn parse_function_expression(expression: &str, range: Option<&str>, points: Option<usize>) -> Result<Dataset> {
    let func = function::Function::new(expression);
    
    // Determine range
    let (x_min, x_max) = if let Some(range_str) = range {
        parse_range(range_str)?
    } else {
        function::detect_range(expression)
    };
    
    // Generate dataset
    func.generate_dataset(x_min, x_max, points)
}

/// Parse range string in format "min:max"
fn parse_range(range_str: &str) -> Result<(f64, f64)> {
    let parts: Vec<&str> = range_str.split(':').collect();
    if parts.len() != 2 {
        return Err(anyhow!("Range must be in format 'min:max', got: {}", range_str));
    }
    
    let min: f64 = parts[0].parse()
        .map_err(|_| anyhow!("Invalid minimum value: {}", parts[0]))?;
    let max: f64 = parts[1].parse()
        .map_err(|_| anyhow!("Invalid maximum value: {}", parts[1]))?;
    
    if min >= max {
        return Err(anyhow!("Minimum value must be less than maximum value"));
    }
    
    Ok((min, max))
}

pub fn parse_csv(file_path: &str) -> Result<Dataset> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    
    // Get headers for axis labels
    let headers = reader.headers()?.clone();
    let x_label = headers.get(0).unwrap_or("x").to_string();
    let y_label = headers.get(1).unwrap_or("y").to_string();
    
    let mut points = Vec::new();
    
    for result in reader.records() {
        let record = result?;
        let x: f64 = record.get(0).unwrap_or("0").trim().parse()?;
        let y: f64 = record.get(1).unwrap_or("0").trim().parse()?;
        points.push(DataPoint { x, y });
    }
    
    Ok(Dataset {
        points,
        x_label,
        y_label,
    })
}