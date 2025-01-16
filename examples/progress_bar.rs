//! Example demonstrating a colored progress bar with gradient color change

use inksac::prelude::*;
use std::{io::Write, thread, time::Duration};

/// Linear interpolation between two RGB colors
fn lerp_color(start: (u8, u8, u8), end: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);
    (
        (start.0 as f32 + (end.0 as f32 - start.0 as f32) * t) as u8,
        (start.1 as f32 + (end.1 as f32 - start.1 as f32) * t) as u8,
        (start.2 as f32 + (end.2 as f32 - start.2 as f32) * t) as u8,
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let total = 50;
    // More vibrant and pleasing colors
    let start_color = (86, 171, 255);  // Light blue
    let mid_color = (255, 135, 255);   // Pink/Purple
    let end_color = (98, 255, 161);    // Mint green

    // Styles for different parts of the progress bar
    let empty_style = Style::builder()
        .foreground(Color::new_rgb(70, 70, 70)?)  // Darker gray for empty section
        .build();
    
    let border_style = Style::builder()
        .foreground(Color::new_rgb(150, 150, 150)?)  // Lighter gray for borders
        .bold()
        .build();

    let percent_style = Style::builder()
        .foreground(Color::White)
        .bold()
        .build();

    print!("\n");  // Start with a newline for better spacing
    
    for i in 0..=total {
        let progress = i as f32 / total as f32;
        let filled = (progress * 30.0) as usize;  // Slightly shorter bar
        let empty = 30 - filled;

        // Two-stage color interpolation for smoother gradient
        let (r, g, b) = if progress < 0.5 {
            lerp_color(start_color, mid_color, progress * 2.0)
        } else {
            lerp_color(mid_color, end_color, (progress - 0.5) * 2.0)
        };

        let bar_style = Style::builder()
            .foreground(Color::new_rgb(r, g, b)?)
            .bold()
            .build();

        // Using better characters for the progress bar
        print!("\r  ");  // Add some left padding
        print!("{}", "├".style(border_style));
        print!("{}", "━".repeat(filled).style(bar_style));
        print!("{}", "─".repeat(empty).style(empty_style));
        print!("{}", "┤".style(border_style));
        
        // Percentage with padding to prevent jitter
        print!(" {}%", format!("{:>3}", (progress * 100.0) as u8).style(percent_style));
        
        // Existing spinner for visual feedback
        let spinner = match i % 10 {
            0 => "⠋",
            1 => "⠙",
            2 => "⠹",
            3 => "⠸",
            4 => "⠼",
            5 => "⠴",
            6 => "⠦",
            7 => "⠧",
            8 => "⠇",
            9 => "⠏",
            _ => "",
        };
        print!(" {}", spinner.style(bar_style));

        // Added spinning circle indicator on the right
        let circle_spinner = match i % 4 {
            0 => "◐",
            1 => "◓",
            2 => "◑",
            3 => "◒",
            _ => "",
        };
        print!(" {}", circle_spinner.style(bar_style));

        std::io::stdout().flush()?;
        thread::sleep(Duration::from_millis(50));
    }
    println!("\n");  // Add final newlines for spacing

    Ok(())
}
