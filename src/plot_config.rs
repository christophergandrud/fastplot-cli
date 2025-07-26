use anyhow::Result;
use crate::line_style::LineStyle;

/// Unified configuration for all plot types - implements "deep module" design
/// by hiding complexity behind a simple interface
#[derive(Debug, Clone)]
pub struct PlotConfig {
    /// Data source: CSV file path or function expression
    pub source: String,
    /// Plot title to display
    pub title: String,
    /// Optional color (named color or hex code)
    pub color: Option<String>,
    /// Optional range for function plots (e.g., "-5:5")
    pub range: Option<String>,
    /// Number of points to evaluate for functions
    pub points: usize,
}

impl PlotConfig {
    /// Create a new plot configuration with sensible defaults
    pub fn new(source: String) -> Self {
        Self {
            source,
            title: "Plot".to_string(),
            color: None,
            range: None,
            points: 200,
        }
    }

    /// Builder pattern for setting title
    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// Builder pattern for setting color
    pub fn with_color(mut self, color: Option<String>) -> Self {
        self.color = color;
        self
    }

    /// Builder pattern for setting range
    pub fn with_range(mut self, range: Option<String>) -> Self {
        self.range = range;
        self
    }

    /// Builder pattern for setting points
    pub fn with_points(mut self, points: usize) -> Self {
        self.points = points;
        self
    }
}

/// Plot-specific parameters separated by type
#[derive(Debug, Clone)]
pub enum PlotType {
    Scatter {
        point_char: char,
    },
    Line {
        style: LineStyle,
        points_only: bool,
        lines_only: bool,
        point_char: Option<char>,
        line_char: Option<char>,
    },
    Bar {
        bar_char: char,
        bar_width: usize,
        category_order: Option<Vec<String>>,
    },
}

impl PlotType {
    /// Create a scatter plot type with default settings
    pub fn scatter() -> Self {
        Self::Scatter {
            point_char: '●',
        }
    }

    /// Create a line plot type with default settings
    pub fn line() -> Self {
        Self::Line {
            style: LineStyle::default(),
            points_only: false,
            lines_only: false,
            point_char: None,
            line_char: None,
        }
    }

    /// Create a bar plot type with default settings
    pub fn bar() -> Self {
        Self::Bar {
            bar_char: '█',
            bar_width: 1,
            category_order: None,
        }
    }

    /// Builder method for scatter plot point character
    pub fn with_point_char(self, point_char: char) -> Self {
        match self {
            Self::Scatter { .. } => Self::Scatter { point_char },
            _ => self,
        }
    }

    /// Builder method for line plot style
    pub fn with_line_style(self, style: LineStyle) -> Self {
        match self {
            Self::Line { points_only, lines_only, point_char, line_char, .. } => {
                Self::Line { style, points_only, lines_only, point_char, line_char }
            }
            _ => self,
        }
    }

    /// Builder method for line plot points only
    pub fn with_points_only(self, points_only: bool) -> Self {
        match self {
            Self::Line { style, lines_only, point_char, line_char, .. } => {
                Self::Line { style, points_only, lines_only, point_char, line_char }
            }
            _ => self,
        }
    }

    /// Builder method for line plot lines only
    pub fn with_lines_only(self, lines_only: bool) -> Self {
        match self {
            Self::Line { style, points_only, point_char, line_char, .. } => {
                Self::Line { style, points_only, lines_only, point_char, line_char }
            }
            _ => self,
        }
    }

    /// Builder method for line plot point character
    pub fn with_line_point_char(self, point_char: Option<char>) -> Self {
        match self {
            Self::Line { style, points_only, lines_only, line_char, .. } => {
                Self::Line { style, points_only, lines_only, point_char, line_char }
            }
            _ => self,
        }
    }

    /// Builder method for line plot line character
    pub fn with_line_char(self, line_char: Option<char>) -> Self {
        match self {
            Self::Line { style, points_only, lines_only, point_char, .. } => {
                Self::Line { style, points_only, lines_only, point_char, line_char }
            }
            _ => self,
        }
    }

    /// Builder method for bar plot character
    pub fn with_bar_char(self, bar_char: char) -> Self {
        match self {
            Self::Bar { bar_width, category_order, .. } => {
                Self::Bar { bar_char, bar_width, category_order }
            }
            _ => self,
        }
    }

