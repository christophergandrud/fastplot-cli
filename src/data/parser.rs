use crate::data::types::{DataFrame, Series, DataFormat};
use anyhow::{Result, anyhow};
use std::io::Read;
use csv::{ReaderBuilder, StringRecord};

fn detect_likely_delimiters(field: &str, current_delimiter: char) -> Vec<char> {
    let common_delimiters = [',', '\t', ' ', ';', '|'];
    let mut detected = Vec::new();
    
    for &delim in &common_delimiters {
        if delim != current_delimiter && field.contains(delim) {
            detected.push(delim);
        }
    }
    detected
}

fn suggest_delimiter_fix(field: &str, current_delimiter: char) -> String {
    let detected = detect_likely_delimiters(field, current_delimiter);
    let mut suggestions = Vec::new();
    
    for &delim in &detected {
        let delim_name = match delim {
            ',' => "comma",
            '\t' => "tab", 
            ' ' => "space",
            ';' => "semicolon",
            '|' => "pipe",
            _ => "delimiter",
        };
        let flag = match delim {
            '\t' => "-d\\t".to_string(),
            ' ' => "-d' '".to_string(),
            _ => format!("-d{}", delim),
        };
        suggestions.push(format!("Try {} delimiter with {}", delim_name, flag));
    }
    
    if suggestions.is_empty() {
        "Check your data format and delimiter setting.".to_string()
    } else {
        suggestions.join(" or ")
    }
}

#[allow(dead_code)]
pub struct FastParser {
    delimiter: u8,
    has_header: bool,
    buffer_size: usize,
}

impl FastParser {
    pub fn new(delimiter: char, has_header: bool) -> Self {
        Self {
            delimiter: delimiter as u8,
            has_header,
            buffer_size: 8192, // 8KB buffer
        }
    }

    pub fn parse_stream<R: Read>(&self, reader: R) -> Result<DataFrame> {
        let mut csv_reader = ReaderBuilder::new()
            .delimiter(self.delimiter)
            .has_headers(self.has_header)
            .buffer_capacity(self.buffer_size)
            .from_reader(reader);

        let mut dataframe = DataFrame::new();
        let mut headers: Option<Vec<String>> = None;

        if self.has_header {
            let header_record = csv_reader.headers()?;
            headers = Some(header_record.iter().map(|s| s.to_string()).collect());
            dataframe.headers = headers.clone();
        }

        let mut records: Vec<StringRecord> = Vec::new();
        for result in csv_reader.records() {
            let record = result?;
            records.push(record);
        }

        if records.is_empty() {
            return Ok(dataframe);
        }

        let num_columns = records[0].len();
        let mut columns: Vec<Vec<f64>> = vec![Vec::with_capacity(records.len()); num_columns];

        for record in records {
            if record.len() != num_columns {
                return Err(anyhow!("Inconsistent number of columns in data"));
            }

            for (col_idx, field) in record.iter().enumerate() {
                let value = field.trim().parse::<f64>()
                    .map_err(|_| {
                        let delimiter_char = self.delimiter as char;
                        let suggestions = suggest_delimiter_fix(field, delimiter_char);
                        anyhow!("Failed to parse '{}' as number. {}", field, suggestions)
                    })?;
                columns[col_idx].push(value);
            }
        }

        for (idx, column_data) in columns.into_iter().enumerate() {
            let column_name = if let Some(ref headers) = headers {
                headers.get(idx).cloned().unwrap_or_else(|| format!("col_{}", idx))
            } else {
                format!("col_{}", idx)
            };

            dataframe.add_column(Series::new(column_name, column_data));
        }

        Ok(dataframe)
    }

    pub fn parse_string(&self, data: &str) -> Result<DataFrame> {
        self.parse_stream(data.as_bytes())
    }

    pub fn parse_string_with_auto_detect(&self, data: &str) -> Result<DataFrame> {
        match self.parse_string(data) {
            Ok(df) => Ok(df),
            Err(original_error) => {
                let first_line = data.lines().next().unwrap_or("");
                let detected_delimiters = detect_likely_delimiters(first_line, self.delimiter as char);
                
                for &delim in &detected_delimiters {
                    let auto_parser = FastParser::new(delim, self.has_header);
                    if let Ok(df) = auto_parser.parse_string(data) {
                        eprintln!("Auto-detected {} delimiter", match delim {
                            ',' => "comma",
                            '\t' => "tab",
                            ' ' => "space", 
                            ';' => "semicolon",
                            '|' => "pipe",
                            _ => "unknown",
                        });
                        return Ok(df);
                    }
                }
                
                Err(original_error)
            }
        }
    }

