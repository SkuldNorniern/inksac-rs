//! Basic color enum
//!
//! This enum represents the basic colors supported by ANSI escape codes.
//! It includes the 8 basic colors and an empty color.
//!
//! # Examples
//!
//! ```rust
//! use inksac::Color;
//!
//! let red = Color::Red;
//! ```
//!  

use crate::ansi;
use crate::check_color_support;
use crate::env::ColorSupport;
use crate::error::ColorError;
use std::borrow::Cow;

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
    Color256(u8),
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
        let support = check_color_support()?;
        match support {
            ColorSupport::TrueColor => Ok(Color::RGB(r, g, b)),
            _ => Err(ColorError::UnsupportedColorMode(
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

        let support = check_color_support()?;
        match support {
            ColorSupport::TrueColor => Ok(Color::HEX(hex)),
            _ => Err(ColorError::UnsupportedColorMode(
                ColorSupport::TrueColor,
                support,
            )),
        }
    }

    pub fn validate_hex(hex: &str) -> Result<(u8, u8, u8), ColorError> {
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

    /// Convert a color to its foreground ANSI escape sequence
    ///
    /// This internal function converts the color to the appropriate ANSI escape sequence
    /// for foreground (text) coloring. It handles all color variants including basic ANSI,
    /// RGB, HEX, HSV, and HSL.
    ///
    /// # Returns
    /// * `Cow<'static, str>` - The ANSI escape sequence for the color
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
            Color::Color256(code) => Cow::Owned(ansi::fg_256(code)),
            Color::HEX(code) => {
                let (r, g, b) = Self::validate_hex(code)
                    .expect("Invalid hex code - this should be validated at construction");
                Cow::Owned(ansi::fg_rgb(r, g, b))
            }
            Color::HSV(h, s, v) => {
                let (r, g, b) = Self::hsv_to_rgb(h, s, v).expect(
                    "Failed to convert HSV to RGB - this should be validated at construction",
                );
                Cow::Owned(ansi::fg_rgb(r, g, b))
            }
            Color::HSL(h, s, l) => {
                let (r, g, b) = Self::hsl_to_rgb(h, s, l).expect(
                    "Failed to convert HSL to RGB - this should be validated at construction",
                );
                Cow::Owned(ansi::fg_rgb(r, g, b))
            }
        }
    }

    /// Convert a color to its background ANSI escape sequence
    ///
    /// This internal function converts the color to the appropriate ANSI escape sequence
    /// for background coloring. It handles all color variants including basic ANSI,
    /// RGB, HEX, HSV, and HSL.
    ///
    /// # Returns
    /// * `Cow<'static, str>` - The ANSI escape sequence for the color
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
            Color::Color256(code) => Cow::Owned(ansi::bg_256(code)),
            Color::HEX(code) => {
                let (r, g, b) = Self::validate_hex(code)
                    .expect("Invalid hex code - this should be validated at construction");
                Cow::Owned(ansi::bg_rgb(r, g, b))
            }
            Color::HSV(h, s, v) => {
                let (r, g, b) = Self::hsv_to_rgb(h, s, v).expect(
                    "Failed to convert HSV to RGB - this should be validated at construction",
                );
                Cow::Owned(ansi::bg_rgb(r, g, b))
            }
            Color::HSL(h, s, l) => {
                let (r, g, b) = Self::hsl_to_rgb(h, s, l).expect(
                    "Failed to convert HSL to RGB - this should be validated at construction",
                );
                Cow::Owned(ansi::bg_rgb(r, g, b))
            }
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

        let support = check_color_support()?;
        match support {
            ColorSupport::TrueColor => Ok(Color::HSV(h, s, v)),
            _ => Err(ColorError::UnsupportedColorMode(
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

        let support = check_color_support()?;
        match support {
            ColorSupport::TrueColor => Ok(Color::HSL(h, s, l)),
            _ => Err(ColorError::UnsupportedColorMode(
                ColorSupport::TrueColor,
                support,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::tests::run_with_env_vars;

    #[test]
    fn test_new_rgb() {
        run_with_env_vars(&[
            ("COLORTERM", Some("truecolor")),
            ("TERM", Some("xterm-256color")),
            ("NO_COLOR", None),
        ], || {
            let color = Color::new_rgb(255, 128, 0).unwrap();
            if let Color::RGB(r, g, b) = color {
                assert_eq!(r, 255);
                assert_eq!(g, 128);
                assert_eq!(b, 0);
            }
        });
    }

    #[test]
    fn test_new_hex() {
        run_with_env_vars(&[
            ("COLORTERM", Some("truecolor")),
            ("TERM", Some("xterm-256color")),
            ("NO_COLOR", None),
        ], || {
            let color = Color::new_hex("#FF8000").unwrap();
            if let Color::HEX(hex) = color {
                assert_eq!(hex, "#FF8000");
            }
        });
    }
    // ... rest of the tests ...
}
