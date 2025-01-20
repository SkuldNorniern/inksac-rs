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
mod env;
mod error;
mod string;
mod style;

// Add prelude module
pub mod prelude {
    pub use crate::color::Color;
    pub use crate::env::{check_color_support, is_color_available, ColorSupport};
    pub use crate::error::ColorError;
    pub use crate::string::{ColoredString, Styleable};
    pub use crate::style::{Style, StyleBuilder};
}

// Keep existing pub use statements for backward compatibility
pub use color::Color;
pub use env::{check_color_support, is_color_available, ColorSupport};
pub use error::ColorError;
pub use string::{ColoredString, Styleable};
pub use style::{Style, StyleBuilder};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::tests::run_with_env_vars;

    #[test]
    fn test_style_builder() {
        run_with_env_vars(&[("COLORTERM", Some("truecolor"))], || {
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
        });
    }

    #[test]
    fn test_colored_string() {
        run_with_env_vars(&[("COLORTERM", Some("truecolor"))], || {
            let style = Style::builder().foreground(Color::Green).build();
            let colored = "test".style(style);
            assert_eq!(colored.into_string(), "test");
        });
    }
}
