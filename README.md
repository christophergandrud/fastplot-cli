# fastplot-cli

A fast terminal plotting tool written in Rust that supports both scatter plots and line plots with rich customization options.

## Quick Start

### Install
```bash
# Build and install locally
cargo build --release
cargo install --path .
```

## Plot Types

### Scatter Plots
Create scatter plots with customizable symbols and colors.

```bash
# Basic scatter plot
fastplot scatter data.csv --title "My Scatter Plot"

# Custom point character and color
fastplot scatter data.csv --point-char "+" --color red

# With hex color
fastplot scatter data.csv --point-char "★" --color "#ff6b35"
```

### Line Plots
Create connected line plots with multiple style options.

```bash
# Basic line plot (default Unicode style)
fastplot line data.csv --title "My Line Plot"

# ASCII style for compatibility
fastplot line data.csv --style ascii

# Smooth Unicode style
fastplot line data.csv --style smooth

# Dashed lines
fastplot line data.csv --style dashed

# Lines only (no points)
fastplot line data.csv --lines-only

# Points only (no lines)  
fastplot line data.csv --points-only

# Custom characters
fastplot line data.csv --point-char "◆" --line-char "─"

# With colors
fastplot line data.csv --color blue --title "Colored Line Plot"
```

## Examples with Sample Data

### Create Sample Data
```bash
# Create sample quadratic data
echo "x,y
0,0
1,1
2,4
3,9
4,16
5,25" > quadratic.csv

# Create sample sine wave data
echo "x,y
0,0
0.5,0.479
1,0.841
1.5,0.997
2,0.909
2.5,0.598
3,0.141
3.5,-0.351
4,-0.757
4.5,-0.977
5,-0.959
5.5,-0.705
6,-0.279" > sine.csv
```

### Run Examples
```bash
# Scatter plot examples
fastplot scatter quadratic.csv --title "Quadratic Function" --point-char "●" --color green
fastplot scatter sine.csv --title "Sine Wave" --point-char "○" --color cyan

# Line plot examples  
fastplot line quadratic.csv --title "Quadratic Growth" --color red
fastplot line sine.csv --title "Sine Wave" --style smooth --color blue
fastplot line quadratic.csv --title "Points Only" --points-only --color purple
fastplot line sine.csv --title "Lines Only" --lines-only --style dashed
```

## CSV Format
CSV files should have two columns with headers:
```csv
x,y
-5,25
-3,9
-1,1
0,0
1,1
3,9
5,25
```

## Example Outputs

### Scatter Plot
```
Quadratic Function

y
       │
       │                                                                    ●
 20    ┤
       │                                                              ●
       │
 15    ┤
       │                                                      ●
       │
 10    ┤
       │                                              ●
       │
  5    ┤                                      ●
       │
       │              ●
  0    ┤      ●
       │
       └──────┬───────────────┬──────────────┬───────────────┬───────────────┬──────
              0               1              2               3               4

x
```

### Line Plot
```
Sine Wave

y
       │
       │          ·●·
  1    ┤        ··   ··
       │      ··       ··
       │    ··           ··
       │  ··               ··
  0    ┤●·                   ·●
       │                       ··
       │                         ··
       │                           ··
 -1    ┤                             ·●·
       │
       └──────┬───────────────┬──────────────┬───────────────┬───────────────┬──────
              0               1              2               3               4

x
```

## Features

- **Multiple Plot Types**: Scatter plots and line plots
- **Rich Styling**: Unicode and ASCII character sets, custom symbols
- **Color Support**: Named colors (red, blue, green, etc.) and hex codes (#ff6b35)
- **Flexible Display**: Points-only, lines-only, or combined modes
- **Automatic Scaling**: Smart axis scaling with proper tick generation
- **Fast Performance**: Efficient Rust implementation with Bresenham line algorithm
- **Layered Rendering**: Proper z-ordering (lines below points, labels on top)

## Command Reference

### Scatter Plots
```bash
fastplot scatter <file> [OPTIONS]

OPTIONS:
    -t, --title <TITLE>          Plot title [default: "Scatter Plot"]
    -p, --point-char <CHAR>      Point character [default: "●"]
    -c, --color <COLOR>          Color (named or hex code)
```

### Line Plots  
```bash
fastplot line <file> [OPTIONS]

OPTIONS:
    -t, --title <TITLE>       Plot title [default: "Line Plot"]
    -S, --style <STYLE>       Line style: default, ascii, smooth, dashed [default: "default"]
        --points-only         Show only points (no lines)
        --lines-only          Show only lines (no points)
    -p, --point-char <CHAR>   Custom point character
    -l, --line-char <CHAR>    Custom line character
    -c, --color <COLOR>       Color (named or hex code)
```

### Available Colors
- **Named**: red, green, blue, yellow, magenta, cyan, white, black
- **Bright**: bright_red, bright_green, bright_blue, etc.
- **Hex codes**: #ff6b35, #4ecdc4, #45b7d1, etc.