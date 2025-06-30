use fastplot_cli::*;
use std::f64::consts::PI;

fn main() -> anyhow::Result<()> {
    println!("🚀 FastPlot-CLI Demo - All Plot Types\n");

    // Generate sample data for demonstrations
    let sample_data = generate_sample_data();
    
    demo_bar_chart(&sample_data)?;
    demo_line_plot(&sample_data)?;
    demo_scatter_plot(&sample_data)?;
    demo_histogram(&sample_data)?;
    demo_density_plot(&sample_data)?;
    demo_box_plot(&sample_data)?;
    
    println!("\n✅ All plot demos completed successfully!");
    println!("📖 See README.md for more examples and usage instructions.");
    
    Ok(())
}

fn generate_sample_data() -> SampleDataSets {
    // Generate various datasets for demonstration
    let bar_data = vec![10.0, 25.0, 15.0, 30.0, 20.0, 35.0, 18.0];
    
    let line_data: Vec<(f64, f64)> = (0..50)
        .map(|i| {
            let x = i as f64 * 0.2;
            let y = 5.0 * (x * 0.5).sin() + 2.0 * (x * 0.8).cos() + x * 0.1;
            (x, y)
        })
        .collect();

    let scatter_data: Vec<(f64, f64)> = (0..30)
        .map(|i| {
            let x = i as f64 / 3.0;
            let y = 2.0 + x * 0.8 + (rand::random::<f64>() - 0.5) * 2.0;
            (x, y)
        })
        .collect();

    let histogram_data: Vec<f64> = (0..200)
        .map(|_| {
            // Generate normal distribution-like data
            let u1: f64 = rand::random();
            let u2: f64 = rand::random();
            let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos();
            5.0 + 2.0 * z0 // mean=5, std=2
        })
        .collect();

    let box_data_group1 = vec![2.1, 2.5, 2.8, 3.2, 3.5, 3.8, 4.1, 4.5, 4.8, 5.2, 7.1]; // some outliers
    let box_data_group2 = vec![1.8, 2.2, 2.6, 2.9, 3.3, 3.7, 4.0, 4.4, 4.7, 5.1];
    let box_data_group3 = vec![3.1, 3.4, 3.8, 4.1, 4.5, 4.9, 5.2, 5.6, 5.9, 6.3, 8.2]; // some outliers

    SampleDataSets {
        bar_data,
        line_data,
        scatter_data,
        histogram_data,
        box_data_groups: vec![box_data_group1, box_data_group2, box_data_group3],
    }
}

fn demo_bar_chart(data: &SampleDataSets) -> anyhow::Result<()> {
    println!("📊 Bar Chart Demo");
    println!("================");

    // Create DataFrame for bar chart
    let series = Series {
        name: "Revenue".to_string(),
        data: data.bar_data.clone(),
    };
    let dataframe = DataFrame {
        columns: vec![series],
        headers: None,
    };

    let config = PlotConfig {
        width: 50,
        height: 15,
        title: Some("Quarterly Revenue".to_string()),
        xlabel: Some("Quarter".to_string()),
        ylabel: Some("Revenue ($K)".to_string()),
        delimiter: ',',
        has_header: false,
        format: DataFormat::XY,
        xlim: None,
        ylim: None,
        color: Some("blue".to_string()),
        symbol: Some('█'),
    };

    // Vertical bar chart
    let bar_chart = plot::BarChart::vertical();
    let output = bar_chart.render(&dataframe, &config)?;
    println!("{}", output);

    // Horizontal bar chart
    let config_horizontal = PlotConfig {
        color: Some("green".to_string()),
        title: Some("Quarterly Revenue (Horizontal)".to_string()),
        ..config
    };
    let bar_chart_h = plot::BarChart::horizontal();
    let output_h = bar_chart_h.render(&dataframe, &config_horizontal)?;
    println!("{}", output_h);

    Ok(())
}

fn demo_line_plot(data: &SampleDataSets) -> anyhow::Result<()> {
    println!("\n📈 Line Plot Demo");
    println!("=================");

    // Create DataFrame for line plot
    let x_series = Series {
        name: "Time".to_string(),
        data: data.line_data.iter().map(|(x, _)| *x).collect(),
    };
    let y_series = Series {
        name: "Signal".to_string(),
        data: data.line_data.iter().map(|(_, y)| *y).collect(),
    };
    let dataframe = DataFrame {
        columns: vec![x_series, y_series],
        headers: None,
    };

    let config = PlotConfig {
        width: 60,
        height: 20,
        title: Some("Signal Over Time".to_string()),
        xlabel: Some("Time (s)".to_string()),
        ylabel: Some("Amplitude".to_string()),
        delimiter: ',',
        has_header: false,
        format: DataFormat::XY,
        xlim: None,
        ylim: None,
        color: Some("red".to_string()),
        symbol: Some('●'),
    };

    let line_plot = plot::LinePlot::single();
    let output = line_plot.render(&dataframe, &config)?;
    println!("{}", output);

    Ok(())
}

