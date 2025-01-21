//! Environment detection and color support functionality
//!
//! This module handles the detection of terminal capabilities and color support levels.
//! It provides functions to check the current environment and determine what color
//! features are available.

use crate::error::ColorError;
use std::env;

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
    // Handle NO_COLOR first as it takes absolute precedence
    if env::var("NO_COLOR").is_ok() {
        return Ok(ColorSupport::NoColor);
    }

    let clicolor = env::var("CLICOLOR").unwrap_or_default();
    if clicolor == "0" {
        return Ok(ColorSupport::NoColor);
    }

    let mut support = ColorSupport::NoColor;

    // Check COLORTERM for true color support
    let colorterm = env::var("COLORTERM").unwrap_or_default();
    if colorterm.contains("truecolor") || colorterm.contains("24bit") {
        support = ColorSupport::TrueColor;
    }

    let term = env::var("TERM").unwrap_or_default().to_lowercase();

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
        support = ColorSupport::TrueColor;
    }

    // Check TERM_PROGRAM for specific terminals that support true color
    let term_program = env::var("TERM_PROGRAM").unwrap_or_default();
    if ["iTerm.app", "Apple_Terminal", "Hyper"].contains(&term_program.as_str()) {
        support = ColorSupport::TrueColor;
    }

    // If no true color support was detected, check for 256 colors or basic colors
    if support == ColorSupport::NoColor {
        if term.contains("256color") || term.contains("256") {
            support = ColorSupport::Color256;
        } else if term.contains("color")
            || term.contains("ansi")
            || term.contains("xterm")
            || term.contains("screen")
        {
            support = ColorSupport::Basic;
        }
    }

    // Handle CLICOLOR_FORCE after determining actual support level
    let clicolor_force = env::var("CLICOLOR_FORCE").unwrap_or_default();
    if clicolor_force == "1" {
        // If no support was detected but CLICOLOR_FORCE is set, use Basic
        // Otherwise, keep the highest detected support level
        if support == ColorSupport::NoColor {
            support = ColorSupport::Basic;
        }
    }

    Ok(support)
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
pub(crate) mod tests {
    use super::*;
    use crate::Color;

    pub(crate) fn run_with_env_vars<F, T>(vars: &[(&str, Option<&str>)], test: F) -> T
    where
        F: FnOnce() -> T,
    {
        // Store original env vars
        let original: Vec<(String, Option<String>)> = vars
            .iter()
            .map(|(k, _)| (k.to_string(), env::var(k).ok()))
            .collect();

        // Check if we're running in a CI environment
        let is_ci = env::var("CI").is_ok()
            || env::var("GITHUB_ACTIONS").is_ok()
            || env::var("GITLAB_CI").is_ok()
            || env::var("TRAVIS").is_ok()
            || env::var("CIRCLECI").is_ok();

        // If in CI, always set color support
        if is_ci {
            env::set_var("COLORTERM", "truecolor");
            env::set_var("TERM", "xterm-256color");
            env::remove_var("NO_COLOR");
        }

        // Set test-specific env vars
        for (key, value) in vars {
            match value {
                Some(v) => env::set_var(key, v),
                None => env::remove_var(key),
            }
        }

        // Run the test
        let result = test();

        // Restore original env vars
        for (key, value) in original {
            match value {
                Some(v) => env::set_var(&key, v),
                None => env::remove_var(&key),
            }
        }

        result
    }

    #[test]
    fn test_all_color_scenarios() {
        // Test color support
        run_with_env_vars(
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

        // Test no color environment
        run_with_env_vars(
            &[("NO_COLOR", Some("")), ("TERM", None), ("COLORTERM", None)],
            || {
                let support = check_color_support().expect("Color support check failed");
                assert_eq!(support, ColorSupport::NoColor);
            },
        );

        // Test CLICOLOR_FORCE
        run_with_env_vars(
            &[
                ("NO_COLOR", None),
                ("COLORTERM", None),
                ("TERM", None),
                ("CLICOLOR_FORCE", Some("1")),
                ("CLICOLOR", None),
            ],
            || {
                let support = check_color_support().expect("Color support check failed");
                assert_eq!(support, ColorSupport::Basic);
            },
        );

        // Test CLICOLOR disable
        run_with_env_vars(
            &[
                ("CLICOLOR", Some("0")),
                ("NO_COLOR", None),
                ("COLORTERM", None),
                ("TERM", None),
                ("CLICOLOR_FORCE", None),
            ],
            || {
                let support = check_color_support().expect("Color support check failed");
                assert_eq!(support, ColorSupport::NoColor);
            },
        );

        // Test RGB color
        run_with_env_vars(
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

        // Test HEX color
        run_with_env_vars(
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
