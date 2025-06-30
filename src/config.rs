use crate::data::PlotConfig;
use crate::cli::PlotOptions;

impl From<&PlotOptions> for PlotConfig {
    fn from(options: &PlotOptions) -> Self {
        Self {
            width: options.width,
            height: options.height,
            title: options.title.clone(),
            xlabel: options.xlabel.clone(),
            ylabel: options.ylabel.clone(),
            delimiter: options.delimiter,
            has_header: options.has_header,
            format: options.format.clone(),
            xlim: options.xlim,
            ylim: options.ylim,
            color: options.color.clone(),
            symbol: options.symbol,
        }
    }
}