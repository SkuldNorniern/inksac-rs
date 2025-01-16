//! Example demonstrating various features of the inksac library

use inksac::{check_color_support, Color, ColorSupport, Style, Styleable};

fn print_color_demo() -> Result<(), Box<dyn std::error::Error>> {
    // First, check color support
    let support = check_color_support()?;
    println!("Color Support Level: {}\n", support);

    // Basic colors
    let basic_colors = [
        ("Black", Color::Black),
        ("Red", Color::Red),
        ("Green", Color::Green),
        ("Yellow", Color::Yellow),
        ("Blue", Color::Blue),
        ("Magenta", Color::Magenta),
        ("Cyan", Color::Cyan),
        ("White", Color::White),
    ];

    println!("Basic Colors:");
    for (name, color) in basic_colors {
        let style = Style::builder().foreground(color).bold().build();
        println!("{:<8}: {}", name, "■■■■".style(style));
    }
    println!();

    // RGB Colors (if supported)
    if matches!(support, ColorSupport::TrueColor) {
        println!("RGB Colors:");
        let rgb_colors = [
            (255, 100, 0),
            (100, 255, 0),
            (0, 255, 100),
            (0, 100, 255),
            (100, 0, 255),
            (255, 0, 100),
        ];

        for (r, g, b) in rgb_colors {
            let style = Style::builder()
                .foreground(Color::new_rgb(r, g, b)?)
                .build();
            println!("RGB({}, {}, {}): {}", r, g, b, "■■■■".style(style));
        }
        println!();

        // Color manipulation
        let base_color = Color::new_rgb(255, 100, 0)?;
        println!("Color Manipulation:");
        println!(
            "Original: {}",
            "■■■■".style(Style::builder().foreground(base_color).build())
        );

        let lighter = base_color.lighten(30)?;
        println!(
            "Lighter : {}",
            "■■■■".style(Style::builder().foreground(lighter).build())
        );

        let darker = base_color.darken(30)?;
        println!(
            "Darker  : {}",
            "■■■■".style(Style::builder().foreground(darker).build())
        );
        println!();
    }

    // Text styles
    println!("Text Styles:");
    let text = "Styled Text";

    let bold = Style::builder().bold().build();
    println!("Bold:      {}", text.style(bold));

    let italic = Style::builder().italic().build();
    println!("Italic:    {}", text.style(italic));

    let underline = Style::builder().underline().build();
    println!("Underline: {}", text.style(underline));

    let dim = Style::builder().dim().build();
    println!("Dim:       {}", text.style(dim));

    // Combined styles
    let combined = Style::builder()
        .foreground(Color::Green)
        .background(Color::Black)
        .bold()
        .italic()
        .build();
    println!("\nCombined Styles:");
    println!("{}", "Multiple styles combined!".style(combined));

    Ok(())
}

fn main() {
    if let Err(e) = print_color_demo() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
