# Inksac

Inksac is a Rust library that enhances terminal outputs by providing easy to use ANSI color support. It's designed to be simple, flexible, and enjoyable to use, giving your terminal applications a colorful edge.

## Features

1. **Check ANSI Color Support**: Inksac has a built-in function to check if the current terminal supports ANSI colors.

2. **Customizable Styles**: Create your own text styles with customizable foreground and background colors. 

3. **Colorful Text Output**: Enhance your terminal applications with colorful text outputs using Inksac's `ColoredString`.

4. **True Color Support**: Inksac Supports True Color by RGB, HEX values.

5. **Format Support**: Inksac Support Bold, Dim, Italic, Underline - More options can be added in the future.

## Usage

```rust
use inksac::check_color_available;
use inksac::types::*;

// Check if the terminal supports ANSI colors
match check_color_available() {
    Ok(_) => println!("Terminal supports ANSI colors"),
    Err(_) => println!("Terminal does not support ANSI colors"),
}

// Define your style
const TITLESTYLE: Style = Style{
    forground: Some(Color::Green),
    background: Some(Color::Red),
    bold: true,
    dim: false,
    italic: false,
    underline: false
};

// Create a colored string
let title_text: ColoredString = ColoredString::new(
    "Hello World",
    TITLESTYLE
);

// Output the colored string
println!("{}", title_text);
```


## Installation

```sh
cargo add inksac

```



## Disclaimer

Please note that color support depends on the terminal's capabilities and might not work as expected on all platforms.


---
Happy Coding!

