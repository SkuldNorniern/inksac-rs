//! Component-Based Terminal Colorization Library
//!
//! This crate facilitates the coloring of terminal text output through a component-based system.
//! You can define various styles and apply them to strings, then print them to the terminal,
//! all while enjoying type safety and composability.
//!
//! ## Steps to Use
//!
//! 1. **Predefine Your Styles**: Before anything else, set up your styles using [`Style`](crate::types::Style) or with the builder pattern using [`StyleBuilder`](crate::types::StyleBuilder).
//! 2. **Create Colored Strings**: Using the predefined styles, create colored strings with [`ColoredString`](crate::types::ColoredString), or use the [`Stylish`](crate::types::Stylish) trait.
//! 3. **Print the Colored String**: Print the `ColoredString` instances just as you would with regular strings.
//!
//! ## Example
//!
//! Below is an example that demonstrates the usage of this crate, including utilizing the builder pattern for creating styles:
//!
//! ```rust
//! use inksac::is_color_available;
//! use inksac::types::*;
//!
//! match is_color_available() {
//!     Ok(_) => println!("Terminal supports ANSI colors"),
//!     Err(_) => println!("Terminal does not support ANSI colors"),
//! }
//!
//! // Step 1: Predefine Your Styles using the builder pattern
//! let title_style = Style::builder()
//!     .foreground(Color::Green)
//!     .background(Color::Red)
//!     .underline()
//!     .build();
//!
//! // Step 2: Create Colored Strings
//! let title_text: ColoredString = ColoredString::new(
//!     "Hello World",
//!     title_style,
//! );
//!
//! // Step 3: Print the Colored String
//! println!("{}", title_text);
//! ```
//!
//! Please make sure your terminal supports ANSI colors by using the [`is_color_available`] function before attempting to print colored text.
//!
mod ansi_base;
pub mod types;

pub use ansi_base::is_color_available;
