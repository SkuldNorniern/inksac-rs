use std::fmt;

use crate::ansi_base::{BOLD, DIM, ITALIC, RESET, UNDERLINE};

/// String with the colored text
/// # Example
/// ```rust
/// use inksac::types::*;
///
/// let TITLESTYLE: Style = Style{
///     forground: Some(Color::Green),
///     background: Some(Color::Red),
///     ..Default::default()
/// };
/// let title_text: ColoredString = ColoredString::new(
///    "Hello World",
///     TITLESTYLE
///     );
/// println!("{}", title_text);
/// ```

#[derive(Debug, Clone)]
pub struct ColoredString {
    pub string: String,
    pub style: Style,
}

#[allow(dead_code)]
impl ColoredString {
    pub fn new(string: &str, style: Style) -> Self {
        Self {
            string: string.to_string(),
            style,
        }
    }
    /// Returns the non colored String
    pub fn to_no_style(&self) -> String {
        self.string.clone()
    }
}

impl fmt::Display for ColoredString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.style, self.string, RESET)
    }
}

/// Style for the colored text
/// # Example
/// ```rust
/// use inksac::types::*;
///
/// let TITLESTYLE: Style = Style{
///     forground: Some(Color::Green),
///     background: Some(Color::Red),
///     ..Default::default()
/// };
/// ```

#[derive(Debug, Clone, Copy, Default)]
pub struct Style {
    pub forground: Option<Color>,
    pub background: Option<Color>,
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
}
impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fg = self.forground.unwrap_or(Color::Empty).to_fg();
        let bg = self.background.unwrap_or(Color::Empty).to_bg();
        let bold = if self.bold { BOLD } else { "" };
        let dim = if self.dim { DIM } else { "" };
        let italic = if self.italic { ITALIC } else { "" };
        let underline = if self.underline { UNDERLINE } else { "" };

        write!(f, "{}{}{}{}{}{}", fg, bg, bold, dim, italic, underline)
    }
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }
    /// Toggle Bold
    pub fn bold(mut self) -> Self {
        if !self.bold {
            self.bold = true;
        } else {
            self.bold = false;
        }
        self
    }
    /// Toggle Dim
    pub fn dim(mut self) -> Self {
        if !self.dim {
            self.dim = true;
        } else {
            self.dim = false;
        }
        self
    }
    /// Toggle Italic
    pub fn italic(mut self) -> Self {
        if !self.italic {
            self.italic = true;
        } else {
            self.italic = false;
        }
        self
    }
    /// Toggle Underline
    pub fn underline(mut self) -> Self {
        if !self.underline {
            self.underline = true;
        } else {
            self.underline = false;
        }
        self
    }
}

/// Color Palette

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,

    Empty,
    RGB(u8, u8, u8),
    HEX(&'static str),
}
impl Color {
    fn to_fg(self) -> String {
        match self {
            Color::Black => "\x1b[30m".to_string(),
            Color::Red => "\x1b[31m".to_string(),
            Color::Green => "\x1b[32m".to_string(),
            Color::Yellow => "\x1b[33m".to_string(),
            Color::Blue => "\x1b[34m".to_string(),
            Color::Magenta => "\x1b[35m".to_string(),
            Color::Cyan => "\x1b[36m".to_string(),
            Color::White => "\x1b[37m".to_string(),
            Color::Empty => "".to_string(),
            Color::RGB(r, g, b) => format!("\x1b[38;2;{};{};{}m", r, g, b),
            Color::HEX(code) => {
                let (r, g, b) = Self::hex_to_rgb(code);

                format!("\x1b[38;2;{};{};{}m", r, g, b)
            }
        }
    }
    fn to_bg(self) -> String {
        match self {
            Color::Black => "\x1b[40m".to_string(),
            Color::Red => "\x1b[41m".to_string(),
            Color::Green => "\x1b[42m".to_string(),
            Color::Yellow => "\x1b[43m".to_string(),
            Color::Blue => "\x1b[44m".to_string(),
            Color::Magenta => "\x1b[45m".to_string(),
            Color::Cyan => "\x1b[46m".to_string(),
            Color::White => "\x1b[47m".to_string(),
            Color::Empty => "".to_string(),
            Color::RGB(r, g, b) => format!("\x1b[48;2;{};{};{}m", r, g, b),
            Color::HEX(code) => {
                let (r, g, b) = Self::hex_to_rgb(code);

                format!("\x1b[48;2;{};{};{}m", r, g, b)
            }
        }
    }
    fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        (r, g, b)
    }
}

/*#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn underline() {
        let style = Style::default();
        let style2 = Style{
            forground: Some(Color::HEX("#6667AB")),
            ..Default::default()
        };
        let text = ColoredString::new("hello", style.clone());
        let text2 = ColoredString::new("world", style2.clone());

        println!("{}", text);
        println!("{}", text2);
        panic!();

    }
}*/
