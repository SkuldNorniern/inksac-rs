// FIX: ASAP: re-design this cursed module structure
use crate::ansi_base::{BOLD, DIM, ITALIC, RESET, UNDERLINE};
use std::fmt;

/// String with the colored text
///
/// # Example
///
/// ```
/// use inksac::types::*;
///
/// let TITLESTYLE: Style = Style{
///     foreground: Color::Green,
///     background: Color::Red,
///     ..Default::default()
/// };
/// let title_text = "Hello World".styled(TITLESTYLE);
/// println!("{}", title_text);
/// ```
#[derive(Debug, Clone)]
pub struct ColoredString {
    pub string: String,
    pub style: Style,
}

impl ColoredString {
    /// Creates a new `ColoredString` with the given string and style.
    pub fn new(string: &str, style: Style) -> Self {
        Self {
            string: string.into(),
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

// FIX!: LATER: trait name should be verb
/// Trait for types that can be styled with a `Style`
pub trait Stylish {
    // FIX!: LATER: trait's only method should have consistent name with the trait
    fn styled(self, style: Style) -> ColoredString;
}

// FIX: blanket impl for everything that implements `ToString` or `AsRef<str>`
impl Stylish for String {
    fn styled(self, style: Style) -> ColoredString {
        ColoredString::new(&self, style)
    }
}

impl<'a> Stylish for &'a str {
    fn styled(self, style: Style) -> ColoredString {
        ColoredString::new(self, style)
    }
}

/// A struct representing various styles that can be applied to a string.
///
/// Styles include foreground and background color, boldness, dimness, italicization, and underlining.
///
/// # Example
///
/// ```
/// use inksac::types::*;
///
/// let TITLESTYLE: Style = Style{
///     foreground: Color::Green,
///     background: Color::Red,
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Style {
    pub foreground: Color,
    pub background: Color,
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fg = if self.foreground != Color::Empty {
            self.foreground.to_fg()
        } else {
            Color::Empty.to_fg()
        };
        let bg = if self.background != Color::Empty {
            self.background.to_bg()
        } else {
            Color::Empty.to_bg()
        };
        let bold = if self.bold { BOLD } else { "" };
        let dim = if self.dim { DIM } else { "" };
        let italic = if self.italic { ITALIC } else { "" };
        let underline = if self.underline { UNDERLINE } else { "" };

        write!(f, "{}{}{}{}{}{}", fg, bg, bold, dim, italic, underline)
    }
}

impl Style {
    /// Creates a new instance of `StyleBuilder` with default values.
    ///
    /// # Example
    ///
    /// ```
    /// use inksac::types::Style;
    ///
    /// let builder = Style::builder();
    /// ```
    pub fn builder() -> StyleBuilder {
        StyleBuilder::default()
    }
}

// FIX!: unnecessary builder pattern
/// A builder struct for constructing a `Style` instance with various configurations.
pub struct StyleBuilder {
    style: Style,
}

impl Default for StyleBuilder {
    /// Constructs a new `StyleBuilder` with a default `Style`.
    fn default() -> Self {
        Self {
            style: Style::default(),
        }
    }
}

impl StyleBuilder {
    /// Sets the foreground color of the style.
    ///
    /// # Arguments
    ///
    /// * `color` - An option containing a `Color` enum variant to set as the foreground color.
    ///
    /// # Example
    ///
    /// ```
    /// use inksac::types::{StyleBuilder, Color};
    ///
    /// let style = StyleBuilder::default()
    ///     .foreground(Color::Green)
    ///     .build();
    /// ```
    pub fn foreground(mut self, color: Color) -> Self {
        // FIX!: ASAP: take & return mutable reference rather than taking ownership
        // | e.g. (&mut self, color: Color) -> &mut Self
        // | also applys to every builder pattern methods below
        self.style.foreground = color;
        self
    }

    /// Sets the background color of the style.
    ///
    /// # Arguments
    ///
    /// * `color` - An option containing a `Color` enum variant to set as the background color.
    ///
    /// # Example
    ///
    /// ```
    /// use inksac::types::{StyleBuilder, Color};
    ///
    /// let style = StyleBuilder::default()
    ///     .background(Color::Red)
    ///     .build();
    /// ```
    pub fn background(mut self, color: Color) -> Self {
        self.style.background = color;
        self
    }

    /// Sets the bold attribute of the style to true.
    ///
    /// # Example
    ///
    /// ```
    /// use inksac::types::StyleBuilder;
    ///
    /// let style = StyleBuilder::default()
    ///     .bold()
    ///     .build();
    /// ```
    pub fn bold(mut self) -> Self {
        self.style.bold = true;
        self
    }

    /// Sets the dim attribute of the style to true.
    ///
    /// # Example
    ///
    /// ```
    /// use inksac::types::StyleBuilder;
    ///
    /// let style = StyleBuilder::default()
    ///     .dim()
    ///     .build();
    /// ```
    pub fn dim(mut self) -> Self {
        self.style.dim = true;
        self
    }

    /// Sets the italic attribute of the style to true.
    ///
    /// # Example
    ///
    /// ```
    /// use inksac::types::StyleBuilder;
    ///
    /// let style = StyleBuilder::default()
    ///     .italic()
    ///     .build();
    /// ```
    pub fn italic(mut self) -> Self {
        self.style.italic = true;
        self
    }

    /// Sets the underline attribute of the style to true.
    ///
    /// # Example
    ///
    /// ```
    /// use inksac::types::StyleBuilder;
    ///
    /// let style = StyleBuilder::default()
    ///     .underline()
    ///     .build();
    /// ```
    pub fn underline(mut self) -> Self {
        self.style.underline = true;
        self
    }

    /// Builds and returns a `Style` instance with the configurations set in the builder.
    ///
    /// # Example
    ///
    /// ```
    /// use inksac::types::{StyleBuilder,Color};
    ///
    /// let style = StyleBuilder::default()
    ///     .foreground(Color::Green)
    ///     .bold()
    ///     .build();
    /// ```
    pub fn build(self) -> Style {
        self.style
    }
}

/// Represents the different colors that can be used for text foreground and background styling.
///
/// The enum provides several options to specify colors:
/// - Predefined color values (e.g., `Black`, `Red`, `Green`, etc.)
/// - RGB values with the `RGB` variant
/// - Hexadecimal color codes with the `HEX` variant
///
/// # Examples
///
/// Using predefined color values:
///
/// ```
/// use inksac::types::Color;
///
/// let red = Color::Red;
/// let green = Color::Green;
/// ```
///
/// Using RGB values:
///
/// ```
/// use inksac::types::Color;
///
/// let custom_color = Color::RGB(128, 0, 128);
/// ```
///
/// Using a hexadecimal color code:
///
/// ```
/// use inksac::types::Color;
///
/// let custom_color = Color::HEX("#800080");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,

    /// Represents an absence of color.
    #[default]
    Empty,

    /// Specifies a color using RGB values.
    RGB(u8, u8, u8),

    /// Specifies a color using a hexadecimal color code.
    HEX(&'static str),
}

impl Color {
    /// Converts the `Color` enum variant to its corresponding foreground ANSI escape code string.
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
                // FIX: converting str to integer and back to String
                let (r, g, b) = match Self::hex_to_rgb(code) {
                    Some(rgb) => rgb,
                    None => panic!("Invalid hex code: {}", code),
                };

                format!("\x1b[38;2;{};{};{}m", r, g, b)
            }
        }
    }

    /// Converts the `Color` enum variant to its corresponding background ANSI escape code string.
    fn to_bg(self) -> String {
        match self {
            // FIX!: use `Cow<'static, str>` to avoid `to_string()`
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
                let (r, g, b) = match Self::hex_to_rgb(code) {
                    Some(rgb) => rgb,
                    None => panic!("Invalid hex code: {}", code),
                };

                format!("\x1b[48;2;{};{};{}m", r, g, b)
            }
        }
    }

    /// Converts a hexadecimal color code (as a string) to a tuple of RGB values.
    ///
    /// This is used internally by the `to_fg` and `to_bg` methods when handling `Color::HEX` variants.
    ///
    /// # Parameters
    ///
    /// - `hex`: A string slice representing the hexadecimal color code.
    ///
    /// # Returns
    ///
    /// A tuple of three `u8` values representing the red, green, and blue components of the color, respectively.
    ///
    fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
        let hex = hex.strip_prefix('#')?;

        // if the length of the hex string is not 6, panic the code
        // Since the terminal does not support `RGBA` colors anyway
        if hex.len() != 6 {
            return None;
        }

        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

        Some((r, g, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        let hex = "#ff0000";
        let (r, g, b) = Color::hex_to_rgb(hex).unwrap();
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
    }
}
