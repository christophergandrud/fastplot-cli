use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    pub name: String,
    pub data: Vec<f64>,
}

impl Series {
    pub fn new(name: String, data: Vec<f64>) -> Self {
        Self { name, data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataFrame {
    pub columns: Vec<Series>,
    pub headers: Option<Vec<String>>,
}

impl DataFrame {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            headers: None,
        }
    }

    pub fn with_headers(headers: Vec<String>) -> Self {
        Self {
            columns: Vec::new(),
            headers: Some(headers),
        }
    }

    pub fn add_column(&mut self, series: Series) {
        self.columns.push(series);
    }

    pub fn num_columns(&self) -> usize {
        self.columns.len()
    }

    pub fn num_rows(&self) -> usize {
        self.columns.first().map(|s| s.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.columns.is_empty() || self.num_rows() == 0
    }
}

impl Default for DataFrame {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotConfig {
    pub width: usize,
    pub height: usize,
    pub title: Option<String>,
    pub xlabel: Option<String>,
    pub ylabel: Option<String>,
    pub delimiter: char,
    pub has_header: bool,
    pub format: DataFormat,
    pub xlim: Option<(f64, f64)>,
    pub ylim: Option<(f64, f64)>,
    pub color: Option<String>,
    pub symbol: Option<char>,
}

impl Default for PlotConfig {
    fn default() -> Self {
        Self {
            width: 80,
            height: 20,
            title: None,
            xlabel: None,
            ylabel: None,
            delimiter: '\t',
            has_header: false,
            format: DataFormat::XY,
            xlim: None,
            ylim: None,
            color: None,
            symbol: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataFormat {
    XY,    // First col X, second col Y
    XYY,   // First col X, remaining cols are Y series
    XYXY,  // Alternating X,Y pairs
    YX,    // First col Y, second col X (swapped)
}

impl std::str::FromStr for DataFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "xy" => Ok(DataFormat::XY),
            "xyy" => Ok(DataFormat::XYY),
            "xyxy" => Ok(DataFormat::XYXY),
            "yx" => Ok(DataFormat::YX),
            _ => Err(format!("Unknown data format: {}", s)),
        }
    }
}