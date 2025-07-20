use anyhow::{Result, anyhow};
use csv::ReaderBuilder;
use std::fs::File;
use crate::function;

#[derive(Debug, Clone)]
pub enum DataPoint {
    Numeric(f64, f64),
    Categorical(String, f64),
}

impl DataPoint {
    pub fn y(&self) -> f64 {
        match self {
            DataPoint::Numeric(_, y) => *y,
            DataPoint::Categorical(_, y) => *y,
        }
    }
    
    #[allow(dead_code)]
    pub fn x_numeric(&self) -> Option<f64> {
        match self {
            DataPoint::Numeric(x, _) => Some(*x),
            DataPoint::Categorical(_, _) => None,
        }
    }
    
    #[allow(dead_code)]
    pub fn x_categorical(&self) -> Option<&str> {
        match self {
            DataPoint::Numeric(_, _) => None,
            DataPoint::Categorical(x, _) => Some(x),
        }
    }
    
    #[allow(dead_code)]
    pub fn is_categorical(&self) -> bool {
        matches!(self, DataPoint::Categorical(_, _))
    }
}

#[derive(Debug, Clone)]
pub struct LegacyDataPoint {
    pub x: f64,
    pub y: f64,
}

impl From<LegacyDataPoint> for DataPoint {
    fn from(legacy: LegacyDataPoint) -> Self {
        DataPoint::Numeric(legacy.x, legacy.y)
    }
}

#[derive(Debug)]
pub struct Dataset {
    pub points: Vec<DataPoint>,
    pub x_label: String,
    pub y_label: String,
    pub is_categorical: bool,
    pub categories: Vec<String>,
}

impl Dataset {
    pub fn new_numeric(points: Vec<DataPoint>, x_label: String, y_label: String) -> Self {
        Self {
            points,
            x_label,
            y_label,
            is_categorical: false,
            categories: Vec::new(),
        }
    }
    
    pub fn new_categorical(points: Vec<DataPoint>, x_label: String, y_label: String, categories: Vec<String>) -> Self {
        Self {
            points,
            x_label,
            y_label,
            is_categorical: true,
            categories,
        }
    }
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
    
    // Generate dataset (functions always produce numeric data)
    let dataset = func.generate_dataset(x_min, x_max, points)?;
    
    // Functions already return the new Dataset format
    Ok(dataset)
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
    let mut is_categorical = false;
    let mut categories = Vec::new();
    
    for result in reader.records() {
        let record = result?;
        let x_str = record.get(0).unwrap_or("0").trim();
        let y: f64 = record.get(1).unwrap_or("0").trim().parse()?;
        
        // Try to parse x as numeric first
        if let Ok(x_numeric) = x_str.parse::<f64>() {
            // If we haven't seen categorical data yet, continue with numeric
            if !is_categorical {
                points.push(DataPoint::Numeric(x_numeric, y));
            } else {
                // Mix of categorical and numeric - treat as categorical
                let category = x_str.to_string();
                if !categories.contains(&category) {
                    categories.push(category.clone());
                }
                points.push(DataPoint::Categorical(category, y));
            }
        } else {
            // X is categorical
            if !is_categorical {
                // First categorical value found - convert previous numeric points to categorical
                is_categorical = true;
                let old_points = std::mem::take(&mut points);
                for old_point in old_points {
                    if let DataPoint::Numeric(x, y) = old_point {
                        let category = x.to_string();
                        if !categories.contains(&category) {
                            categories.push(category.clone());
                        }
                        points.push(DataPoint::Categorical(category, y));
                    }
                }
            }
            
            let category = x_str.to_string();
            if !categories.contains(&category) {
                categories.push(category.clone());
            }
            points.push(DataPoint::Categorical(category, y));
        }
    }
    
    if is_categorical {
        Ok(Dataset::new_categorical(points, x_label, y_label, categories))
    } else {
        Ok(Dataset::new_numeric(points, x_label, y_label))
    }
}

/// Reorder categorical dataset according to custom category order
pub fn reorder_categories(mut dataset: Dataset, custom_order: Vec<String>) -> Result<Dataset> {
    if !dataset.is_categorical {
        return Ok(dataset);
    }
    
    // Check that all custom categories exist in the dataset
    for custom_cat in &custom_order {
        if !dataset.categories.contains(custom_cat) {
            return Err(anyhow!("Custom category '{}' not found in data. Available categories: {:?}", 
                custom_cat, dataset.categories));
        }
    }
    
    // Reorder the categories vector
    dataset.categories = custom_order;
    
    Ok(dataset)
}