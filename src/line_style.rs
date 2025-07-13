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
    pub fn points_only() -> Self {
        Self {
            show_lines: false,
            ..Default::default()
        }
    }

    pub fn lines_only() -> Self {
        Self {
            show_points: false,
            ..Default::default()
        }
    }

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

    pub fn with_chars(point_char: char, line_char: char) -> Self {
        Self {
            point_char,
            line_char,
            ..Default::default()
        }
    }
}

pub struct LineCharSets;

impl LineCharSets {
    pub const BASIC: (char, char) = ('o', '.');
    pub const UNICODE_DOTS: (char, char) = ('●', '·');
    pub const UNICODE_SMOOTH: (char, char) = ('◆', '─');
    pub const UNICODE_DASHED: (char, char) = ('◆', '╌');
    pub const STARS: (char, char) = ('★', '*');
    pub const TRIANGLES: (char, char) = ('▲', '·');
    pub const SQUARES: (char, char) = ('■', '·');
    pub const GRAPH_PAPER: (char, char) = ('+', '-');
}