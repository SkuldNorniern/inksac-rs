//! Color definitions and operations
//!
//! This module provides the [`Color`] enum and associated functions for working
//! with colors in terminal environments. It handles:
//!
//! - Basic ANSI colors (16 colors)
//! - RGB colors (24-bit true color)
//! - Hex color codes
//! - Color manipulation (lighten/darken)
//!
//! # Terminal Support
//!
//! Color support is automatically detected and operations will return appropriate
//! errors if the requested color mode isn't supported.
//!
//! # Examples
//!
//! ```rust
//! use inksac::{Color, ColorError};
//!
//! fn color_examples() -> Result<(), ColorError> {
//!     // Basic ANSI color
//!     let red = Color::Red;
//!
//!     // RGB color
//!     let orange = Color::new_rgb(255, 165, 0)?;
//!
//!     // Hex color
//!     let blue = Color::new_hex("#0000FF")?;
//!
//!     // Color manipulation
//!     let lighter = orange.lighten(30)?;
//!     let darker = blue.darken(20)?;
//!
//!     Ok(())
//! }
//! ```

use std::borrow::Cow;
use crate::error::{ColorError, ColorSupport};
use crate::check_color_support;

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
    #[default]
    Empty,
    RGB(u8, u8, u8),
    HEX(&'static str),
}

