# fastplot-cli

A fast terminal plotting tool written in Rust for both data files and mathematical functions.

## Prerequisites

- **Rust**: Version 1.70.0 or later ([Install Rust](https://rustup.rs/))
- **Terminal**: Unicode support recommended for best visual output
- **Platform**: Works on Linux, macOS, and Windows

## Quick Start

```bash
# Run directly with cargo
cargo run --release --bin fastplot -- line test-data/sine.csv --title "Sine Wave"

# Plot a mathematical function
cargo run --release --bin fastplot -- line "function:sin(x)" --title "Sine Wave"

# Optional: Install system-wide for easier access
cargo install --path .
# Both fastplot and fplot binaries are installed for convenience
fastplot line test-data/sine.csv --title "Sine Wave"
# or use the shorter alias:
fplot line test-data/sine.csv --title "Sine Wave"
```

## Features

- **Data Files**: Plot CSV files with scatter, line, and bar plots
- **Categorical Data**: Bar charts with categorical x-axis labels (auto-detected)
- **Mathematical Functions**: Plot expressions like `sin(x)`, `x^2`, `exp(-x)*cos(5*x)`
- **Rich Styling**: Unicode/ASCII styles, custom colors, point characters
- **Smart Ranges**: Automatic scaling or custom ranges (`--range="-5:5"`)
- **Fast Performance**: Efficient Rust implementation

## Examples

### Beginner: Basic Plots

**Simple CSV Data Plots**
```bash
# Basic line plot with default settings
fastplot line data.csv

# Basic scatter plot
fastplot scatter data.csv

# Basic bar chart (auto-detects categorical data)
fastplot bar test-data/categorical_regions.csv
```

**Simple Function Plots**
```bash
# Plot basic mathematical functions
fastplot line "function:x^2"
fastplot line "function:sin(x)"
fastplot scatter "function:sqrt(x)" --range="0:25"
```

### Intermediate: Styling and Customization

**Custom Styling**
```bash
# Add titles and colors
fastplot line data.csv --title "My Line Plot" --color blue
fastplot scatter data.csv --point-char "●" --color green
fastplot bar test-data/categorical_regions.csv --title "Regional Sales" --color red

# Different line styles
fastplot line data.csv --style smooth --color blue
fastplot line data.csv --points-only --color red
```

**Custom Ranges and Characters**
```bash
# Specify custom ranges for functions
fastplot line "function:exp(x)" --range="-2:2" --title "Exponential"
fastplot line "function:ln(x)" --range="0.1:10" --color purple

# Custom point and bar characters
fastplot scatter data.csv --point-char "+" --color "#ff6b35"
fastplot bar data.csv --bar-char "▓"
```

### Advanced: Complex Functions and Features

**Complex Mathematical Expressions**
```bash
# Multi-function expressions
fastplot line "function:sin(x^2)" --range="-3:3"
fastplot line "function:exp(-x)*cos(5*x)" --range="0:3"
fastplot line "function:sin(x) + cos(x*2)" --range="-5:5"

# Combine styling with complex functions
fastplot line "function:cos(x)" --style smooth --color blue --title "Smooth Cosine"
```

**Advanced Bar Chart Features**
```bash
# Custom category ordering
fastplot bar test-data/categorical_quarters.csv --title "Quarterly Sales" --category-order "Q4,Q3,Q2,Q1"

# Mixed data types (numeric treated as categorical)
fastplot bar test-data/numeric_simple.csv --title "Numeric Data"
```

**Hex Colors and Precision Control**
```bash
# Use hex colors for precise color control
fastplot scatter data.csv --color "#ff6b35" --title "Custom Orange"
fastplot line "function:sin(x)" --color "#1e90ff" --points=500
```

## Sample Data

The `test-data/` directory contains sample CSV files for testing:

```bash
# Categorical data
fastplot bar test-data/categorical_regions.csv --title "Regional Sales"
fastplot bar test-data/categorical_quarters.csv --title "Quarterly Data"

# Numeric data  
fastplot line test-data/sine.csv --title "Sine Wave"
fastplot line test-data/quadratic.csv --title "Quadratic"
fastplot bar test-data/numeric_simple.csv --title "Simple Bar Chart"
```

Or create your own test CSV file:
```bash
echo "x,y
0,0
1,1
2,4
3,9
4,16" > my_data.csv

fastplot line my_data.csv --title "My Data"
```

## Example Output

```
Sine Wave

f(x) = sin(x)
          │
   1      ┤                     ●●●●               ●●●
          │                    ●●  ●              ●●  ●
          │      ●            ●●    ●            ●     ●
 0.5      ┤      ●            ●     ●            ●      ●
          │       ●           ●      ●           ●      ●
   0      ┤        ●        ●         ●        ●●        ●
          │        ●        ●         ●        ●         ●
          │         ●      ●●          ●       ●          ●
-0.5      ┤         ●      ●           ●      ●            ●
          │          ●    ●●            ●     ●            ●
  -1      ┤           ●●●●                ●●●               
          │
          └──────┬──────────────┬───────────────┬──────────────┬──────
                -10            -5               0              5
```

## Supported Functions

**Trigonometric**: `sin(x)`, `cos(x)`, `tan(x)`, `asin(x)`, `acos(x)`, `atan(x)`  
**Exponential**: `exp(x)`, `exp2(x)`  
**Logarithmic**: `ln(x)`, `log(x)`, `log2(x)`, `log10(x)`  
**Other**: `sqrt(x)`, `abs(x)`, `pow(x,y)`  
**Constants**: `pi`, `e`  
**Operations**: `+`, `-`, `*`, `/`, `^` (exponentiation)

## Command Options

### Common Options
```bash
-t, --title <TITLE>       Plot title
-c, --color <COLOR>       Color (red, blue, #ff6b35, etc.)
-r, --range <RANGE>       X range as min:max (e.g., "-5:5")
```

### Line Plot Options
```bash
-S, --style <STYLE>       Line style: default, ascii, smooth, dashed
    --points-only         Show only points (no lines)
    --lines-only          Show only lines (no points)
-p, --point-char <CHAR>   Custom point character
-l, --line-char <CHAR>    Custom line character
```

### Scatter Plot Options
```bash
-p, --point-char <CHAR>   Point character [default: "●"]
```

### Bar Chart Options
```bash
-b, --bar-char <CHAR>     Bar character [default: "█"]
-w, --bar-width <NUM>     Bar width in characters [default: 1]
    --category-order <LIST>  Custom category order as "Q1,Q2,Q3,Q4"
```

### Function Plot Options  
```bash
    --points <NUM>        Number of evaluation points [default: 200]
```

## Color Reference

All plot types support the `--color` option with the following values:

### Named Colors

**Standard Colors:**
- `red`, `green`, `blue`, `yellow`
- `magenta` (also `purple`), `cyan`
- `white`, `black`

**Bright Colors:**
- `bright_red`, `bright_green`, `bright_blue`, `bright_yellow`
- `bright_magenta` (also `bright_purple`), `bright_cyan`

### Hex Colors
- Use format: `#RRGGBB` (e.g., `#ff6b35`, `#1e90ff`, `#32cd32`)
- All standard hex color codes are supported

### Examples
```bash
# Named colors
fastplot line data.csv --color red
fastplot scatter data.csv --color bright_blue
fastplot bar data.csv --color purple

# Hex colors
fastplot line data.csv --color "#ff6b35"
fastplot scatter data.csv --color "#1e90ff"
fastplot bar data.csv --color "#32cd32"
```

## CSV Format

### File Structure
- **Required**: Two columns (x-axis and y-axis data)
- **Headers**: Column headers are optional but recommended
- **Separator**: Use commas (`,`) to separate columns
- **Data Types**: Automatic detection between numeric and categorical data

### Data Type Detection
The tool automatically determines data types:
- **Numeric**: If the first column contains only numbers, treats as numeric data
- **Categorical**: If any non-numeric values are found, treats the entire first column as categories
- **Mixed Data**: If numeric and categorical values are mixed, categorical takes precedence

### Examples

**Numeric data with headers:**
```csv
x,y
-2,4
-1,1
0,0
1,1
2,4
```

**Numeric data without headers:**
```csv
-2,4
-1,1
0,0
1,1
2,4
```

**Categorical data:**
```csv
Category,Sales
Q1,120
Q2,85
Q3,95
Q4,140
```

### Supported Formats
- **Whitespace**: Spaces around values are automatically trimmed
- **Numbers**: Integers and floating-point numbers (e.g., `3.14`, `-2.5`, `1e-3`)
- **Categories**: Text strings, numbers as strings (e.g., "2023", "Group A")
- **Empty Values**: Rows with empty cells are skipped

## Troubleshooting

### Common Issues

**"Function evaluation failed for all points in range"**
- Check that your function syntax is correct (e.g., `"function:sin(x)"`)
- Verify function names match supported functions (see Supported Functions section)
- Try a different range with `--range="-10:10"`

**"No such file or directory"**
- Ensure the CSV file path is correct
- Use absolute paths if having issues with relative paths
- Check that the file has proper CSV format with comma separators

**Plot appears garbled or uses ASCII characters**
- Your terminal may not support Unicode properly
- Try using `--style ascii` for ASCII-only output
- Ensure your terminal encoding is set to UTF-8

**Colors not displaying correctly**
- Some terminals have limited color support
- Try named colors like `--color red` instead of hex codes
- Check your terminal's color capabilities

**Installation Issues**
- Ensure Rust 1.70.0+ is installed: `rustc --version`
- Try `cargo clean` and rebuild if encountering build errors
- On older systems, you may need to update your Rust toolchain: `rustup update`

### Getting Help

If you encounter other issues:
1. Check that your input data follows the CSV format examples
2. Try the examples in this README to verify basic functionality
3. Use `fastplot --help` or `fplot --help` for command usage
4. Use `fastplot <command> --help` for specific command options