use colored::{Colorize, control};

/// Apply color to a character using a standardized color system.
/// This is the single source of truth for color application across all plot types.
pub fn apply_color(ch: char, color_str: &str) -> Option<String> {
    // Force color output even in non-TTY environments
    control::set_override(true);
    let ch_str = ch.to_string();
    
    // Handle hex colors (#RRGGBB)
    if color_str.starts_with('#') && color_str.len() == 7 {
        if let Ok(r) = u8::from_str_radix(&color_str[1..3], 16) {
            if let Ok(g) = u8::from_str_radix(&color_str[3..5], 16) {
                if let Ok(b) = u8::from_str_radix(&color_str[5..7], 16) {
                    return Some(ch_str.truecolor(r, g, b).to_string());
                }
            }
        }
    }
    
    // Handle named colors
    match color_str.to_lowercase().as_str() {
        "red" => Some(ch_str.red().to_string()),
        "green" => Some(ch_str.green().to_string()),
        "blue" => Some(ch_str.blue().to_string()),
        "yellow" => Some(ch_str.yellow().to_string()),
        "magenta" | "purple" => Some(ch_str.magenta().to_string()),
        "cyan" => Some(ch_str.cyan().to_string()),
        "white" => Some(ch_str.white().to_string()),
        "black" => Some(ch_str.black().to_string()),
        "bright_red" => Some(ch_str.bright_red().to_string()),
        "bright_green" => Some(ch_str.bright_green().to_string()),
        "bright_blue" => Some(ch_str.bright_blue().to_string()),
        "bright_yellow" => Some(ch_str.bright_yellow().to_string()),
        "bright_magenta" | "bright_purple" => Some(ch_str.bright_magenta().to_string()),
        "bright_cyan" => Some(ch_str.bright_cyan().to_string()),
        _ => None,
    }
}