impl Color {
    /// Create a new RGB color
    ///
    /// This function will check if the terminal supports true color (16 million colors)
    /// before creating the color.
    ///
    /// # Arguments
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    ///
    /// # Returns
    /// * `Ok(Color)` if the terminal supports true color
    /// * `Err(ColorError)` if true color is not supported
    ///
    /// # Examples
    /// ```rust
    /// use inksac::Color;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let orange = Color::new_rgb(255, 165, 0)?;
    ///     let purple = Color::new_rgb(128, 0, 128)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new_rgb(r: u8, g: u8, b: u8) -> Result<Self, ColorError> {
        match check_color_support()? {
            ColorSupport::TrueColor => Ok(Color::RGB(r, g, b)),
            support => Err(ColorError::UnsupportedColorMode(
                ColorSupport::TrueColor,
                support
            )),
        }
    }

    /// Create a new color from a hexadecimal color code
    ///
    /// The hex code must start with '#' and be followed by exactly 6 hexadecimal
    /// digits (e.g., "#FF0000" for red).
    ///
    /// # Arguments
    /// * `hex` - Hexadecimal color code (e.g., "#FF0000")
    ///
    /// # Returns
    /// * `Ok(Color)` if the hex code is valid and terminal supports true color
    /// * `Err(ColorError)` if the hex code is invalid or true color is not supported
    ///
    /// # Examples
    /// ```rust
    /// use inksac::Color;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let red = Color::new_hex("#FF0000")?;
    ///     let green = Color::new_hex("#00FF00")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new_hex(hex: &'static str) -> Result<Self, ColorError> {
        Self::validate_hex(hex)?;
        
        match check_color_support()? {
            ColorSupport::TrueColor => Ok(Color::HEX(hex)),
            support => Err(ColorError::UnsupportedColorMode(
                ColorSupport::TrueColor,
                support
            )),
        }
    }

    pub(crate) fn validate_hex(hex: &str) -> Result<(u8, u8, u8), ColorError> {
        let hex = hex.strip_prefix('#')
            .ok_or_else(|| ColorError::InvalidHexCode(hex.to_string()))?;

        if hex.len() != 6 {
            return Err(ColorError::InvalidHexCode(hex.to_string()));
        }

        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| ColorError::InvalidHexCode(hex.to_string()))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| ColorError::InvalidHexCode(hex.to_string()))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| ColorError::InvalidHexCode(hex.to_string()))?;

        Ok((r, g, b))
    }

    pub(crate) fn to_fg(self) -> Cow<'static, str> {
        match self {
            Color::Black => Cow::Borrowed("\x1b[30m"),
            Color::Red => Cow::Borrowed("\x1b[31m"),
            Color::Green => Cow::Borrowed("\x1b[32m"),
            Color::Yellow => Cow::Borrowed("\x1b[33m"),
            Color::Blue => Cow::Borrowed("\x1b[34m"),
            Color::Magenta => Cow::Borrowed("\x1b[35m"),
            Color::Cyan => Cow::Borrowed("\x1b[36m"),
            Color::White => Cow::Borrowed("\x1b[37m"),
            Color::Empty => Cow::Borrowed(""),
            Color::RGB(r, g, b) => Cow::Owned(format!("\x1b[38;2;{};{};{}m", r, g, b)),
            Color::HEX(code) => {
                let (r, g, b) = Self::validate_hex(code)
                    .expect("Invalid hex code - this should be validated at construction");
                Cow::Owned(format!("\x1b[38;2;{};{};{}m", r, g, b))
            }
        }
    }

    pub(crate) fn to_bg(self) -> Cow<'static, str> {
        match self {
            Color::Black => Cow::Borrowed("\x1b[40m"),
            Color::Red => Cow::Borrowed("\x1b[41m"),
            Color::Green => Cow::Borrowed("\x1b[42m"),
            Color::Yellow => Cow::Borrowed("\x1b[43m"),
            Color::Blue => Cow::Borrowed("\x1b[44m"),
            Color::Magenta => Cow::Borrowed("\x1b[45m"),
            Color::Cyan => Cow::Borrowed("\x1b[46m"),
            Color::White => Cow::Borrowed("\x1b[47m"),
            Color::Empty => Cow::Borrowed(""),
            Color::RGB(r, g, b) => Cow::Owned(format!("\x1b[48;2;{};{};{}m", r, g, b)),
            Color::HEX(code) => {
                let (r, g, b) = Self::validate_hex(code)
                    .expect("Invalid hex code - this should be validated at construction");
                Cow::Owned(format!("\x1b[48;2;{};{};{}m", r, g, b))
            }
        }
    }

    /// Lighten a color by a percentage
    ///
    /// # Arguments
    /// * `percent` - Amount to lighten (0-100)
    ///
    /// # Returns
    /// * `Ok(Color)` - Lightened color
    /// * `Err(ColorError)` - If color manipulation fails
    ///
    /// # Examples
    /// ```rust
    /// use inksac::Color;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let color = Color::new_rgb(255, 100, 0)?;
    ///     let lighter = color.lighten(30)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn lighten(self, percent: u8) -> Result<Self, ColorError> {
        match self {
            Color::RGB(r, g, b) => {
                let percent = f32::from(percent.min(100)) / 100.0;
                let r = ((255.0 - f32::from(r)) * percent + f32::from(r)) as u8;
                let g = ((255.0 - f32::from(g)) * percent + f32::from(g)) as u8;
                let b = ((255.0 - f32::from(b)) * percent + f32::from(b)) as u8;
                Color::new_rgb(r, g, b)
            }
            Color::HEX(hex) => {
                let (r, g, b) = Self::validate_hex(hex)?;
                Color::RGB(r, g, b).lighten(percent)
            }
            _ => Ok(self),
        }
    }

    /// Darken a color by a percentage
    ///
    /// # Arguments
    /// * `percent` - Amount to darken (0-100)
    ///
    /// # Returns
    /// * `Ok(Color)` - Darkened color
    /// * `Err(ColorError)` - If color manipulation fails
    ///
    /// # Examples
    /// ```rust
    /// use inksac::Color;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let color = Color::new_rgb(255, 100, 0)?;
    ///     let darker = color.darken(30)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn darken(self, percent: u8) -> Result<Self, ColorError> {
        match self {
            Color::RGB(r, g, b) => {
                let percent = f32::from(percent.min(100)) / 100.0;
                let r = (f32::from(r) * (1.0 - percent)) as u8;
                let g = (f32::from(g) * (1.0 - percent)) as u8;
                let b = (f32::from(b) * (1.0 - percent)) as u8;
                Color::new_rgb(r, g, b)
            }
            Color::HEX(hex) => {
                let (r, g, b) = Self::validate_hex(hex)?;
                Color::RGB(r, g, b).darken(percent)
            }
            _ => Ok(self),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_color() {
        let color = Color::new_rgb(255, 128, 0);
        assert!(color.is_ok());
    }

    #[test]
    fn test_hex_validation() {
        assert!(Color::validate_hex("#FF8000").is_ok());
        assert!(Color::validate_hex("FF8000").is_err());
        assert!(Color::validate_hex("#FF800").is_err());
        assert!(Color::validate_hex("#GGGGGG").is_err());
    }
} 