# fastplot-cli

A fast terminal plotting tool written in Rust.

## Quick Start

### Install
```bash
# Build and install locally
cargo build --release
cargo install --path .
```

### Usage
```bash
# Create a scatter plot from CSV data
fastplot scatter data.csv --title "My Plot"

# Use custom symbol
fastplot scatter data.csv --symbol "+"

# Or run directly with cargo
cargo run -- scatter data.csv --title "My Plot"
```

### CSV Format
CSV file should have two columns with headers:
```csv
x_position,y_value
-5,5
0,0
5,-5
```

### Example Output
```
Test Scatter Plot

y_value
 10 ┼
    │
  5 ┼     ●
    │
  0 ┼           ●
    │
 -5 ┼                 ●
    │
-10 ┼─────┼─────┼─────┼─────┼
  -10    -5     0     5    10
           x_position
```

## Features

- Fast CSV parsing
- Automatic axis scaling (-10 to +10 range)
- Customizable plot titles and symbols
- Modular design for extensibility