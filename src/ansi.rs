//! ANSI escape codes for terminal text formatting
//!
//! This module provides constants and utilities for ANSI escape sequences used for text formatting
//! in terminal environments. These codes are used internally by the library to
//! apply various text styles and colors.
//!
//! # Note
//! These codes may not work in all terminal environments. Use the `check_color_support()`
//! function to verify terminal capabilities.

// Base ANSI components
const ESC_BASE: &str = "\x1b[";
const SUFFIX: &str = "m";

// Basic formatting codes
#[allow(dead_code)]
const RESET_CODE: &str = "0";
#[allow(dead_code)]
const BOLD_CODE: &str = "1";
#[allow(dead_code)]
const DIM_CODE: &str = "2";
#[allow(dead_code)]
const ITALIC_CODE: &str = "3";
#[allow(dead_code)]
const UNDERLINE_CODE: &str = "4";

// Color base codes
#[allow(dead_code)]
const FG_BASE: &str = "3";  // 30-37 for foreground
#[allow(dead_code)]
const BG_BASE: &str = "4";  // 40-47 for background

// RGB color base codes
const RGB_FG_BASE: &str = "38";
const RGB_BG_BASE: &str = "48";

// Basic formatting
pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const ITALIC: &str = "\x1b[3m";
pub const UNDERLINE: &str = "\x1b[4m";

// Basic foreground colors
pub const FG_BLACK: &str = "\x1b[30m";
pub const FG_RED: &str = "\x1b[31m";
pub const FG_GREEN: &str = "\x1b[32m";
pub const FG_YELLOW: &str = "\x1b[33m";
pub const FG_BLUE: &str = "\x1b[34m";
pub const FG_MAGENTA: &str = "\x1b[35m";
pub const FG_CYAN: &str = "\x1b[36m";
pub const FG_WHITE: &str = "\x1b[37m";

// Basic background colors
pub const BG_BLACK: &str = "\x1b[40m";
pub const BG_RED: &str = "\x1b[41m";
pub const BG_GREEN: &str = "\x1b[42m";
pub const BG_YELLOW: &str = "\x1b[43m";
pub const BG_BLUE: &str = "\x1b[44m";
pub const BG_MAGENTA: &str = "\x1b[45m";
pub const BG_CYAN: &str = "\x1b[46m";
pub const BG_WHITE: &str = "\x1b[47m";

/// Creates an RGB foreground color code
pub(crate) fn fg_rgb(r: u8, g: u8, b: u8) -> String {
    format!("{}{};2;{};{};{}{}", ESC_BASE, RGB_FG_BASE, r, g, b, SUFFIX)
}

/// Creates an RGB background color code
pub(crate) fn bg_rgb(r: u8, g: u8, b: u8) -> String {
    format!("{}{};2;{};{};{}{}", ESC_BASE, RGB_BG_BASE, r, g, b, SUFFIX)
}