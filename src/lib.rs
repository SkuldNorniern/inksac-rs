//! Component Based Terminal Colorization
//!
//! # Usage Steps
//!
//! 1. Predefine your [Styles](crate::types::Style)
//! 2. Create a [ColoredString](crate::types::ColoredString) with the predefined style
//! 3. print the ColoredString just like using a normal String
//!
//!
//! # Example
//!
//!
//! ```rust
//! use inksac::check_color_available;
//! use inksac::types::*;
//!
//! match check_color_available() {
//!     Ok(_) => println!("Terminal supports ANSI colors"),
//!     Err(_) => println!("Terminal does not support ANSI colors"),
//! }
//!
//! // Step 1
//! const TITLESTYLE: Style = Style{
//!     forground: Some(Color::Green),
//!     background: Some(Color::Red),
//!     bold: false,
//!     dim: false,
//!     italic: false,
//!     underline: false
//! };
//! // Step 2
//! let title_text: ColoredString = ColoredString::new(
//!     "Hello World",
//!     TITLESTYLE
//! );
//!
//! // Step 3
//! println!("{}", title_text);
//! ```

mod ansi_base;
pub mod types;

pub use ansi_base::check_color_available;
