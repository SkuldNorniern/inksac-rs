//! Example demonstrating color conversion between RGB, 256-color, and basic ANSI colors

use inksac::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Color Conversion Comparison (RGB → 256 → Basic):");
    println!("----------------------------------------------");

    let test_colors = [
        ((0, 0, 0), "Black"),
        ((255, 255, 255), "White"),
        ((128, 128, 128), "Mid Gray"),
        ((255, 0, 0), "Red"),
        ((0, 255, 0), "Green"),
        ((0, 0, 255), "Blue"),
        ((255, 255, 0), "Yellow"),
        ((255, 0, 255), "Magenta"),
        ((0, 255, 255), "Cyan"),
        ((128, 64, 32), "Brown"),
        ((70, 130, 180), "Steel Blue"),
    ];

    for ((r, g, b), name) in test_colors {
        // Original RGB color
        let rgb_style = Style::builder()
            .foreground(Color::new_rgb(r, g, b)?)
            .bold()
            .build();

        // Get the 256-color code
        let code_256 = Color::rgb_to_256(r, g, b);
        let color_256 = Color::Color256(code_256);
        let style_256 = Style::builder().foreground(color_256).bold().build();

        // Get the basic ANSI color
        let basic = Color::rgb_to_basic(r, g, b);
        let basic_style = Style::builder().foreground(basic).bold().build();

        println!(
            "{:<15} RGB({:>3},{:>3},{:>3}): {} | 256({:>3}): {} | Basic: {}",
            name,
            r,
            g,
            b,
            "■■■■".style(rgb_style),
            code_256,
            "■■■■".style(style_256),
            "■■■■".style(basic_style),
        );
    }

    // Show some edge cases
    println!("\nEdge Cases and Special Colors:");
    println!("-----------------------------");

    let edge_cases = [
        // Brown variations
        ((139, 69, 19), "Saddle Brown"),
        ((128, 64, 0), "Brown"),
        ((165, 42, 42), "Brown 2"),
        ((160, 82, 45), "Sienna"),
        ((210, 105, 30), "Chocolate"),
        ((184, 134, 11), "Dark Goldenrod"),
        ((153, 76, 0), "Darker Brown"),
        ((102, 51, 0), "Deep Brown"),
        // Very dark gray (specific test case)
        ((32, 32, 32), "Very Dark Gray"),
        // Near-boundary cases
        ((51, 51, 51), "Dark Gray"),
        ((102, 102, 102), "Medium Gray"),
        ((204, 204, 204), "Light Gray"),
        ((254, 254, 254), "Almost White"),
        ((1, 1, 1), "Almost Black"),
        // Almost-primary colors
        ((254, 0, 0), "Near Red"),
        ((0, 254, 0), "Near Green"),
        ((0, 0, 254), "Near Blue"),
        // Web colors
        ((147, 112, 219), "Medium Purple"),
        ((64, 224, 208), "Turquoise"),
        ((250, 128, 114), "Salmon"),
        ((85, 107, 47), "Dark Olive"),
        ((219, 112, 147), "Pale Violet"),
        // Subtle variations
        ((128, 0, 0), "Maroon"),
        ((128, 0, 128), "Purple"),
        ((0, 128, 128), "Teal"),
        // Mixed intensities
        ((192, 64, 64), "Light Red"),
        ((64, 192, 64), "Light Green"),
        ((64, 64, 192), "Light Blue"),
        // Color cube edge cases
        ((51, 0, 0), "Dark Red"),
        ((102, 0, 0), "Medium Red"),
        ((204, 0, 0), "Bright Red"),
        // Pastels
        ((255, 182, 193), "Light Pink"),
        ((176, 224, 230), "Powder Blue"),
        ((255, 218, 185), "Peach Puff"),
        // Earth tones
        ((210, 180, 140), "Tan"),
        // Neon colors
        ((255, 0, 127), "Neon Pink"),
        ((127, 255, 0), "Neon Green"),
        ((0, 127, 255), "Neon Blue"),
        // Gradient steps
        ((51, 51, 51), "20% Gray"),
        ((102, 102, 102), "40% Gray"),
        ((153, 153, 153), "60% Gray"),
        ((204, 204, 204), "80% Gray"),
        // Color blends
        ((64, 0, 128), "Deep Purple"),
        ((0, 128, 64), "Sea Green"),
    ];

    for ((r, g, b), name) in edge_cases {
        let rgb_style = Style::builder()
            .foreground(Color::new_rgb(r, g, b)?)
            .bold()
            .build();

        let code_256 = Color::rgb_to_256(r, g, b);
        let style_256 = Style::builder()
            .foreground(Color::Color256(code_256))
            .bold()
            .build();

        let basic = Color::rgb_to_basic(r, g, b);
        let basic_style = Style::builder().foreground(basic).bold().build();

        println!(
            "{:<15} RGB({:>3},{:>3},{:>3}): {} | 256({:>3}): {} | Basic: {}",
            name,
            r,
            g,
            b,
            "■■■■".style(rgb_style),
            code_256,
            "■■■■".style(style_256),
            "■■■■".style(basic_style),
        );
    }

    Ok(())
}
