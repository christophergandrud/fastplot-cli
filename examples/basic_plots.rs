use fastplot_cli::data::{FastParser, PlotConfig, DataFormat};
use fastplot_cli::plot::{Plot, LinePlot, BarPlot, ScatterPlot, Histogram, Canvas};
use anyhow::Result;

fn main() -> Result<()> {
    println!("FastPlot CLI - Basic Plotting Examples");
    println!("=====================================\n");

    // Example 1: Simple line plot from CSV data
    println!("Example 1: Simple Line Plot");
    let csv_data = "x,y\n0,0\n1,1\n2,4\n3,9\n4,16\n5,25";
    let parser = FastParser::new(',', true);
    let df = parser.parse_string(csv_data)?;
    
    let config = PlotConfig {
        width: 60,
        height: 15,
        title: Some("Quadratic Function".to_string()),
        xlabel: Some("X".to_string()),
        ylabel: Some("Y".to_string()),
        ..Default::default()
    };
    
    let line_plot = LinePlot;
    let result = line_plot.render(&df, &config)?;
    println!("{}\n", result);

    // Example 2: Bar plot
    println!("Example 2: Bar Plot");
    let bar_data = "category,value\nA,10\nB,25\nC,15\nD,30\nE,20";
    let df = parser.parse_string(bar_data)?;
    
    let config = PlotConfig {
        width: 50,
        height: 12,
        title: Some("Category Values".to_string()),
        ..Default::default()
    };
    
    let bar_plot = BarPlot;
    let result = bar_plot.render(&df, &config)?;
    println!("{}\n", result);

    // Example 3: Scatter plot with sine wave
    println!("Example 3: Scatter Plot - Sine Wave");
    let mut sine_data = String::from("x,y\n");
    for i in 0..20 {
        let x = i as f64 * 0.5;
        let y = (x * 2.0).sin();
        sine_data.push_str(&format!("{:.2},{:.3}\n", x, y));
    }
    
    let df = parser.parse_string(&sine_data)?;
    let config = PlotConfig {
        width: 70,
        height: 18,
        title: Some("Sine Wave".to_string()),
        xlabel: Some("Radians".to_string()),
        ylabel: Some("sin(2x)".to_string()),
        symbol: Some('•'),
        ..Default::default()
    };
    
    let scatter_plot = ScatterPlot;
    let result = scatter_plot.render(&df, &config)?;
    println!("{}\n", result);

    // Example 4: Canvas direct manipulation
    println!("Example 4: Direct Canvas Drawing");
    let mut canvas = Canvas::new(50, 15);
    canvas.set_ranges((0.0, 10.0), (-2.0, 2.0));
    
    // Draw a cosine wave
    for i in 0..100 {
        let x = i as f64 / 10.0;
        let y = (x * 2.0).cos();
        canvas.plot_point(x, y, '*');
    }
    
    canvas.draw_axis();
    println!("Cosine Wave (Direct Canvas):");
    println!("{}", canvas.render());

    // Example 5: Multi-column data (XYY format)
    println!("Example 5: Multi-Series Data");
    let mut multi_data = String::from("x,series1,series2\n");
    for i in 0..15 {
        let x = i as f64;
        let y1 = (x / 3.0).sin();
        let y2 = (x / 2.0).cos();
        multi_data.push_str(&format!("{},{:.3},{:.3}\n", x, y1, y2));
    }
    
    let df = parser.parse_string(&multi_data)?;
    let config = PlotConfig {
        width: 60,
        height: 16,
        title: Some("Multiple Series".to_string()),
        format: DataFormat::XYY,
        ..Default::default()
    };
    
    let line_plot = LinePlot;
    let result = line_plot.render(&df, &config)?;
    println!("{}\n", result);

    // Example 6: Different delimiters (TSV)
    println!("Example 6: Tab-Separated Values");
    let tsv_data = "time\ttemperature\n0\t20.0\n1\t21.5\n2\t23.0\n3\t22.8\n4\t21.2\n5\t19.5";
    let parser_tsv = FastParser::new('\t', true);
    let df = parser_tsv.parse_string(tsv_data)?;
    
    let config = PlotConfig {
        width: 55,
        height: 14,
        title: Some("Temperature Over Time".to_string()),
        xlabel: Some("Hours".to_string()),
        ylabel: Some("°C".to_string()),
        ..Default::default()
    };
    
    let line_plot = LinePlot;
    let result = line_plot.render(&df, &config)?;
    println!("{}\n", result);

    println!("All examples completed successfully!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_run() {
        // This test ensures all examples can be executed without panicking
        main().expect("Examples should run without errors");
    }
}