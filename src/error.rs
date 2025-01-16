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

/// Represents errors that can occur when working with colors
#[derive(Debug)]
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
}

/// Terminal color support levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSupport {
    NoColor,
    Basic,    // 16 colors
    Color256, // 256 colors
    TrueColor, // 16 million colors
}
impl std::fmt::Display for ColorSupport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorSupport::NoColor => write!(f, "No Color"),
            ColorSupport::Basic => write!(f, "Basic"),
            ColorSupport::Color256 => write!(f, "Color256"),
            ColorSupport::TrueColor => write!(f, "TrueColor"),
        }
    }
}
impl std::fmt::Display for ColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorError::NoTerminalSupport => write!(f, "Terminal does not support ANSI colors"),
            ColorError::InvalidHexCode(hex) => write!(f, "Invalid hex color code: {}", hex),
            ColorError::InvalidRGB(msg) => write!(f, "Invalid RGB values: {}", msg),
            ColorError::UnsupportedColorMode(req, avail) => 
                write!(f, "Terminal doesn't support {:?} (available: {:?})", req, avail),
            ColorError::EnvError(e) => write!(f, "Environment error: {}", e),
            ColorError::InvalidOperation(msg) => write!(f, "Invalid operation attempted: {}", msg),
            ColorError::ColorManipulation(msg) => write!(f, "Color manipulation error: {}", msg),
        }
    }
}

impl std::error::Error for ColorError {}

impl From<std::env::VarError> for ColorError {
    fn from(err: std::env::VarError) -> Self {
        ColorError::EnvError(err)
    }
} 