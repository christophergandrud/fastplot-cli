use crossterm::style::{Color, Stylize};

/// Centralized color parsing and handling utilities
pub struct ColorUtils;

impl ColorUtils {
    /// Parse a color string (hex or named) into a Color enum
    pub fn parse_color(color_str: &Option<String>) -> Option<Color> {
        color_str.as_ref().and_then(|s| Self::parse_color_string(s))
    }

    /// Parse a color string into a Color enum
    pub fn parse_color_string(color_str: &str) -> Option<Color> {
        // Try hex color first
        if let Some(color) = Self::parse_hex_color(color_str) {
            return Some(color);
        }

        // Fall back to named colors
        Self::parse_named_color(color_str)
    }

    /// Parse hex color (#RRGGBB format)
    pub fn parse_hex_color(color_str: &str) -> Option<Color> {
        if color_str.starts_with('#') && color_str.len() == 7 {
            if let Ok(hex_value) = u32::from_str_radix(&color_str[1..], 16) {
                let r = ((hex_value >> 16) & 0xFF) as u8;
                let g = ((hex_value >> 8) & 0xFF) as u8;
                let b = (hex_value & 0xFF) as u8;
                return Some(Color::Rgb { r, g, b });
            }
        }
        None
    }

    /// Parse named color
    pub fn parse_named_color(color_str: &str) -> Option<Color> {
        match color_str.to_lowercase().as_str() {
            "red" => Some(Color::Red),
            "green" => Some(Color::Green),
            "blue" => Some(Color::Blue),
            "yellow" => Some(Color::Yellow),
            "magenta" => Some(Color::Magenta),
            "cyan" => Some(Color::Cyan),
            "white" => Some(Color::White),
            "black" => Some(Color::Black),
            "dark_red" => Some(Color::DarkRed),
            "dark_green" => Some(Color::DarkGreen),
            "dark_blue" => Some(Color::DarkBlue),
            "dark_yellow" => Some(Color::DarkYellow),
            "dark_magenta" => Some(Color::DarkMagenta),
            "dark_cyan" => Some(Color::DarkCyan),
            "grey" | "gray" => Some(Color::Grey),
            "dark_grey" | "dark_gray" => Some(Color::DarkGrey),
            _ => None,
        }
    }

    /// Apply color styling to text
    pub fn apply_color(text: &str, color: Option<Color>) -> String {
        match color {
            Some(c) => format!("{}", text.with(c)),
            None => text.to_string(),
        }
    }

    /// Apply color styling to text using color string
    pub fn apply_color_string(text: &str, color_str: &str) -> String {
        if let Some(color) = Self::parse_color_string(color_str) {
            Self::apply_color(text, Some(color))
        } else {
            text.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_named_colors() {
        assert!(matches!(ColorUtils::parse_named_color("red"), Some(Color::Red)));
        assert!(matches!(ColorUtils::parse_named_color("RED"), Some(Color::Red)));
        assert!(matches!(ColorUtils::parse_named_color("blue"), Some(Color::Blue)));
        assert!(matches!(ColorUtils::parse_named_color("invalid"), None));
    }

    #[test]
    fn test_parse_hex_colors() {
        assert!(matches!(ColorUtils::parse_hex_color("#FF0000"), Some(Color::Rgb { r: 255, g: 0, b: 0 })));
        assert!(matches!(ColorUtils::parse_hex_color("#00FF00"), Some(Color::Rgb { r: 0, g: 255, b: 0 })));
        assert!(matches!(ColorUtils::parse_hex_color("#0000FF"), Some(Color::Rgb { r: 0, g: 0, b: 255 })));
        assert!(matches!(ColorUtils::parse_hex_color("FF0000"), None)); // No #
        assert!(matches!(ColorUtils::parse_hex_color("#FF00"), None)); // Too short
    }

    #[test]
    fn test_parse_color_option() {
        let red_option = Some("red".to_string());
        assert!(matches!(ColorUtils::parse_color(&red_option), Some(Color::Red)));
        
        let none_option: Option<String> = None;
        assert!(matches!(ColorUtils::parse_color(&none_option), None));
    }
}