fn demo_scatter_plot(data: &SampleDataSets) -> anyhow::Result<()> {
    println!("\n🎯 Scatter Plot Demo");
    println!("====================");

    // Create DataFrame for scatter plot
    let x_series = Series {
        name: "X".to_string(),
        data: data.scatter_data.iter().map(|(x, _)| *x).collect(),
    };
    let y_series = Series {
        name: "Y".to_string(),
        data: data.scatter_data.iter().map(|(_, y)| *y).collect(),
    };
    let dataframe = DataFrame {
        columns: vec![x_series, y_series],
        headers: None,
    };

    let config = PlotConfig {
        width: 50,
        height: 20,
        title: Some("X vs Y Relationship".to_string()),
        xlabel: Some("X Variable".to_string()),
        ylabel: Some("Y Variable".to_string()),
        delimiter: ',',
        has_header: false,
        format: DataFormat::XY,
        xlim: None,
        ylim: None,
        color: Some("magenta".to_string()),
        symbol: Some('●'),
    };

    let scatter_plot = plot::ScatterPlot::default();
    let output = scatter_plot.render(&dataframe, &config)?;
    println!("{}", output);

    Ok(())
}

fn demo_histogram(data: &SampleDataSets) -> anyhow::Result<()> {
    println!("\n📊 Histogram Demo");
    println!("=================");

    // Create DataFrame for histogram
    let series = Series {
        name: "Values".to_string(),
        data: data.histogram_data.clone(),
    };
    let dataframe = DataFrame {
        columns: vec![series],
        headers: None,
    };

    let config = PlotConfig {
        width: 55,
        height: 18,
        title: Some("Data Distribution".to_string()),
        xlabel: Some("Value".to_string()),
        ylabel: Some("Frequency".to_string()),
        delimiter: ',',
        has_header: false,
        format: DataFormat::XY,
        xlim: None,
        ylim: None,
        color: Some("cyan".to_string()),
        symbol: Some('█'),
    };

    let histogram = plot::Histogram::with_bins(15);
    let output = histogram.render(&dataframe, &config)?;
    println!("{}", output);

    Ok(())
}

fn demo_density_plot(data: &SampleDataSets) -> anyhow::Result<()> {
    println!("\n🌊 Density Plot Demo");
    println!("====================");

    // Create DataFrame for density plot
    let series = Series {
        name: "Values".to_string(),
        data: data.histogram_data.clone(),
    };
    let dataframe = DataFrame {
        columns: vec![series],
        headers: None,
    };

    let config = PlotConfig {
        width: 55,
        height: 18,
        title: Some("Probability Density".to_string()),
        xlabel: Some("Value".to_string()),
        ylabel: Some("Density".to_string()),
        delimiter: ',',
        has_header: false,
        format: DataFormat::XY,
        xlim: None,
        ylim: None,
        color: Some("yellow".to_string()),
        symbol: Some('●'),
    };

    let density_plot = plot::DensityPlot::auto_bandwidth()
        .with_kernel(plot::KernelType::Gaussian)
        .with_resolution(100);
    let output = density_plot.render(&dataframe, &config)?;
    println!("{}", output);

    Ok(())
}

fn demo_box_plot(data: &SampleDataSets) -> anyhow::Result<()> {
    println!("\n📦 Box Plot Demo");
    println!("================");

    // Create DataFrame with multiple groups for box plot
    let series1 = Series {
        name: "Group A".to_string(),
        data: data.box_data_groups[0].clone(),
    };
    let series2 = Series {
        name: "Group B".to_string(),
        data: data.box_data_groups[1].clone(),
    };
    let series3 = Series {
        name: "Group C".to_string(),
        data: data.box_data_groups[2].clone(),
    };
    let dataframe = DataFrame {
        columns: vec![series1, series2, series3],
        headers: None,
    };

    let config = PlotConfig {
        width: 50,
        height: 20,
        title: Some("Group Comparison".to_string()),
        xlabel: Some("Groups".to_string()),
        ylabel: Some("Values".to_string()),
        delimiter: ',',
        has_header: false,
        format: DataFormat::XYY,
        xlim: None,
        ylim: None,
        color: Some("blue".to_string()),
        symbol: None,
    };

    let box_plot = plot::BoxPlot::vertical();
    let output = box_plot.render(&dataframe, &config)?;
    println!("{}", output);

    Ok(())
}

struct SampleDataSets {
    bar_data: Vec<f64>,
    line_data: Vec<(f64, f64)>,
    scatter_data: Vec<(f64, f64)>,
    histogram_data: Vec<f64>,
    box_data_groups: Vec<Vec<f64>>,
}

use rand; // Note: This would need to be added to dependencies for a real demo