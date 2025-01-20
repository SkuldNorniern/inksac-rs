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

mod basic;
mod convert;
mod manipulation;

pub use self::basic::Color;
