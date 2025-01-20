//! Environment detection and color support functionality
//!
//! This module handles the detection of terminal capabilities and color support levels.
//! It provides functions to check the current environment and determine what color
//! features are available.

use std::env;
use crate::error::ColorError;

/// Terminal color support levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorSupport {
    NoColor = 0,
    Basic = 1,     // 16 colors
    Color256 = 2,  // 256 colors
    TrueColor = 3, // 16 million colors
}

impl ColorSupport {
    /// Check if this support level can handle the requested level
    pub fn supports(&self, required: ColorSupport) -> bool {
        *self >= required
    }
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

/// Check the level of color support in the current terminal
///
/// # Returns
/// - `Ok(ColorSupport)` indicating the level of color support
/// - `Err(ColorError)` if the terminal environment cannot be detected
///
/// # Examples
/// ```
/// use inksac::{check_color_support, ColorSupport};
///
/// match check_color_support() {
///     Ok(support) => match support {
///         ColorSupport::TrueColor => println!("Terminal supports true color"),
///         ColorSupport::Color256 => println!("Terminal supports 256 colors"),
///         ColorSupport::Basic => println!("Terminal supports basic colors"),
///         ColorSupport::NoColor => println!("Terminal does not support colors"),
///     },
///     Err(e) => eprintln!("Failed to detect color support: {}", e),
/// }
/// ```
pub fn check_color_support() -> Result<ColorSupport, ColorError> {
    // Check NO_COLOR environment variable first (https://no-color.org/)
    if env::var("NO_COLOR").is_ok() {
        return Ok(ColorSupport::NoColor);
    }

    // Check COLORTERM for true color support
    let colorterm = env::var("COLORTERM").unwrap_or_default();
    if colorterm.contains("truecolor") || colorterm.contains("24bit") {
        return Ok(ColorSupport::TrueColor);
    }

    // Check TERM_PROGRAM for specific terminals that support true color
    let term_program = env::var("TERM_PROGRAM").unwrap_or_default();
    if ["iTerm.app", "Apple_Terminal", "Hyper"].contains(&term_program.as_str()) {
        return Ok(ColorSupport::TrueColor);
    }

    // Check various terminal environment variables
    let term = env::var("TERM").unwrap_or_default().to_lowercase();
    let clicolor = env::var("CLICOLOR").unwrap_or_default();
    let clicolor_force = env::var("CLICOLOR_FORCE").unwrap_or_default();

    // Force color if CLICOLOR_FORCE is set to 1
    if clicolor_force == "1" {
        return Ok(ColorSupport::Basic);
    }

    // Disable color if CLICOLOR is set to 0
    if clicolor == "0" {
        return Ok(ColorSupport::NoColor);
    }

    // List of terminals that support true color
    let truecolor_terms = [
        "xterm-truecolor",
        "konsole",
        "tmux",
        "screen-truecolor",
        "alacritty",
        "kitty",
        "terminator",
        "terminology",
        "eterm",
        "rxvt-unicode",
        "xterm-ghostty",
        "vte",
        "termious",
    ];
    if truecolor_terms.iter().any(|&t| term.contains(t)) {
        return Ok(ColorSupport::TrueColor);
    }

    // Check for 256 color support
    if term.contains("256color") || term.contains("256") {
        Ok(ColorSupport::Color256)
    } else if term.contains("color")
        || term.contains("ansi")
        || term.contains("xterm")
        || term.contains("screen")
    {
        Ok(ColorSupport::Basic)
    } else {
        Ok(ColorSupport::NoColor)
    }
}

/// Check if the terminal supports ANSI colors
///
/// # Returns
/// - `Ok(())` if the terminal supports ANSI colors
/// - `Err(ColorError::NoTerminalSupport)` if the terminal does not support ANSI colors
pub fn is_color_available() -> Result<(), ColorError> {
    match check_color_support()? {
        ColorSupport::NoColor => Err(ColorError::NoTerminalSupport),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;
    /// Helper function to run tests with a controlled environment
    fn with_env_vars<F, T>(vars: &[(&str, Option<&str>)], test: F) -> T
    where
        F: FnOnce() -> T,
    {
        // List of all color-related environment variables we need to control
        let color_vars = [
            "NO_COLOR",
            "COLORTERM",
            "TERM",
            "TERM_PROGRAM",
            "CLICOLOR",
            "CLICOLOR_FORCE",
        ];

        // Store original values
        let original: Vec<(String, Option<String>)> = color_vars
            .iter()
            .map(|&name| (name.to_string(), env::var(name).ok()))
            .collect();

        // Clear all color-related environment variables
        for var in &color_vars {
            env::remove_var(var);
        }

        // Set test-specific variables
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
    fn test_color_support() {
        with_env_vars(
            &[
                ("NO_COLOR", None),
                ("COLORTERM", Some("truecolor")),
                ("TERM", Some("xterm-256color")),
            ],
            || {
                let support = check_color_support();
                assert!(support.is_ok());
            },
        );
    }

    #[test]
    fn test_no_color_env() {
        let support = with_env_vars(
            &[("NO_COLOR", Some("")), ("TERM", None), ("COLORTERM", None)],
            || check_color_support().expect("Color support check failed"),
        );
        assert_eq!(support, ColorSupport::NoColor);
    }

    #[test]
    fn test_true_color_support() {
        let support = with_env_vars(
            &[("NO_COLOR", None), ("COLORTERM", Some("truecolor"))],
            || check_color_support().expect("Color support check failed"),
        );
        assert_eq!(support, ColorSupport::TrueColor);

        let support = with_env_vars(&[("NO_COLOR", None), ("COLORTERM", Some("24bit"))], || {
            check_color_support().expect("Color support check failed")
        });
        assert_eq!(support, ColorSupport::TrueColor);
    }

    #[test]
    fn test_256_color_support() {
        let support = with_env_vars(
            &[
                ("NO_COLOR", None),
                ("COLORTERM", None),
                ("TERM", Some("xterm-256color")),
            ],
            || check_color_support().expect("Color support check failed"),
        );
        assert_eq!(support, ColorSupport::Color256);
    }

    #[test]
    fn test_basic_color_support() {
        let support = with_env_vars(
            &[
                ("NO_COLOR", None),
                ("COLORTERM", None),
                ("TERM", Some("xterm")),
                ("TERM_PROGRAM", Some("test")),
                ("CLICOLOR", Some("1")),
                ("CLICOLOR_FORCE", Some("1")),
            ],
            || check_color_support().expect("Color support check failed"),
        );
        assert_eq!(support, ColorSupport::Basic);
    }

    #[test]
    fn test_clicolor_force() {
        let support = with_env_vars(
            &[
                ("CLICOLOR_FORCE", Some("1")),
                ("NO_COLOR", None),
                ("COLORTERM", None),
                ("TERM", None),
            ],
            || check_color_support().expect("Color support check failed"),
        );
        assert_eq!(support, ColorSupport::Basic);
    }

    #[test]
    fn test_clicolor_disable() {
        let support = with_env_vars(
            &[
                ("CLICOLOR", Some("0")),
                ("NO_COLOR", None),
                ("COLORTERM", None),
                ("TERM", Some("xterm-256color")),
                ("CLICOLOR_FORCE", None),
            ],
            || check_color_support().expect("Color support check failed"),
        );
        assert_eq!(support, ColorSupport::NoColor);
    }

    #[test]
    fn test_no_term_env() {
        let support = with_env_vars(
            &[("NO_COLOR", None), ("COLORTERM", None), ("TERM", None)],
            || check_color_support().expect("Color support check failed"),
        );
        assert_eq!(support, ColorSupport::NoColor);
    }

    #[test]
    fn test_rgb_color() {
        with_env_vars(
            &[
                ("NO_COLOR", None),
                ("COLORTERM", Some("truecolor")),
                ("TERM", Some("xterm-256color")),
            ],
            || {
                let rgb = Color::new_rgb(255, 128, 0);
                assert!(rgb.is_ok());
            },
        );
    }

    #[test]
    fn test_hex_color() {
        with_env_vars(
            &[
                ("NO_COLOR", None),
                ("COLORTERM", Some("truecolor")),
                ("TERM", Some("xterm-256color")),
            ],
            || {
                let hex = Color::new_hex("#FF8000");
                assert!(hex.is_ok());
            },
        );
    }
}


