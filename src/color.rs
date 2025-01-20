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
                let (r, g, b) = Self::hsv_to_rgb(h, s, v);
                Cow::Owned(ansi::fg_rgb(r, g, b))
            }
            Color::HSL(h, s, l) => {
                let (r, g, b) = Self::hsl_to_rgb(h, s, l);
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

    /// Convert HSV (Hue, Saturation, Value) color values to RGB
    ///
    /// # Arguments
    /// * `h` - Hue angle in degrees (0-360)
    /// * `s` - Saturation percentage (0-100)
    /// * `v` - Value percentage (0-100)
    ///
    /// # Returns
    /// * `(u8, u8, u8)` - RGB color components (0-255)
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

    /// Convert HSL (Hue, Saturation, Lightness) color values to RGB
    ///
    /// # Arguments
    /// * `h` - Hue angle in degrees (0-360)
    /// * `s` - Saturation percentage (0-100)
    /// * `l` - Lightness percentage (0-100)
    ///
    /// # Returns
    /// * `(u8, u8, u8)` - RGB color components (0-255)
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

    /// Convert RGB color values to the nearest 256-color code
    ///
    /// This function maps RGB colors to the 256-color palette used by many terminals.
    /// It handles both the 6x6x6 color cube (216 colors) and the grayscale ramp (24 levels).
    ///
    /// # Arguments
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    ///
    /// # Returns
    /// * `u8` - The nearest 256-color code (16-255)
    pub fn rgb_to_256(r: u8, g: u8, b: u8) -> u8 {
        if r == g && g == b {
            // Grayscale shortcut
            if r == 0 {
                return 16; // black
            }
            if r == 255 {
                return 231; // white
            }
            // Find the nearest grayscale color (24 levels)
            return 232 + ((r as f32 / 255.0 * 23.0).round() as u8);
        }

        // Convert to 6x6x6 color cube indices (0-5 for each component)
        let r = (r as f32 / 255.0 * 5.0).round() as u8;
        let g = (g as f32 / 255.0 * 5.0).round() as u8;
        let b = (b as f32 / 255.0 * 5.0).round() as u8;

        // Calculate the 256-color code
        16 + 36 * r + 6 * g + b
    }

    /// Convert 256-color code to RGB color values
    ///
    /// # Arguments
    /// * `code` - The 256-color code (16-255)
    ///
    /// # Returns
    /// * `(u8, u8, u8)` - RGB color components (0-255)
    pub fn code_to_rgb(code: u8) -> (u8, u8, u8) {
        (code / 36, (code % 36) / 6, code % 6)
    }

    /// Convert RGB color values to the nearest basic ANSI color
    ///
    /// This function maps RGB colors to the 8 basic ANSI colors by analyzing
    /// the relative luminance and dominant color components.
    ///
    /// # Arguments
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    ///
    /// # Returns
    /// * `Color` - The nearest basic ANSI color
    pub fn rgb_to_basic(r: u8, g: u8, b: u8) -> Color {
        // Convert to f32 for calculations
        let r_f = f32::from(r);
        let g_f = f32::from(g);
        let b_f = f32::from(b);

        // Calculate relative luminance
        let luminance = (0.2126 * r_f + 0.7152 * g_f + 0.0722 * b_f) / 255.0;

        // Handle extreme cases (very dark/light)
        if r < 10 && g < 10 && b < 10 {
            return Color::Black;
        }
        if r > 245 && g > 245 && b > 245 {
            return Color::White;
        }

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let diff = max - min;

        // If very low saturation, handle as grayscale
        if diff < 20 {
            return if luminance < 0.5 {
                Color::Black
            } else {
                Color::White
            };
        }

        // Calculate color ratios for better comparison
        let r_ratio = r_f / 255.0;
        let g_ratio = g_f / 255.0;
        let b_ratio = b_f / 255.0;

        // Special case for browns/yellows
        if r > g && g > b {
            // If red is dominant but green is significant
            let g_to_r_ratio = g_f / r_f;

            // More sensitive yellow detection for browns
            if g_to_r_ratio > 0.4 && b < g / 2 {
                return Color::Yellow;
            }
        }

        // Special case for purples/magentas
        if r > 0 && b > 0 && g < r && g < b {
            // If both red and blue are present and green is lower
            let r_to_b_ratio = r_f / b_f;
            let b_to_r_ratio = b_f / r_f;

            // If either red or blue is at least 40% of the other
            if r_to_b_ratio > 0.4 || b_to_r_ratio > 0.4 {
                return Color::Magenta;
            }
        }

        // Special case for cyans
        if g > 0 && b > 0 && r < g && r < b {
            // If both green and blue are present and red is lower
            let g_to_b_ratio = g_f / b_f;
            let b_to_g_ratio = b_f / g_f;

            // For cyan, both components should be more balanced
            if g_to_b_ratio > 0.65 && b_to_g_ratio > 0.65 {
                return Color::Cyan;
            }
        }

        let r_dominant = r_ratio >= g_ratio && r_ratio >= b_ratio;
        let g_dominant = g_ratio >= r_ratio && g_ratio >= b_ratio;
        let b_dominant = b_ratio >= r_ratio && b_ratio >= g_ratio;

        // Check secondary color strengths
        let has_red = r > 64;
        let has_green = g > 64;
        let has_blue = b > 64;

        match (r_dominant, g_dominant, b_dominant) {
            (true, false, false) => {
                if has_green && g > (r / 3) {
                    Color::Yellow
                } else {
                    Color::Red
                }
            }
            (false, true, false) => {
                if has_blue && b > (g / 3) {
                    Color::Cyan
                } else {
                    Color::Green
                }
            }
            (false, false, true) => {
                // If blue is dominant and green is less than 65% of blue, it's blue
                if g_f / b_f < 0.65 {
                    Color::Blue
                } else if has_red && r > (b / 3) {
                    Color::Magenta
                } else {
                    Color::Cyan
                }
            }
            _ => {
                if r > 128 && g > 128 && b < 128 {
                    Color::Yellow
                } else if r > 128 && b > 128 && g < 128 {
                    Color::Magenta
                } else if g > 128 && b > 128 && r < 128 {
                    Color::Cyan
                } else if luminance > 0.6 {
                    Color::White
                } else {
                    Color::Black
                }
            }
        }
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

    #[test]
    fn test_rgb_to_256() {
        // Test grayscale colors
        assert_eq!(Color::rgb_to_256(0, 0, 0), 16); // Black
        assert_eq!(Color::rgb_to_256(255, 255, 255), 231); // White
        assert_eq!(Color::rgb_to_256(128, 128, 128), 244); // Mid-gray
        assert_eq!(Color::rgb_to_256(32, 32, 32), 235); // Dark gray
        assert_eq!(Color::rgb_to_256(220, 220, 220), 252); // Light gray

        // Test primary colors
        assert_eq!(Color::rgb_to_256(255, 0, 0), 196); // Pure red
        assert_eq!(Color::rgb_to_256(0, 255, 0), 46); // Pure green
        assert_eq!(Color::rgb_to_256(0, 0, 255), 21); // Pure blue

        // Test mixed colors
        assert_eq!(Color::rgb_to_256(255, 255, 0), 226); // Yellow
        assert_eq!(Color::rgb_to_256(255, 0, 255), 201); // Magenta
        assert_eq!(Color::rgb_to_256(0, 255, 255), 51); // Cyan

        // Test edge cases
        assert_eq!(Color::rgb_to_256(1, 1, 1), 232); // Almost black
        assert_eq!(Color::rgb_to_256(254, 254, 254), 255); // Almost white
        assert_eq!(Color::rgb_to_256(127, 127, 127), 243); // Perfect mid-gray

        // Test color cube boundaries
        assert_eq!(Color::rgb_to_256(51, 0, 0), 52); // Dark red boundary
        assert_eq!(Color::rgb_to_256(102, 0, 0), 88); // Medium red boundary
        assert_eq!(Color::rgb_to_256(204, 0, 0), 160); // Bright red boundary

        // Test mixed values
        assert_eq!(Color::rgb_to_256(128, 64, 32), 131); // Brown
        assert_eq!(Color::rgb_to_256(70, 130, 180), 74); // Steel Blue
        assert_eq!(Color::rgb_to_256(85, 107, 47), 101); // Dark Olive Green
        assert_eq!(Color::rgb_to_256(219, 112, 147), 175); // Pale Violet Red
    }

    #[test]
    fn test_rgb_to_basic() {
        // Test primary colors
        assert_eq!(Color::rgb_to_basic(255, 0, 0), Color::Red);
        assert_eq!(Color::rgb_to_basic(0, 255, 0), Color::Green);
        assert_eq!(Color::rgb_to_basic(0, 0, 255), Color::Blue);

        // Test secondary colors
        assert_eq!(Color::rgb_to_basic(255, 255, 0), Color::Yellow);
        assert_eq!(Color::rgb_to_basic(255, 0, 255), Color::Magenta);
        assert_eq!(Color::rgb_to_basic(0, 255, 255), Color::Cyan);

        // Test black and white
        assert_eq!(Color::rgb_to_basic(0, 0, 0), Color::Black);
        assert_eq!(Color::rgb_to_basic(255, 255, 255), Color::White);

        // Test colors with dominant components
        assert_eq!(Color::rgb_to_basic(200, 60, 60), Color::Red);
        assert_eq!(Color::rgb_to_basic(60, 200, 60), Color::Green);
        assert_eq!(Color::rgb_to_basic(60, 60, 200), Color::Blue);

        // Test mixed colors with two dominant components
        assert_eq!(Color::rgb_to_basic(200, 200, 60), Color::Yellow);
        assert_eq!(Color::rgb_to_basic(200, 60, 200), Color::Magenta);
        assert_eq!(Color::rgb_to_basic(60, 200, 200), Color::Cyan);

        // Test dark mixed colors
        assert_eq!(Color::rgb_to_basic(100, 50, 50), Color::Red);
        assert_eq!(Color::rgb_to_basic(50, 100, 50), Color::Green);
        assert_eq!(Color::rgb_to_basic(50, 50, 100), Color::Blue);
    }
}
