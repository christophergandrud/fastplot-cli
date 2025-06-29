use fastplot_cli::data::{FastParser, DataFormat};
use fastplot_cli::plot::{Plot, LinePlot, BarPlot, ScatterPlot};
use fastplot_cli::data::PlotConfig;

#[test]
fn test_youplot_csv_compatibility() {
    // Test CSV format that YouPlot would accept
    let csv_data = "x,y\n1,2\n3,4\n5,6";
    let parser = FastParser::new(',', true);
    let df = parser.parse_string(csv_data).unwrap();
    
    assert_eq!(df.num_columns(), 2);
    assert_eq!(df.num_rows(), 3);
    assert_eq!(df.headers, Some(vec!["x".to_string(), "y".to_string()]));
}

#[test]
fn test_youplot_tsv_compatibility() {
    // Test TSV format (YouPlot's default)
    let tsv_data = "time\tvalue\n0\t1.0\n1\t2.5\n2\t4.0";
    let parser = FastParser::new('\t', true);
    let df = parser.parse_string(tsv_data).unwrap();
    
    assert_eq!(df.num_columns(), 2);
    assert_eq!(df.num_rows(), 3);
    assert_eq!(df.columns[0].name, "time");
    assert_eq!(df.columns[1].name, "value");
}

#[test]
fn test_youplot_no_header_compatibility() {
    // Test data without headers (YouPlot default behavior)
    let data = "1\t2\n3\t4\n5\t6";
    let parser = FastParser::new('\t', false);
    let df = parser.parse_string(data).unwrap();
    
    assert_eq!(df.num_columns(), 2);
    assert_eq!(df.num_rows(), 3);
    assert_eq!(df.headers, None);
    assert_eq!(df.columns[0].name, "col_0");
    assert_eq!(df.columns[1].name, "col_1");
}

#[test]
fn test_youplot_xy_format() {
    // Test XY format (YouPlot default)
    let data = "1,2\n3,4\n5,6";
    let parser = FastParser::new(',', false);
    let df = parser.parse_with_format(data.as_bytes(), DataFormat::XY).unwrap();
    
    assert_eq!(df.num_columns(), 2);
    assert_eq!(df.columns[0].data, vec![1.0, 3.0, 5.0]); // X values
    assert_eq!(df.columns[1].data, vec![2.0, 4.0, 6.0]); // Y values
}

#[test]
fn test_youplot_yx_format() {
    // Test YX format (swapped columns)
    let data = "2,1\n4,3\n6,5";
    let parser = FastParser::new(',', false);
    let df = parser.parse_with_format(data.as_bytes(), DataFormat::YX).unwrap();
    
    assert_eq!(df.num_columns(), 2);
    assert_eq!(df.columns[0].data, vec![1.0, 3.0, 5.0]); // X values (swapped)
    assert_eq!(df.columns[1].data, vec![2.0, 4.0, 6.0]); // Y values (swapped)
}

#[test]
fn test_youplot_xyy_format() {
    // Test XYY format (one X column, multiple Y columns)
    let data = "1,2,3\n4,5,6\n7,8,9";
    let parser = FastParser::new(',', false);
    let df = parser.parse_with_format(data.as_bytes(), DataFormat::XYY).unwrap();
    
    assert_eq!(df.num_columns(), 3);
    assert_eq!(df.columns[0].data, vec![1.0, 4.0, 7.0]); // X values
    assert_eq!(df.columns[1].data, vec![2.0, 5.0, 8.0]); // Y1 values
    assert_eq!(df.columns[2].data, vec![3.0, 6.0, 9.0]); // Y2 values
}

#[test]
fn test_youplot_xyxy_format() {
    // Test XYXY format (alternating X,Y pairs)
    let data = "1,2,3,4\n5,6,7,8";
    let parser = FastParser::new(',', false);
    let df = parser.parse_with_format(data.as_bytes(), DataFormat::XYXY).unwrap();
    
    assert_eq!(df.num_columns(), 4);
    // Should maintain original column order for XYXY
    assert_eq!(df.columns[0].data, vec![1.0, 5.0]); // X1
    assert_eq!(df.columns[1].data, vec![2.0, 6.0]); // Y1
    assert_eq!(df.columns[2].data, vec![3.0, 7.0]); // X2
    assert_eq!(df.columns[3].data, vec![4.0, 8.0]); // Y2
}

#[test]
fn test_youplot_space_delimiter() {
    // Test space-delimited data (common in scientific data)
    let data = "1.0 2.0\n3.0 4.0\n5.0 6.0";
    let parser = FastParser::new(' ', false);
    let df = parser.parse_string(data).unwrap();
    
    assert_eq!(df.num_columns(), 2);
    assert_eq!(df.num_rows(), 3);
}

