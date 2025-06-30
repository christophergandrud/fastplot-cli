use fastplot_cli::*;

fn main() -> anyhow::Result<()> {
    println!("🚀 FastPlot-CLI Simple Demo\n");

    // Demo 1: Bar Chart
    demo_bar_chart()?;
    
    // Demo 2: Line Plot
    demo_line_plot()?;
    
    // Demo 3: Scatter Plot
    demo_scatter_plot()?;

    println!("\n✅ Simple demos completed!");
    println!("📖 Check README.md for more advanced examples.");
    
    Ok(())
}

fn demo_bar_chart() -> anyhow::Result<()> {
    println!("📊 Bar Chart Example");
    println!("====================");

    // Simple bar data - quarterly sales
    let sales_data = vec![45.2, 52.8, 38.1, 61.3, 55.7];
    
    let series = Series {
        name: "Sales".to_string(),
        data: sales_data,
    };
    
    let dataframe = DataFrame {
        columns: vec![series],
        headers: None,
    };

    let config = PlotConfig {
        width: 50,
        height: 15,
        title: Some("Quarterly Sales ($K)".to_string()),
        xlabel: Some("Quarter".to_string()),
        ylabel: Some("Sales".to_string()),
        delimiter: ',',
        has_header: false,
        format: DataFormat::XY,
        xlim: None,
        ylim: None,
        color: Some("blue".to_string()),
        symbol: Some('█'),
    };

    let bar_chart = BarChart::vertical();
    let output = bar_chart.render(&dataframe, &config)?;
    println!("{}", output);

    Ok(())
}

fn demo_line_plot() -> anyhow::Result<()> {
    println!("\n📈 Line Plot Example");
    println!("====================");

    // Simple time series data
    let time_data = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let values = vec![2.1, 3.5, 2.8, 4.2, 3.9, 5.1, 4.7, 5.8, 5.3, 6.2];
    
    let x_series = Series {
        name: "Time".to_string(),
        data: time_data,
    };
    
    let y_series = Series {
        name: "Temperature".to_string(),
        data: values,
    };
    
    let dataframe = DataFrame {
        columns: vec![x_series, y_series],
        headers: None,
    };

    let config = PlotConfig {
        width: 55,
        height: 18,
        title: Some("Temperature Over Time".to_string()),
        xlabel: Some("Hours".to_string()),
        ylabel: Some("°C".to_string()),
        delimiter: ',',
        has_header: false,
        format: DataFormat::XY,
        xlim: None,
        ylim: None,
        color: Some("red".to_string()),
        symbol: Some('●'),
    };

    let line_plot = LinePlot::single();
    let output = line_plot.render(&dataframe, &config)?;
    println!("{}", output);

    Ok(())
}

fn demo_scatter_plot() -> anyhow::Result<()> {
    println!("\n🎯 Scatter Plot Example");
    println!("=======================");

    // Simple X-Y relationship data
    let x_data = vec![1.2, 2.3, 3.1, 4.5, 5.2, 6.1, 7.3, 8.1, 9.2, 10.1];
    let y_data = vec![2.8, 4.1, 3.9, 6.2, 5.8, 7.1, 8.3, 7.9, 9.4, 10.2];
    
    let x_series = Series {
        name: "Input".to_string(),
        data: x_data,
    };
    
    let y_series = Series {
        name: "Output".to_string(),
        data: y_data,
    };
    
    let dataframe = DataFrame {
        columns: vec![x_series, y_series],
        headers: None,
    };

    let config = PlotConfig {
        width: 45,
        height: 16,
        title: Some("Input vs Output".to_string()),
        xlabel: Some("Input Value".to_string()),
        ylabel: Some("Output Value".to_string()),
        delimiter: ',',
        has_header: false,
        format: DataFormat::XY,
        xlim: None,
        ylim: None,
        color: Some("green".to_string()),
        symbol: Some('●'),
    };

    let scatter_plot = ScatterPlot::default();
    let output = scatter_plot.render(&dataframe, &config)?;
    println!("{}", output);

    Ok(())
}