    pub fn parse_with_format<R: Read>(&self, reader: R, format: DataFormat) -> Result<DataFrame> {
        let mut base_df = self.parse_stream(reader)?;
        
        match format {
            DataFormat::XY => {
                if base_df.num_columns() < 2 {
                    return Err(anyhow!("XY format requires at least 2 columns"));
                }
                Ok(base_df)
            },
            DataFormat::YX => {
                if base_df.num_columns() < 2 {
                    return Err(anyhow!("YX format requires at least 2 columns"));
                }
                base_df.columns.swap(0, 1);
                Ok(base_df)
            },
            DataFormat::XYY => {
                if base_df.num_columns() < 2 {
                    return Err(anyhow!("XYY format requires at least 2 columns"));
                }
                Ok(base_df)
            },
            DataFormat::XYXY => {
                if base_df.num_columns() % 2 != 0 {
                    return Err(anyhow!("XYXY format requires even number of columns"));
                }
                Ok(base_df)
            },
        }
    }
}

impl Default for FastParser {
    fn default() -> Self {
        Self::new('\t', false)
    }
}

#[allow(dead_code)]
pub fn parse_csv_data(data: &str, delimiter: char, has_header: bool) -> Result<DataFrame> {
    let parser = FastParser::new(delimiter, has_header);
    parser.parse_string(data)
}

#[allow(dead_code)]
pub fn parse_file_data<R: Read>(reader: R, delimiter: char, has_header: bool, format: DataFormat) -> Result<DataFrame> {
    let parser = FastParser::new(delimiter, has_header);
    parser.parse_with_format(reader, format)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_csv_parsing() {
        let data = "1.0,2.0\n3.0,4.0\n5.0,6.0";
        let parser = FastParser::new(',', false);
        let df = parser.parse_string(data).unwrap();
        
        assert_eq!(df.num_columns(), 2);
        assert_eq!(df.num_rows(), 3);
        assert_eq!(df.columns[0].data, vec![1.0, 3.0, 5.0]);
        assert_eq!(df.columns[1].data, vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_csv_with_headers() {
        let data = "x,y\n1.0,2.0\n3.0,4.0";
        let parser = FastParser::new(',', true);
        let df = parser.parse_string(data).unwrap();
        
        assert_eq!(df.num_columns(), 2);
        assert_eq!(df.num_rows(), 2);
        assert_eq!(df.headers, Some(vec!["x".to_string(), "y".to_string()]));
        assert_eq!(df.columns[0].name, "x");
        assert_eq!(df.columns[1].name, "y");
    }

    #[test]
    fn test_tsv_parsing() {
        let data = "1.0\t2.0\n3.0\t4.0";
        let parser = FastParser::new('\t', false);
        let df = parser.parse_string(data).unwrap();
        
        assert_eq!(df.num_columns(), 2);
        assert_eq!(df.num_rows(), 2);
    }

    #[test]
    fn test_yx_format() {
        let data = "2.0,1.0\n4.0,3.0";
        let parser = FastParser::new(',', false);
        let df = parser.parse_with_format(data.as_bytes(), DataFormat::YX).unwrap();
        
        // Should swap first two columns
        assert_eq!(df.columns[0].data, vec![1.0, 3.0]); // X values
        assert_eq!(df.columns[1].data, vec![2.0, 4.0]); // Y values
    }

    #[test]
    fn test_empty_data() {
        let data = "";
        let parser = FastParser::new(',', false);
        let df = parser.parse_string(data).unwrap();
        
        assert!(df.is_empty());
        assert_eq!(df.num_columns(), 0);
        assert_eq!(df.num_rows(), 0);
    }

    #[test]
    fn test_parse_error() {
        let data = "1.0,invalid\n3.0,4.0";
        let parser = FastParser::new(',', false);
        let result = parser.parse_string(data);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_inconsistent_columns() {
        let data = "1.0,2.0\n3.0"; // Second row has fewer columns
        let parser = FastParser::new(',', false);
        let result = parser.parse_string(data);
        
        assert!(result.is_err());
    }
}