#[test]
fn test_youplot_decimal_handling() {
    // Test various decimal formats
    let data = "1.5,2.75\n3.25,4.0\n5.125,6.875";
    let parser = FastParser::new(',', false);
    let df = parser.parse_string(data).unwrap();
    
    assert_eq!(df.columns[0].data[0], 1.5);
    assert_eq!(df.columns[1].data[0], 2.75);
    assert_eq!(df.columns[0].data[2], 5.125);
    assert_eq!(df.columns[1].data[2], 6.875);
}

#[test]
fn test_youplot_scientific_notation() {
    // Test scientific notation numbers
    let data = "1e-3,2.5e2\n3.14e0,1.23e-4";
    let parser = FastParser::new(',', false);
    let df = parser.parse_string(data).unwrap();
    
    assert_eq!(df.columns[0].data[0], 0.001);
    assert_eq!(df.columns[1].data[0], 250.0);
    assert_eq!(df.columns[0].data[1], 3.14);
    assert_eq!(df.columns[1].data[1], 0.000123);
}

#[test]
fn test_youplot_negative_numbers() {
    // Test negative numbers
    let data = "-1,2\n3,-4\n-5,-6";
    let parser = FastParser::new(',', false);
    let df = parser.parse_string(data).unwrap();
    
    assert_eq!(df.columns[0].data, vec![-1.0, 3.0, -5.0]);
    assert_eq!(df.columns[1].data, vec![2.0, -4.0, -6.0]);
}

#[test]
fn test_youplot_whitespace_handling() {
    // Test data with extra whitespace (should be trimmed)
    let data = " 1 , 2 \n 3 , 4 \n 5 , 6 ";
    let parser = FastParser::new(',', false);
    let df = parser.parse_string(data).unwrap();
    
    assert_eq!(df.columns[0].data, vec![1.0, 3.0, 5.0]);
    assert_eq!(df.columns[1].data, vec![2.0, 4.0, 6.0]);
}

#[test]
fn test_youplot_default_plot_config() {
    // Test that default config matches YouPlot-like settings
    let config = PlotConfig::default();
    
    assert_eq!(config.width, 80);
    assert_eq!(config.height, 20);
    assert_eq!(config.delimiter, '\t'); // YouPlot default is tab
    assert_eq!(config.has_header, false); // YouPlot default is no header
    assert!(matches!(config.format, DataFormat::XY));
    assert_eq!(config.title, None);
}

#[test]
fn test_plot_rendering_compatibility() {
    // Test that plots can be rendered (placeholder functionality)
    let data = "1,2\n3,4\n5,6";
    let parser = FastParser::new(',', false);
    let df = parser.parse_string(data).unwrap();
    
    let config = PlotConfig::default();
    
    // Test line plot
    let line_plot = LinePlot;
    let result = line_plot.render(&df, &config).unwrap();
    assert!(result.contains("Line plot"));
    
    // Test bar plot
    let bar_plot = BarPlot;
    let result = bar_plot.render(&df, &config).unwrap();
    assert!(result.contains("Bar plot"));
    
    // Test scatter plot
    let scatter_plot = ScatterPlot;
    let result = scatter_plot.render(&df, &config).unwrap();
    assert!(result.contains("Scatter plot"));
}

#[test]
fn test_error_handling_compatibility() {
    // Test error cases that YouPlot would also encounter
    let parser = FastParser::new(',', false);
    
    // Invalid number format
    let invalid_data = "1,invalid\n3,4";
    let result = parser.parse_string(invalid_data);
    assert!(result.is_err());
    
    // Inconsistent column count
    let inconsistent_data = "1,2\n3"; // Missing second column
    let result = parser.parse_string(inconsistent_data);
    assert!(result.is_err());
    
    // Empty data should not error but return empty DataFrame
    let empty_data = "";
    let result = parser.parse_string(empty_data);
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_large_dataset_compatibility() {
    // Test that we can handle reasonably large datasets like YouPlot
    let mut large_data = String::from("x,y\n");
    for i in 0..10000 {
        large_data.push_str(&format!("{},{}\n", i, i * 2));
    }
    
    let parser = FastParser::new(',', true);
    let df = parser.parse_string(&large_data).unwrap();
    
    assert_eq!(df.num_rows(), 10000);
    assert_eq!(df.num_columns(), 2);
    assert_eq!(df.columns[0].data[9999], 9999.0);
    assert_eq!(df.columns[1].data[9999], 19998.0);
}