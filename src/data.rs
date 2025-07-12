use anyhow::Result;
use csv::ReaderBuilder;
use std::fs::File;

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
        let x: f64 = record.get(0).unwrap_or("0").parse()?;
        let y: f64 = record.get(1).unwrap_or("0").parse()?;
        points.push(DataPoint { x, y });
    }
    
    Ok(Dataset {
        points,
        x_label,
        y_label,
    })
}