//! Error types and handling for color operations
//!
//! This module provides the error types used throughout the library for
//! handling color-related failures in a type-safe way.
//!
//! # Examples
//!
//! ```rust
//! use inksac::{Color, ColorError};
//!
//! fn try_color() -> Result<(), ColorError> {
//!     // Attempt color operations
//!     let rgb = Color::new_rgb(255, 128, 0)?;
//!     let hex = Color::new_hex("#FF8000")?;
//!     
//!     Ok(())
//! }
//! ```

use crate::env::ColorSupport;
/// Represents errors that can occur when working with colors
#[derive(Debug, Eq, PartialEq)]
pub enum ColorError {
    /// Terminal does not support ANSI colors
    NoTerminalSupport,
    /// Invalid hex color code provided
    InvalidHexCode(String),
    /// Failed to parse RGB values
    InvalidRGB(String),
    /// Terminal doesn't support requested color mode
    UnsupportedColorMode(ColorSupport, ColorSupport), // (requested, available)
    /// Environment variable error
    EnvError(std::env::VarError),
    /// Invalid operation attempted
    InvalidOperation(String),
    /// Color manipulation error
    ColorManipulation(String),
    /// Invalid color value
    InvalidColorValue(String),
    ColorConversionError {
        from: &'static str,
        to: &'static str,
        reason: String,
    },
    ComponentOutOfRange {
        component: &'static str,
        value: &'static str,   
        min: &'static str,
        max: &'static str,
    },
    /// Color component validation error
    ValidationError {
        component: &'static str,
        value: String,
        reason: &'static str,
    },
    /// Terminal capability error
    TerminalError {
        requested: &'static str,
        available: Option<&'static str>,
        reason: String,
    },
    /// Error during color space conversion
    ColorSpaceConversion {
        from: &'static str,
        to: &'static str,
        reason: String,
    },
    /// Mathematical error during color calculations
    ColorCalculation(String),
    /// Error during color interpolation
    InterpolationError(String),
}

impl std::fmt::Display for ColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorError::NoTerminalSupport => write!(f, "Terminal does not support ANSI colors"),
            ColorError::InvalidHexCode(hex) => write!(f, "Invalid hex color code: {}", hex),
            ColorError::InvalidRGB(msg) => write!(f, "Invalid RGB values: {}", msg),
            ColorError::UnsupportedColorMode(req, avail) => {
                write!(f, "Terminal doesn't support {} (available: {})", req, avail)
            }
            ColorError::EnvError(e) => write!(f, "Environment error: {}", e),
            ColorError::InvalidOperation(msg) => write!(f, "Invalid operation attempted: {}", msg),
            ColorError::ColorManipulation(msg) => write!(f, "Color manipulation error: {}", msg),
            ColorError::InvalidColorValue(msg) => write!(f, "Invalid color value: {}", msg),
            ColorError::ColorConversionError { from, to, reason } => {
                write!(f, "Color conversion error: {} to {} failed: {}", from, to, reason)
            }
            ColorError::ComponentOutOfRange { component, value, min, max } => {
                write!(f, "Component {} out of range: {} is not between {} and {}", component, value, min, max)
            }
            ColorError::ValidationError { component, value, reason } => {
                write!(f, "Validation error: {} is invalid: {}\n Reason: {}", component, value, reason)
            }
            ColorError::TerminalError { requested, available, reason } => {
                write!(f, "Terminal error: {} is not supported: {}\n Available: {}", requested, reason, available.unwrap_or("none"))
            }
            ColorError::ColorSpaceConversion { from, to, reason } => {
                write!(f, "Color space conversion error: {} to {} failed: {}", from, to, reason)
            }
            ColorError::ColorCalculation(msg) => write!(f, "Color calculation error: {}", msg),
            ColorError::InterpolationError(msg) => write!(f, "Color interpolation error: {}", msg),
        }
    }
}

impl std::error::Error for ColorError {}

impl From<std::env::VarError> for ColorError {
    fn from(err: std::env::VarError) -> Self {
        ColorError::EnvError(err)
    }
}


impl From<std::num::ParseFloatError> for ColorError {
    fn from(err: std::num::ParseFloatError) -> Self {
        ColorError::ColorCalculation(err.to_string())
    }
}
