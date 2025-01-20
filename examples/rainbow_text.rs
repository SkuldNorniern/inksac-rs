//! Example demonstrating text with gradient colors

use inksac::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "Rainbow Text Example";
    let colors = [
        (255, 0, 0),   // Red
        (255, 127, 0), // Orange
        (255, 255, 0), // Yellow
        (0, 255, 0),   // Green
        (0, 0, 255),   // Blue
        (75, 0, 130),  // Indigo
        (148, 0, 211), // Violet
    ];

    // Print each character with a different color
    for (i, c) in text.chars().enumerate() {
        let color_idx = i % colors.len();
        let (r, g, b) = colors[color_idx];

        let style = Style::builder()
            .foreground(Color::new_rgb(r, g, b)?)
            .bold()
            .build();

        print!("{}", c.to_string().style(style));
    }
    println!();

    Ok(())
}
