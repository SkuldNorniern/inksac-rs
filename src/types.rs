use std::fmt;

use crate::ansi_base::RESET;


// Colored String type

#[derive(Debug, Clone)]
pub struct ColoredString {
    pub string: String,
    pub color: Style
}
#[allow(dead_code)]
impl ColoredString{
    pub fn new(string: String, color: Style) -> Self {
        Self{string, color}
    }
    pub fn to_string(&self) -> String {
        self.string.clone()
    }
}

impl fmt::Display for ColoredString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.color, self.string, RESET)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Style{
    pub forground: Option<Color>,
    pub background: Option<Color>
}
impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fg = self.forground.unwrap_or(Color::Reset).to_fg();
        let bg = self.background.unwrap_or(Color::Reset).to_bg();
        
        write!(f, "{}{}", fg, bg)
    }
}
impl Default for Style {
    fn default() -> Self {
        Self{forground: None, background: None}
    }
}
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
    
    Reset,
    RGB(u8, u8, u8),
}
impl Color {
    fn to_fg(&self) -> String {
        match *self {
            Color::Black => "\x1b[30m".to_string(),
            Color::Red =>  "\x1b[31m".to_string(),
            Color::Green => "\x1b[32m".to_string(),
            Color::Yellow => "\x1b[33m".to_string(),
            Color::Blue => "\x1b[34m".to_string(),
            Color::Magenta => "\x1b[35m".to_string(),
            Color::Cyan => "\x1b[36m".to_string(),
            Color::White => "\x1b[37m".to_string(),
            Color::Reset => "\x1b[0m".to_string(),
            Color::RGB(r, g, b) => format!("\x1b[38;2;{};{};{}m", r, g, b)
        }
    }
    fn to_bg(&self) -> String {
        match self {
            Color::Black => "\x1b[40m".to_string(),
            Color::Red =>  "\x1b[41m".to_string(),
            Color::Green => "\x1b[42m".to_string(),
            Color::Yellow => "\x1b[43m".to_string(),
            Color::Blue => "\x1b[44m".to_string(),
            Color::Magenta => "\x1b[45m".to_string(),
            Color::Cyan => "\x1b[46m".to_string(),
            Color::White => "\x1b[47m".to_string(),
            Color::Reset => "\x1b[0m".to_string(),
            Color::RGB(r, g, b) => format!("\x1b[48;2;{};{};{}m", r, g, b) 

        }
    }
}

