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

use crate::ansi;
use crate::check_color_support;
use crate::error::{ColorError, ColorSupport};
use std::borrow::Cow;
use std::env;

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
    HSV(u16, u8, u8), // Hue (0-360), Saturation (0-100), Value (0-100)
    HSL(u16, u8, u8), // Hue (0-360), Saturation (0-100), Lightness (0-100)
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
                support,
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
                support,
            )),
        }
    }

    pub(crate) fn validate_hex(hex: &str) -> Result<(u8, u8, u8), ColorError> {
        let hex = hex
            .strip_prefix('#')
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
            Color::Black => Cow::Borrowed(ansi::FG_BLACK),
            Color::Red => Cow::Borrowed(ansi::FG_RED),
            Color::Green => Cow::Borrowed(ansi::FG_GREEN),
            Color::Yellow => Cow::Borrowed(ansi::FG_YELLOW),
            Color::Blue => Cow::Borrowed(ansi::FG_BLUE),
            Color::Magenta => Cow::Borrowed(ansi::FG_MAGENTA),
            Color::Cyan => Cow::Borrowed(ansi::FG_CYAN),
            Color::White => Cow::Borrowed(ansi::FG_WHITE),
            Color::Empty => Cow::Borrowed(""),
            Color::RGB(r, g, b) => Cow::Owned(ansi::fg_rgb(r, g, b)),
            Color::HEX(code) => {
                let (r, g, b) = Self::validate_hex(code)
                    .expect("Invalid hex code - this should be validated at construction");
                Cow::Owned(ansi::fg_rgb(r, g, b))
            }
            Color::HSV(h, s, v) => {
                let (r, g, b) = Self::hsv_to_rgb(h, s, v);
                Cow::Owned(ansi::fg_rgb(r, g, b))
            }
            Color::HSL(h, s, l) => {
                let (r, g, b) = Self::hsl_to_rgb(h, s, l);
                Cow::Owned(ansi::fg_rgb(r, g, b))
            }
        }
    }

    pub(crate) fn to_bg(self) -> Cow<'static, str> {
        match self {
            Color::Black => Cow::Borrowed(ansi::BG_BLACK),
            Color::Red => Cow::Borrowed(ansi::BG_RED),
            Color::Green => Cow::Borrowed(ansi::BG_GREEN),
            Color::Yellow => Cow::Borrowed(ansi::BG_YELLOW),
            Color::Blue => Cow::Borrowed(ansi::BG_BLUE),
            Color::Magenta => Cow::Borrowed(ansi::BG_MAGENTA),
            Color::Cyan => Cow::Borrowed(ansi::BG_CYAN),
            Color::White => Cow::Borrowed(ansi::BG_WHITE),
            Color::Empty => Cow::Borrowed(""),
            Color::RGB(r, g, b) => Cow::Owned(ansi::bg_rgb(r, g, b)),
            Color::HEX(code) => {
                let (r, g, b) = Self::validate_hex(code)
                    .expect("Invalid hex code - this should be validated at construction");
                Cow::Owned(ansi::bg_rgb(r, g, b))
            }
            Color::HSV(h, s, v) => {
                let (r, g, b) = Self::hsv_to_rgb(h, s, v);
                Cow::Owned(ansi::bg_rgb(r, g, b))
            }
            Color::HSL(h, s, l) => {
                let (r, g, b) = Self::hsl_to_rgb(h, s, l);
                Cow::Owned(ansi::bg_rgb(r, g, b))
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

    /// Create a new HSV color
    ///
    /// # Arguments
    /// * `h` - Hue (0-360)
    /// * `s` - Saturation (0-100)
    /// * `v` - Value (0-100)
    ///
    /// # Returns
    /// * `Ok(Color)` if the terminal supports true color
    /// * `Err(ColorError)` if true color is not supported
    pub fn new_hsv(h: u16, s: u8, v: u8) -> Result<Self, ColorError> {
        if h > 360 || s > 100 || v > 100 {
            return Err(ColorError::InvalidColorValue(
                "HSV values out of range".into(),
            ));
        }

        match check_color_support()? {
            ColorSupport::TrueColor => Ok(Color::HSV(h, s, v)),
            support => Err(ColorError::UnsupportedColorMode(
                ColorSupport::TrueColor,
                support,
            )),
        }
    }

    /// Create a new HSL color
    ///
    /// # Arguments
    /// * `h` - Hue (0-360)
    /// * `s` - Saturation (0-100)
    /// * `l` - Lightness (0-100)
    ///
    /// # Returns
    /// * `Ok(Color)` if the terminal supports true color
    /// * `Err(ColorError)` if true color is not supported
    pub fn new_hsl(h: u16, s: u8, l: u8) -> Result<Self, ColorError> {
        if h > 360 || s > 100 || l > 100 {
            return Err(ColorError::InvalidColorValue(
                "HSL values out of range".into(),
            ));
        }

        match check_color_support()? {
            ColorSupport::TrueColor => Ok(Color::HSL(h, s, l)),
            support => Err(ColorError::UnsupportedColorMode(
                ColorSupport::TrueColor,
                support,
            )),
        }
    }

    // Helper function to convert HSV to RGB
    fn hsv_to_rgb(h: u16, s: u8, v: u8) -> (u8, u8, u8) {
        let h = f32::from(h) / 60.0;
        let s = f32::from(s) / 100.0;
        let v = f32::from(v) / 100.0;

        let c = v * s;
        let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
        let m = v - c;

        let (r, g, b) = match h as u8 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            5 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };

        (
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }

    // Helper function to convert HSL to RGB
    fn hsl_to_rgb(h: u16, s: u8, l: u8) -> (u8, u8, u8) {
        let h = f32::from(h) / 360.0;
        let s = f32::from(s) / 100.0;
        let l = f32::from(l) / 100.0;

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h * 6.0 % 2.0) - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = match (h * 6.0) as u8 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            5 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };

        (
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn with_test_env<F, T>(test: F) -> T
    where
        F: FnOnce() -> T,
    {
        let vars = [
            ("NO_COLOR", None),
            ("COLORTERM", Some("truecolor")),
            ("TERM", Some("xterm-256color")),
            ("TERM_PROGRAM", Some("test")),
            ("CLICOLOR", Some("1")),
            ("CLICOLOR_FORCE", Some("1")),
        ];

        // Store original environment
        let original: Vec<(String, Option<String>)> = vars
            .iter()
            .map(|(name, _)| (name.to_string(), env::var(name).ok()))
            .collect();

        // Clear all color-related environment variables first
        for (name, _) in &vars {
            env::remove_var(name);
        }

        // Set test environment
        for (name, value) in vars {
            match value {
                Some(v) => env::set_var(name, v),
                None => env::remove_var(name),
            }
        }

        // Run test
        let result = test();

        // Restore original environment
        for (name, value) in original {
            match value {
                Some(v) => env::set_var(name, v),
                None => env::remove_var(&name),
            }
        }

        result
    }

    #[test]
    fn test_rgb_color() {
        with_test_env(|| {
            let color = Color::new_rgb(255, 128, 0);
            assert!(color.is_ok());
        });
    }

    #[test]
    fn test_hex_validation() {
        assert!(Color::validate_hex("#FF8000").is_ok());
        assert!(Color::validate_hex("FF8000").is_err());
        assert!(Color::validate_hex("#FF800").is_err());
        assert!(Color::validate_hex("#GGGGGG").is_err());
    }

    #[test]
    fn test_hsv_color() {
        with_test_env(|| {
            let color = Color::new_hsv(0, 100, 100); // Pure red
            assert!(color.is_ok());

            let invalid_color = Color::new_hsv(361, 100, 100);
            assert!(invalid_color.is_err());
        });
    }

    #[test]
    fn test_hsl_color() {
        with_test_env(|| {
            let color = Color::new_hsl(120, 100, 50); // Pure green
            assert!(color.is_ok());

            let invalid_color = Color::new_hsl(0, 101, 50);
            assert!(invalid_color.is_err());
        });
    }

    #[test]
    fn test_hsv_to_rgb_conversion() {
        let (r, g, b) = Color::hsv_to_rgb(0, 100, 100);
        assert_eq!((r, g, b), (255, 0, 0)); // Pure red

        let (r, g, b) = Color::hsv_to_rgb(120, 100, 100);
        assert_eq!((r, g, b), (0, 255, 0)); // Pure green
    }

    #[test]
    fn test_hsl_to_rgb_conversion() {
        let (r, g, b) = Color::hsl_to_rgb(0, 100, 50);
        assert_eq!((r, g, b), (255, 0, 0)); // Pure red

        let (r, g, b) = Color::hsl_to_rgb(240, 100, 50);
        assert_eq!((r, g, b), (0, 0, 255)); // Pure blue
    }
}
