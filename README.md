# fastplot-cli

A fast terminal plotting tool written in Rust for both data files and mathematical functions.

## Quick Start

```bash
# Build and install
cargo build --release
cargo install --path .

# Plot a CSV file
fastplot line data.csv --title "My Data"

# Plot a mathematical function  
fastplot line "function:sin(x)" --title "Sine Wave"
```

## Features

- **Data Files**: Plot CSV files with scatter, line, and bar plots
- **Categorical Data**: Bar charts with categorical x-axis labels (auto-detected)
- **Mathematical Functions**: Plot expressions like `sin(x)`, `x^2`, `exp(-x)*cos(5*x)`
- **Rich Styling**: Unicode/ASCII styles, custom colors, point characters
- **Smart Ranges**: Automatic scaling or custom ranges (`--range="-5:5"`)
- **Fast Performance**: Efficient Rust implementation

## Examples

### CSV Data Plots
```bash
# Line plots
fastplot line data.csv --title "My Line Plot"
fastplot line data.csv --style smooth --color blue
fastplot line data.csv --points-only --color red

# Scatter plots
fastplot scatter data.csv --point-char "●" --color green
fastplot scatter data.csv --point-char "+" --color "#ff6b35"

# Bar charts (automatically detects categorical vs numeric data)
fastplot bar test-data/categorical_regions.csv --title "Regional Sales"
fastplot bar test-data/categorical_quarters.csv --title "Quarterly Sales" --category-order "Q4,Q3,Q2,Q1"
fastplot bar test-data/numeric_simple.csv --title "Numeric Data"
```

### Mathematical Function Plots
```bash
# Basic functions
fastplot line "function:x^2" --title "Quadratic"
fastplot line "function:sin(x)" --title "Sine Wave"
fastplot line "function:exp(x)" --range="-2:2" --title "Exponential"

# Complex expressions
fastplot line "function:sin(x^2)" --range="-3:3"
fastplot line "function:exp(-x)*cos(5*x)" --range="0:3"
fastplot scatter "function:sqrt(x)" --range="0:25"

# With styling
fastplot line "function:cos(x)" --style smooth --color blue
fastplot line "function:ln(x)" --range="0.1:10" --color purple
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

## CSV Format

CSV files should have two columns with headers:

**Numeric data:**
```csv
x,y
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

The tool automatically detects whether the x-axis contains categorical (string) or numeric data.