    /// Builder method for bar plot width
    pub fn with_bar_width(self, bar_width: usize) -> Self {
        match self {
            Self::Bar { bar_char, category_order, .. } => {
                Self::Bar { bar_char, bar_width, category_order }
            }
            _ => self,
        }
    }

    /// Builder method for bar plot category order
    pub fn with_category_order(self, category_order: Option<Vec<String>>) -> Self {
        match self {
            Self::Bar { bar_char, bar_width, .. } => {
                Self::Bar { bar_char, bar_width, category_order }
            }
            _ => self,
        }
    }
}

/// Unified command structure that hides parameter complexity
/// This is the "deep module" that provides a simple interface for complex operations
#[derive(Debug, Clone)]
pub struct PlotCommand {
    config: PlotConfig,
    plot_type: PlotType,
}

impl PlotCommand {
    /// Create a new plot command
    pub fn new(config: PlotConfig, plot_type: PlotType) -> Self {
        Self { config, plot_type }
    }

    /// Get the plot configuration
    pub fn config(&self) -> &PlotConfig {
        &self.config
    }

    /// Get the plot type
    pub fn plot_type(&self) -> &PlotType {
        &self.plot_type
    }

    /// Execute the plot command - single point of execution logic
    /// This method encapsulates all the complexity of different plot types
    pub fn execute(&self) -> Result<String> {
        use crate::{data, scatter, line_plot, bar_chart};

        // Parse data source using unified configuration
        let mut dataset = data::parse_data_source(
            &self.config.source,
            self.config.range.as_deref(),
            Some(self.config.points),
        )?;

        // Execute based on plot type, but with consistent interface
        match &self.plot_type {
            PlotType::Scatter { point_char } => {
                let output = scatter::render_scatter_plot(
                    &dataset,
                    &self.config.title,
                    *point_char,
                    self.config.color.as_deref(),
                );
                Ok(output)
            }
            PlotType::Line {
                style,
                points_only,
                lines_only,
                point_char,
                line_char,
            } => {
                let mut line_style = style.clone();
                if *points_only {
                    line_style.show_lines = false;
                }
                if *lines_only {
                    line_style.show_points = false;
                }
                if let Some(pc) = point_char {
                    line_style.point_char = *pc;
                }
                if let Some(lc) = line_char {
                    line_style.line_char = *lc;
                }

                let output = line_plot::render_line_plot(
                    &dataset,
                    &self.config.title,
                    line_style,
                    self.config.color.as_deref(),
                );
                Ok(output)
            }
            PlotType::Bar {
                bar_char,
                bar_width,
                category_order,
            } => {
                // Apply custom category ordering if specified
                if let Some(order) = category_order {
                    if dataset.is_categorical {
                        dataset = data::reorder_categories(dataset, order.clone())?;
                    }
                }

                let output = bar_chart::render_bar_chart(
                    &dataset,
                    &self.config.title,
                    *bar_char,
                    *bar_width,
                    self.config.color.as_deref(),
                );
                Ok(output)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plot_config_creation() {
        let config = PlotConfig::new("data.csv".to_string())
            .with_title("Test Plot".to_string())
            .with_color(Some("red".to_string()))
            .with_points(100);

        assert_eq!(config.source, "data.csv");
        assert_eq!(config.title, "Test Plot");
        assert_eq!(config.color, Some("red".to_string()));
        assert_eq!(config.points, 100);
    }

    #[test]
    fn test_plot_type_builders() {
        let scatter = PlotType::scatter().with_point_char('*');
        match scatter {
            PlotType::Scatter { point_char } => assert_eq!(point_char, '*'),
            _ => panic!("Expected scatter plot type"),
        }

        let line = PlotType::line().with_points_only(true);
        match line {
            PlotType::Line { points_only, .. } => assert!(points_only),
            _ => panic!("Expected line plot type"),
        }

        let bar = PlotType::bar().with_bar_char('▓').with_bar_width(2);
        match bar {
            PlotType::Bar { bar_char, bar_width, .. } => {
                assert_eq!(bar_char, '▓');
                assert_eq!(bar_width, 2);
            }
            _ => panic!("Expected bar plot type"),
        }
    }

    #[test]
    fn test_plot_command_creation() {
        let config = PlotConfig::new("test.csv".to_string())
            .with_title("Test".to_string());
        let plot_type = PlotType::scatter();
        let command = PlotCommand::new(config, plot_type);

        assert_eq!(command.config().source, "test.csv");
        assert_eq!(command.config().title, "Test");
    }
}