//! A type-safe terminal text styling library with color support
//! 
//! `inksac` provides a safe and ergonomic way to add colors and styles to terminal text output.
//! It automatically detects terminal capabilities and handles fallbacks gracefully.
//!
//! # Key Features
//!
//! - Safe color handling with terminal capability detection
//! - Support for 16 colors, 256 colors, and true color (16M colors) 
//! - Composable text styling (bold, italic, underline)
//! - RGB and hex color definitions
//! - Error handling for all operations
//! - Zero unsafe code
//!
//! # Basic Usage
//!
//! ```rust
//! use inksac::{Color, Style, Styleable};
//!
//! // Create a style
//! let style = Style::builder()
//!     .foreground(Color::Green)
//!     .bold()
//!     .build();
//!
//! // Apply style to text
//! let colored_text = "Hello, world!".style(style);
//! println!("{}", colored_text);
//! ```
//!
//! # Advanced Usage
//!
//! ```rust
//! use inksac::{Color, Style, Styleable};
//!
//! // RGB colors (requires true color support)
//! let rgb_style = Style::builder()
//!     .foreground(Color::new_rgb(255, 128, 0).unwrap())
//!     .italic()
//!     .build();
//!
//! // Hex colors
//! let hex_style = Style::builder()
//!     .background(Color::new_hex("#FF8000").unwrap())
//!     .underline()
//!     .build();
//!
//! // Compose styles
//! let text = "Custom colors!"
//!     .style(rgb_style)
//!     .with_style(hex_style);
//! ```
//!
//! # Error Handling
//!
//! The library uses proper error handling throughout:
//!
//! ```rust
//! use inksac::{check_color_support, ColorSupport, ColorError};
//!
//! fn print_colored() -> Result<(), ColorError> {
//!     // Check terminal capabilities
//!     let support = check_color_support()?;
//!     
//!     match support {
//!         ColorSupport::TrueColor => {
//!             // Use RGB colors
//!         }
//!         ColorSupport::Basic => {
//!             // Fallback to basic colors
//!         }
//!         _ => {
//!             // Handle no color support
//!         }
//!     }
//!     
//!     Ok(())
//! }
//! ```

mod ansi;
mod color;
mod style;
mod string;
mod error;


pub use color::Color;
pub use style::{Style, StyleBuilder};
pub use string::{ColoredString, Styleable};
pub use error::{ColorError, ColorSupport};

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
    if std::env::var("NO_COLOR").is_ok() {
        return Ok(ColorSupport::NoColor);
    }

    // Check COLORTERM for true color support
    let colorterm = std::env::var("COLORTERM").unwrap_or_default();
    if colorterm.contains("truecolor") || colorterm.contains("24bit") {
        return Ok(ColorSupport::TrueColor);
    }

    // Check various terminal environment variables
    let term = std::env::var("TERM").unwrap_or_default();
    let clicolor = std::env::var("CLICOLOR").unwrap_or_default();
    let clicolor_force = std::env::var("CLICOLOR_FORCE").unwrap_or_default();
    
    // Force color if CLICOLOR_FORCE is set to 1
    if clicolor_force == "1" {
        return Ok(ColorSupport::Basic);
    }
    
    // Disable color if CLICOLOR is set to 0
    if clicolor == "0" {
        return Ok(ColorSupport::NoColor);
    }

    // Check common terminal types
    if term.contains("256color") || term.contains("256") {
        Ok(ColorSupport::Color256)
    } else if term.contains("color") 
        || term.contains("ansi")
        || term.contains("xterm")
        || term.contains("screen") {
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
///
/// # Examples
/// ```
/// use inksac::{is_color_available, ColorError};
///
/// match is_color_available() {
///     Ok(()) => println!("Terminal supports ANSI colors"),
///     Err(e) => eprintln!("Terminal does not support ANSI colors: {}", e),
/// }
/// ```
pub fn is_color_available() -> Result<(), ColorError> {
    match check_color_support()? {
        ColorSupport::NoColor => Err(ColorError::NoTerminalSupport),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_builder() {
        let style = Style::builder()
            .foreground(Color::Red)
            .background(Color::Blue)
            .bold()
            .italic()
            .build();

        assert_eq!(style.foreground, Color::Red);
        assert_eq!(style.background, Color::Blue);
        assert!(style.bold);
        assert!(style.italic);
    }

    #[test]
    fn test_colored_string() {
        let style = Style::builder()
            .foreground(Color::Green)
            .build();
        
        let colored = "test".style(style);
        assert_eq!(colored.into_string(), "test");
    }

    #[test]
    fn test_rgb_color() {
        let rgb = Color::new_rgb(255, 128, 0);
        assert!(rgb.is_ok());
    }

    #[test]
    fn test_hex_color() {
        let hex = Color::new_hex("#FF8000");
        assert!(hex.is_ok());
    }

    #[test]
    fn test_color_support() {
        let support = check_color_support();
        assert!(support.is_ok());
    }
}
