# Inksac
[![rust-fmt](https://github.com/SkuldNorniern/inksac-rs/actions/workflows/rust-fmt.yml/badge.svg)](https://github.com/SkuldNorniern/inksac-rs/actions/workflows/rust-fmt.yml)
[![rust-clippy](https://github.com/SkuldNorniern/inksac-rs/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/SkuldNorniern/inksac-rs/actions/workflows/rust-clippy.yml)
[![rust-audit](https://github.com/SkuldNorniern/inksac-rs/actions/workflows/rust-audit.yml/badge.svg)](https://github.com/SkuldNorniern/inksac-rs/actions/workflows/rust-audit.yml)

Inksac is a Rust library that amplifies terminal outputs by offering a seamless integration with ANSI color support. It is designed to be uncomplicated, flexible, and delightful to use, adding a splash of color to your terminal applications.

## Features

1. **ANSI Color Support Detection**: Automatically detect terminal color support levels including True Color support.

2. **Rich Text Styling**:
   - Foreground and background colors
   - Basic colors (Black, Red, Green, Yellow, Blue, Magenta, Cyan, White)
   - RGB and HEX color support
   - Text formatting: Bold, Dim, Italic, Underline

3. **Color Manipulation**:
   - Color lightening and darkening
   - RGB color interpolation
   - Gradient effects

4. **Flexible API**:
   - Builder pattern for style creation
   - Trait-based styling with `Styleable`
   - Error handling with proper Result types

## Examples

### Basic Usage
```rust
use inksac::{Color, Style, Styleable};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let style = Style::builder()
        .foreground(Color::Green)
        .bold()
        .build();

    println!("{}", "Hello, Colorful World!".style(style));
    Ok(())
}
```

### Styled Error Messages
```rust
use inksac::{Color, Style, Styleable};

let error_style = Style::builder()
    .foreground(Color::Red)
    .bold()
    .build();

println!("{}: {}", "ERROR".style(error_style), "Operation failed");
```

## Installation

Add Inksac to your project:

```sh
cargo add inksac
```

## Examples

Check out the [examples directory](./examples) for more detailed examples including:
- Error message styling
- Rainbow text effects
- Progress bars with gradients
- Formatted tables
- Color support detection

## Platform Support

Inksac automatically detects terminal capabilities and adjusts its output accordingly. The actual color representation may vary based on:
- Terminal emulator capabilities
- Operating system
- Color support level (Basic, 256 colors, True Color)

---
Happy Coding!

