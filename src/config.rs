use crate::data::{PlotConfig, DataFormat};
use crate::cli::Args;

impl From<&Args> for PlotConfig {
    fn from(args: &Args) -> Self {
        Self {
            width: args.width,
            height: args.height,
            title: args.title.clone(),
            xlabel: args.xlabel.clone(),
            ylabel: args.ylabel.clone(),
            delimiter: args.delimiter,
            has_header: args.has_header,
            format: args.format.clone(),
            xlim: args.xlim,
            ylim: args.ylim,
            color: args.color.clone(),
            symbol: args.symbol,
        }
    }
}