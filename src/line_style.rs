#[derive(Debug, Clone)]
pub struct LineStyle {
    pub point_char: char,
    pub line_char: char,
    pub show_points: bool,
    pub show_lines: bool,
}

impl Default for LineStyle {
    fn default() -> Self {
        Self {
            point_char: '●',
            line_char: '·',
            show_points: true,
            show_lines: true,
        }
    }
}

impl LineStyle {
    pub fn with_ascii() -> Self {
        Self {
            point_char: 'o',
            line_char: '.',
            ..Default::default()
        }
    }

    pub fn with_unicode_smooth() -> Self {
        Self {
            point_char: '◆',
            line_char: '─',
            ..Default::default()
        }
    }

    pub fn with_dashed() -> Self {
        Self {
            point_char: '◆',
            line_char: '╌',
            ..Default::default()
        }
    }

}