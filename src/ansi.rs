//! ANSI escape codes for terminal text formatting
//! 
//! This module provides constants for ANSI escape sequences used for text formatting
//! in terminal environments. These codes are used internally by the library to
//! apply various text styles.
//! 
//! # Note
//! These codes may not work in all terminal environments. Use the `check_color_support()`
//! function to verify terminal capabilities.

/// Reset all text attributes to default
pub const RESET: &str = "\x1b[0m";

/// Make text bold or increase intensity
pub const BOLD: &str = "\x1b[1m";

/// Decrease text intensity
/// 
/// Note: Not supported in all terminals
pub const DIM: &str = "\x1b[2m";

/// Make text italic
/// 
/// Note: Not supported in all terminals
pub const ITALIC: &str = "\x1b[3m";

/// Underline text
pub const UNDERLINE: &str = "\x1b[4m"; 