# Inksac
[![rust-fmt](https://github.com/SkuldNorniern/inksac-rs/actions/workflows/rust-fmt.yml/badge.svg)](https://github.com/SkuldNorniern/inksac-rs/actions/workflows/rust-fmt.yml)
[![rust-clippy](https://github.com/SkuldNorniern/inksac-rs/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/SkuldNorniern/inksac-rs/actions/workflows/rust-clippy.yml)
[![rust-audit](https://github.com/SkuldNorniern/inksac-rs/actions/workflows/rust-audit.yml/badge.svg)](https://github.com/SkuldNorniern/inksac-rs/actions/workflows/rust-audit.yml)

Inksac is a Rust library that amplifies terminal outputs by offering a seamless integration with ANSI color support. It is devised to be uncomplicated, flexible, and delightful to utilize, adding a splash of color to your terminal applications.

## Features

1. **Check ANSI Color Support**: Before painting your terminal with vibrant colors, ascertain if the current terminal supports ANSI colors using Inksac's built-in function.

2. **Customizable Styles**: Inksac allows the creation of distinct text styles with customizable foreground and background colors, giving a personal touch to your terminal display.

3. **Vivid Text Outputs**: Enrich your terminal applications with colorful text outputs using the `ColoredString` structure.

4. **True Color Support**: Experience the diversity of colors with Inksac supporting true color by RGB and HEX values.

5. **Text Format Options**: Beautify your text with bold, dim, italic, underline formats. Stay tuned for more format options in future releases.

## Usage

Here is a basic example demonstrating how to utilize the Inksac crate:

```rust
use inksac::is_color_available;
use inksac::types::{Color, ColoredString, Style};

fn main() {
    // Check if the terminal supports ANSI colors
    match is_color_available() {
        Ok(_) => println!("Terminal supports ANSI colors"),
        Err(err) => println!("{}", err),
    }

    // Create a style using the new builder pattern
    let title_style = Style::builder()
        .foreground(Some(Color::Green))
        .background(Some(Color::Red))
        .bold()
        .build();

    // Create a ColoredString with the predefined style
    let title_text = ColoredString::new("Hello World", title_style);

    // Print the ColoredString
    println!("{}", title_text);
}
```

## Installation

To add Inksac as a dependency to your project, run the following command:

```sh
cargo add inksac
```

## Disclaimer

Please note that the actual color representation may vary based on the terminal's capabilities and might not function as anticipated on all platforms.

---
Happy Coding!

