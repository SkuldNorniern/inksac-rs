//! Style definitions and builder pattern implementation
//! 
//! This module provides the Style struct and its builder pattern implementation
//! for creating terminal text styles. Styles can combine colors and text formatting
//! options like bold, italic, and underline.
//! 
//! # Examples
//! 
//! ```rust
//! use inksac::{Style, Color};
//! 
//! // Create a simple style
//! let basic = Style::builder()
//!     .foreground(Color::Green)
//!     .bold()
//!     .build();
//! 
//! // Create a complex style
//! let complex = Style::builder()
//!     .foreground(Color::new_rgb(255, 128, 0).unwrap())
//!     .background(Color::Blue)
//!     .bold()
//!     .italic()
//!     .underline()
//!     .build();
//! ```

use std::fmt;
use crate::color::Color;
use crate::ansi;

/// Represents a complete text style including colors and formatting
#[derive(Debug, Clone, Copy, Default)]
pub struct Style {
    pub(crate) foreground: Color,
    pub(crate) background: Color,
    pub(crate) bold: bool,
    pub(crate) dim: bool,
    pub(crate) italic: bool,
    pub(crate) underline: bool,
}

impl Style {
    /// Create a new StyleBuilder instance
    /// 
    /// # Examples
    /// ```
    /// use inksac::{Style, Color};
    /// 
    /// let style = Style::builder()
    ///     .foreground(Color::Green)
    ///     .bold()
    ///     .build();
    /// ```
    pub fn builder() -> StyleBuilder {
        StyleBuilder::default()
    }

    /// Combine two styles, with the second style overriding the first
    /// 
    /// # Examples
    /// ```rust
    /// use inksac::{Style, Color};
    /// 
    /// let base = Style::builder()
    ///     .foreground(Color::Blue)
    ///     .bold()
    ///     .build();
    /// 
    /// let highlight = Style::builder()
    ///     .background(Color::Yellow)
    ///     .build();
    /// 
    /// let combined = base.compose(highlight);
    /// ```
    pub fn compose(self, other: Style) -> Style {
        Style {
            foreground: if other.foreground == Color::Empty {
                self.foreground
            } else {
                other.foreground
            },
            background: if other.background == Color::Empty {
                self.background
            } else {
                other.background
            },
            bold: self.bold || other.bold,
            dim: self.dim || other.dim,
            italic: self.italic || other.italic,
            underline: self.underline || other.underline,
        }
    }

    /// Create a new style with all attributes cleared
    pub fn clear() -> Style {
        Style::default()
    }

    /// Check if the style has any attributes set
    /// 
    /// # Returns
    /// `true` if no colors or formatting options are set
    pub fn is_empty(&self) -> bool {
        self.foreground == Color::Empty 
            && self.background == Color::Empty
            && !self.bold
            && !self.dim
            && !self.italic
            && !self.underline
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fg = self.foreground.to_fg();
        let bg = self.background.to_bg();
        
        let bold = if self.bold { ansi::BOLD } else { "" };
        let dim = if self.dim { ansi::DIM } else { "" };
        let italic = if self.italic { ansi::ITALIC } else { "" };
        let underline = if self.underline { ansi::UNDERLINE } else { "" };

        write!(f, "{}{}{}{}{}{}", fg, bg, bold, dim, italic, underline)
    }
}

/// Builder for creating Style instances
#[derive(Default)]
pub struct StyleBuilder {
    style: Style,
}

impl StyleBuilder {
    /// Set the foreground (text) color
    pub fn foreground(&mut self, color: Color) -> &mut Self {
        self.style.foreground = color;
        self
    }

    /// Set the background color
    pub fn background(&mut self, color: Color) -> &mut Self {
        self.style.background = color;
        self
    }

    /// Enable bold text
    pub fn bold(&mut self) -> &mut Self {
        self.style.bold = true;
        self
    }

    /// Enable dim text
    pub fn dim(&mut self) -> &mut Self {
        self.style.dim = true;
        self
    }

    /// Enable italic text
    pub fn italic(&mut self) -> &mut Self {
        self.style.italic = true;
        self
    }

    /// Enable underlined text
    pub fn underline(&mut self) -> &mut Self {
        self.style.underline = true;
        self
    }

    /// Build the final Style
    pub fn build(&self) -> Style {
        self.style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_builder() {
        let style = Style::builder()
            .foreground(Color::Red)
            .background(Color::Blue)
            .bold()
            .italic()
            .build();

        assert_eq!(style.foreground, Color::Red);
        assert_eq!(style.background, Color::Blue);
        assert!(style.bold);
        assert!(style.italic);
        assert!(!style.dim);
        assert!(!style.underline);
    }

    #[test]
    fn test_style_display() {
        let style = Style::builder()
            .foreground(Color::Red)
            .bold()
            .build();

        let output = style.to_string();
        assert!(output.contains("\x1b[31m")); // Red
        assert!(output.contains("\x1b[1m")); // Bold
    }
} 