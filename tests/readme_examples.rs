use assert_cmd::Command;
use predicates::prelude::*;

/// Helper function to create a fastplot command
fn fastplot_cmd() -> Command {
    Command::cargo_bin("fastplot").unwrap()
}

/// Test basic line plot with CSV data (Quick Start example)
#[test]
fn test_line_plot_csv_basic() {
    let input = "1,2\n2,4\n3,1\n4,5\n5,3";
    
    fastplot_cmd()
        .arg("line")
        .arg("-d")
        .arg(",")
        .arg("-H")
        .arg("-t")
        .arg("Sample Data")
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("Sample Data"));
}

/// Test bar chart from file (Quick Start example)
#[test]
fn test_bar_chart_from_file() {
    fastplot_cmd()
        .arg("bar")
        .arg("test_data/sales.csv")
        .arg("-d")
        .arg(",")
        .arg("-H")
        .arg("-t")
        .arg("Sales Data")
        .arg("-c")
        .arg("blue")
        .assert()
        .success()
        .stdout(predicate::str::contains("Sales Data"));
}

/// Test scatter plot with custom symbols
#[test]
fn test_scatter_plot_custom() {
    fastplot_cmd()
        .arg("scatter")
        .arg("test_data/data.tsv")
        .arg("--symbol")
        .arg("●")
        .arg("-c")
        .arg("red")
        .arg("-w")
        .arg("60")
        .arg("--height")
        .arg("30")
        .assert()
        .success();
}

/// Test vertical bar chart with stdin
#[test]
fn test_bar_chart_vertical() {
    let input = "10\n25\n15\n30\n20";
    
    fastplot_cmd()
        .arg("bar")
        .arg("-t")
        .arg("Revenue by Quarter")
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("Revenue by Quarter"));
}

/// Test bar chart with custom symbols and colors
#[test]
fn test_bar_chart_custom_style() {
    fastplot_cmd()
        .arg("bar")
        .arg("test_data/sales.csv")
        .arg("-d")
        .arg(",")
        .arg("-H")
        .arg("--symbol")
        .arg("▓")
        .arg("-c")
        .arg("magenta")
        .arg("-w")
        .arg("50")
        .arg("--height")
        .arg("25")
        .assert()
        .success();
}

/// Test single line plot
#[test]
fn test_single_line_plot() {
    let input = "1,2\n2,4\n3,1\n4,5\n5,3";
    
    fastplot_cmd()
        .arg("line")
        .arg("-d")
        .arg(",")
        .arg("-t")
        .arg("Stock Price")
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("Stock Price"));
}

/// Test multiple series line plot
#[test]
fn test_multiple_series_line() {
    fastplot_cmd()
        .arg("lines")
        .arg("test_data/groups.csv")
        .arg("-d")
        .arg(",")
        .arg("-H")
        .arg("-t")
        .arg("Multiple Metrics")
        .assert()
        .success()
        .stdout(predicate::str::contains("Multiple Metrics"));
}

/// Test line plot with custom styling
#[test]
fn test_line_plot_custom_style() {
    fastplot_cmd()
        .arg("line")
        .arg("test_data/data.tsv")
        .arg("--symbol")
        .arg("●")
        .arg("-c")
        .arg("blue")
        .arg("-w")
        .arg("70")
        .arg("--height")
        .arg("20")
        .assert()
        .success();
}

/// Test basic scatter plot
#[test]
fn test_basic_scatter_plot() {
    let input = "1,2\n2,4\n3,1\n4,5\n5,3";
    
    fastplot_cmd()
        .arg("scatter")
        .arg("-d")
        .arg(",")
        .write_stdin(input)
        .assert()
        .success();
}

/// Test histogram with auto-calculated bins
#[test]
fn test_histogram_auto_bins() {
    let input = "1\n2\n2\n3\n3\n3\n4\n4\n5";
    
    fastplot_cmd()
        .arg("hist")
        .arg("-t")
        .arg("Distribution")
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("Distribution"));
}

/// Test histogram with color
#[test]
fn test_histogram_with_color() {
    fastplot_cmd()
        .arg("hist")
        .arg("test_data/data.csv")
        .arg("-d")
        .arg(",")
        .arg("-H")
        .arg("-c")
        .arg("green")
        .assert()
        .success();
}

/// Test density plot
#[test]
fn test_density_plot() {
    fastplot_cmd()
        .arg("density")
        .arg("test_data/data.csv")
        .arg("-d")
        .arg(",")
        .arg("-H")
        .arg("-t")
        .arg("Probability Density")
        .assert()
        .success()
        .stdout(predicate::str::contains("Probability Density"));
}

/// Test box plot
#[test]
fn test_box_plot() {
    fastplot_cmd()
        .arg("boxplot")
        .arg("test_data/data.csv")
        .arg("-d")
        .arg(",")
        .arg("-H")
        .arg("-t")
        .arg("Data Distribution")
        .assert()
        .success()
        .stdout(predicate::str::contains("Data Distribution"));
}

/// Test box plot with multiple groups
#[test]
fn test_box_plot_groups() {
    fastplot_cmd()
        .arg("boxplot")
        .arg("test_data/groups.csv")
        .arg("-d")
        .arg(",")
        .arg("-H")
        .arg("-t")
        .arg("Group Comparison")
        .assert()
        .success()
        .stdout(predicate::str::contains("Group Comparison"));
}

