use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("fastplot").unwrap();
    cmd.arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A high-performance terminal plotting tool"))
        .stdout(predicate::str::contains("Usage: fastplot"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("fastplot").unwrap();
    cmd.arg("--version");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("fastplot-cli"));
}

#[test]
fn test_basic_line_command() {
    let mut cmd = Command::cargo_bin("fastplot").unwrap();
    cmd.arg("line");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("FastPlot CLI - Phase 1 Implementation"))
        .stdout(predicate::str::contains("Line"));
}

#[test]
fn test_command_line_parsing() {
    let mut cmd = Command::cargo_bin("fastplot").unwrap();
    cmd.args(&["line", "--width", "100", "--height", "25", "--title", "Test Plot"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("width: 100"))
        .stdout(predicate::str::contains("height: 25"))
        .stdout(predicate::str::contains("title: Some(\"Test Plot\")"));
}

#[test]
fn test_delimiter_options() {
    let mut cmd = Command::cargo_bin("fastplot").unwrap();
    cmd.args(&["scatter", "-d", ",", "-H"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("delimiter: ','"))
        .stdout(predicate::str::contains("has_header: true"));
}

#[test]
fn test_different_plot_types() {
    let plot_types = ["bar", "hist", "line", "lines", "scatter", "density", "boxplot", "count"];
    
    for plot_type in plot_types {
        let mut cmd = Command::cargo_bin("fastplot").unwrap();
        cmd.arg(plot_type);
        
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("FastPlot CLI"));
    }
}

#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("fastplot").unwrap();
    cmd.arg("invalid_command");
    
    cmd.assert()
        .failure();
}

#[test]
fn test_xlim_ylim_parsing() {
    let mut cmd = Command::cargo_bin("fastplot").unwrap();
    cmd.args(&["line", "--xlim", "0,10", "--ylim", "-1,1"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("xlim: Some((0.0, 10.0))"))
        .stdout(predicate::str::contains("ylim: Some((-1.0, 1.0))"));
}

#[test]
fn test_invalid_xlim_format() {
    let mut cmd = Command::cargo_bin("fastplot").unwrap();
    cmd.args(&["line", "--xlim", "invalid"]);
    
    cmd.assert()
        .failure();
}

#[test]
fn test_format_option() {
    let formats = ["xy", "xyy", "xyxy", "yx"];
    
    for format in formats {
        let mut cmd = Command::cargo_bin("fastplot").unwrap();
        cmd.args(&["line", "--fmt", format]);
        
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(format!("format: {}", format.to_uppercase())));
    }
}

#[test]
fn test_benchmark_command() {
    let mut cmd = Command::cargo_bin("fastplot").unwrap();
    cmd.args(&["benchmark", "--size", "100", "--plot-type", "line"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Benchmark"));
}

#[test]
fn test_generate_command() {
    let mut cmd = Command::cargo_bin("fastplot").unwrap();
    cmd.args(&["generate", "--dataset", "sine", "--size", "50"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generate"));
}

#[test]
fn test_file_input() {
    // Create a temporary CSV file
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "x,y").unwrap();
    writeln!(temp_file, "1,2").unwrap();
    writeln!(temp_file, "3,4").unwrap();
    
    let mut cmd = Command::cargo_bin("fastplot").unwrap();
    cmd.args(&["line", temp_file.path().to_str().unwrap()]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("FastPlot CLI"));
}

#[test]
fn test_output_options() {
    let mut cmd = Command::cargo_bin("fastplot").unwrap();
    cmd.args(&["line", "--pass-data", "--profile", "--benchmark"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("pass_data: true"))
        .stdout(predicate::str::contains("profile: true"))
        .stdout(predicate::str::contains("benchmark: true"));
}

#[test]
fn test_color_and_symbol_options() {
    let mut cmd = Command::cargo_bin("fastplot").unwrap();
    cmd.args(&["scatter", "--color", "red", "--symbol", "*"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("color: Some(\"red\")"))
        .stdout(predicate::str::contains("symbol: Some('*')"));
}

#[test]
fn test_axis_labels() {
    let mut cmd = Command::cargo_bin("fastplot").unwrap();
    cmd.args(&["line", "--xlabel", "Time", "--ylabel", "Temperature"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("xlabel: Some(\"Time\")"))
        .stdout(predicate::str::contains("ylabel: Some(\"Temperature\")"));
}

#[test]
fn test_progress_mode() {
    let mut cmd = Command::cargo_bin("fastplot").unwrap();
    cmd.args(&["line", "--progress"]);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("progress: true"));
}