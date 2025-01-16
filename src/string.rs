//! Colored string implementation and styling trait
//! 
//! This module provides the ColoredString type and the Styleable trait for
//! applying styles to strings. It handles the combination of text content
//! with style information and proper ANSI escape sequence formatting.
//! 
//! # Examples
//! 
//! ```rust
//! use inksac::{Style, Color, ColoredString, Styleable};
//! 
//! let style = Style::builder()
//!     .foreground(Color::Green)
//!     .bold()
//!     .build();
//! 
//! // Using the Styleable trait
//! let colored = "Hello, world!".style(style);
//! println!("{}", colored);
//! 
//! // Direct creation
//! let colored = ColoredString::new("Hello", style);
//! println!("{}", colored);
//! ```

use std::fmt;
use crate::{Style, ansi};

/// A string with an associated style
#[derive(Debug, Clone)]
pub struct ColoredString {
    string: String,
    style: Style,
}

impl ColoredString {
    /// Create a new ColoredString with the given text and style
    /// 
    /// # Arguments
    /// * `string` - The text content
    /// * `style` - The style to apply
    /// 
    /// # Examples
    /// ```rust
    /// use inksac::{ColoredString, Style, Color};
    /// 
    /// let style = Style::builder()
    ///     .foreground(Color::Blue)
    ///     .build();
    /// 
    /// let colored = ColoredString::new("Hello", style);
    /// ```
    pub fn new(string: &str, style: Style) -> Self {
        Self {
            string: string.to_owned(),
            style,
        }
    }

    /// Get the original string without styling
    pub fn to_no_style(&self) -> &str {
        &self.string
    }

    /// Apply additional style to existing ColoredString
    /// 
    /// # Examples
    /// ```rust
    /// use inksac::{Style, Color, Styleable};
    /// 
    /// let base_style = Style::builder()
    ///     .foreground(Color::Blue)
    ///     .build();
    /// 
    /// let highlight = Style::builder()
    ///     .bold()
    ///     .build();
    /// 
    /// let text = "Hello".style(base_style)
    ///     .with_style(highlight);
    /// ```
    pub fn with_style(self, additional: Style) -> Self {
        Self {
            string: self.string,
            style: self.style.compose(additional),
        }
    }

    /// Get the length of the underlying string
    pub fn len(&self) -> usize {
        self.string.len()
    }

    /// Check if the string is empty
    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }

    /// Convert to a plain string with all styling removed
    pub fn into_string(self) -> String {
        self.string
    }
}

impl fmt::Display for ColoredString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.style, self.string, ansi::RESET)
    }
}

/// Trait for applying styles to strings
pub trait Styleable {
    /// Apply a style to create a ColoredString
    fn style(self, style: Style) -> ColoredString;
}

impl<T: AsRef<str>> Styleable for T {
    fn style(self, style: Style) -> ColoredString {
        ColoredString::new(self.as_ref(), style)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;

    #[test]
    fn test_colored_string_creation() {
        let style = Style::builder()
            .foreground(Color::Red)
            .build();
        
        let colored = ColoredString::new("test", style);
        assert_eq!(colored.into_string(), "test");
    }

    #[test]
    fn test_styleable_trait() {
        let style = Style::builder()
            .foreground(Color::Blue)
            .build();

        let colored: ColoredString = "test".style(style);
        assert_eq!(colored.into_string(), "test");
        
        let string = String::from("test");
        let colored = string.style(style);
        assert_eq!(colored.into_string(), "test");
    }

    #[test]
    fn test_display_formatting() {
        let style = Style::builder()
            .foreground(Color::Red)
            .build();
        
        let colored = "test".style(style);
        let output = colored.to_string();
        
        assert!(output.starts_with("\x1b["));
        assert!(output.ends_with("\x1b[0m"));
        assert!(output.contains("test"));
    }
} 