/// Test professional styling example
#[test]
fn test_professional_styling() {
    fastplot_cmd()
        .arg("line")
        .arg("test_data/data.csv")
        .arg("-d")
        .arg(",")
        .arg("-H")
        .arg("-t")
        .arg("Revenue Trends")
        .arg("--xlabel")
        .arg("Month")
        .arg("--ylabel")
        .arg("Revenue ($)")
        .arg("-c")
        .arg("blue")
        .arg("-w")
        .arg("80")
        .arg("--height")
        .arg("25")
        .assert()
        .success()
        .stdout(predicate::str::contains("Revenue Trends"));
}

/// Test colorful scatter plot with limits
#[test]
fn test_colorful_scatter_with_limits() {
    fastplot_cmd()
        .arg("scatter")
        .arg("test_data/data.csv")
        .arg("-d")
        .arg(",")
        .arg("-H")
        .arg("--symbol")
        .arg("●")
        .arg("-c")
        .arg("red")
        .arg("--xlim")
        .arg("0,10")
        .arg("--ylim")
        .arg("0,5")
        .arg("-t")
        .arg("Performance Metrics")
        .assert()
        .success()
        .stdout(predicate::str::contains("Performance Metrics"));
}

/// Test box plot with styling
#[test]
fn test_box_plot_styling() {
    fastplot_cmd()
        .arg("boxplot")
        .arg("test_data/data.csv")
        .arg("-d")
        .arg(",")
        .arg("-H")
        .arg("-c")
        .arg("green")
        .arg("-w")
        .arg("40")
        .arg("--height")
        .arg("15")
        .assert()
        .success();
}

/// Test multi-series line plot scientific style
#[test]
fn test_scientific_multi_series() {
    fastplot_cmd()
        .arg("lines")
        .arg("test_data/groups.csv")
        .arg("-d")
        .arg(",")
        .arg("-H")
        .arg("-t")
        .arg("Treatment vs Control")
        .arg("--xlabel")
        .arg("Time (hours)")
        .arg("--ylabel")
        .arg("Response")
        .assert()
        .success()
        .stdout(predicate::str::contains("Treatment vs Control"));
}

/// Test density plot with title
#[test]
fn test_density_plot_with_title() {
    fastplot_cmd()
        .arg("density")
        .arg("test_data/data.csv")
        .arg("-d")
        .arg(",")
        .arg("-H")
        .arg("-t")
        .arg("Distribution Analysis")
        .assert()
        .success()
        .stdout(predicate::str::contains("Distribution Analysis"));
}

/// Test help output for each command
#[test]
fn test_help_commands() {
    let commands = ["bar", "hist", "line", "lines", "scatter", "density", "boxplot", "count"];
    
    for cmd in &commands {
        fastplot_cmd()
            .arg(cmd)
            .arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("Usage:"));
    }
}

/// Test error handling for missing file
#[test]
fn test_missing_file_error() {
    fastplot_cmd()
        .arg("line")
        .arg("nonexistent_file.csv")
        .assert()
        .failure();
}

/// Test error handling for invalid delimiter
#[test]
fn test_invalid_delimiter() {
    let input = "1,2\n2,4\n3,1";
    
    fastplot_cmd()
        .arg("line")
        .arg("-d")
        .arg("invalid")  // Multi-character delimiter should fail
        .write_stdin(input)
        .assert()
        .failure();
}

/// Test that output is generated for all plot types
#[test]
fn test_all_plot_types_generate_output() {
    let single_input = "1\n2\n3\n4\n5";
    let xy_input = "1,2\n2,4\n3,1\n4,5\n5,3";
    
    // Test single-value plots
    let single_commands = ["bar", "hist", "density", "boxplot", "count"];
    for cmd in &single_commands {
        fastplot_cmd()
            .arg(cmd)
            .arg("-d")
            .arg(",")
            .write_stdin(single_input)
            .assert()
            .success()
            .stdout(predicate::str::is_empty().not());
    }
    
    // Test X,Y plots
    let xy_commands = ["line", "scatter"];
    for cmd in &xy_commands {
        fastplot_cmd()
            .arg(cmd)
            .arg("-d")
            .arg(",")
            .write_stdin(xy_input)
            .assert()
            .success()
            .stdout(predicate::str::is_empty().not());
    }
}

/// Test width and height parameters
#[test]
fn test_width_height_parameters() {
    let input = "1,2\n2,4\n3,1";
    
    fastplot_cmd()
        .arg("line")
        .arg("-d")
        .arg(",")
        .arg("-w")
        .arg("100")
        .arg("--height")
        .arg("40")
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

/// Test color options
#[test]
fn test_color_options() {
    let input = "1,2\n2,4\n3,1";
    let colors = ["red", "green", "blue", "yellow", "magenta", "cyan"];
    
    for color in &colors {
        fastplot_cmd()
            .arg("line")
            .arg("-d")
            .arg(",")
            .arg("-c")
            .arg(color)
            .write_stdin(input)
            .assert()
            .success();
    }
}

/// Test data format options
#[test]
fn test_data_format_options() {
    let input = "1,2\n2,4\n3,1";
    let formats = ["xy", "xyy", "xyxy", "yx"];
    
    for format in &formats {
        fastplot_cmd()
            .arg("line")
            .arg("-d")
            .arg(",")
            .arg("--fmt")
            .arg(format)
            .write_stdin(input)
            .assert()
            .success();
    }
}