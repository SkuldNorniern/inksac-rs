//! Example demonstrating a styled table

use inksac::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let header_style = Style::builder()
        .foreground(Color::White)
        .background(Color::Blue)
        .bold()
        .build();

    let alt_row_style = Style::builder()
        .background(Color::new_rgb(240, 240, 240)?)
        .build();

    // Print header
    println!(
        "{}",
        format!("{:<20} {:<15} {:<10}", "Name", "Role", "Status").style(header_style)
    );

    // Print rows
    let data = [
        ("Alice Smith", "Developer", "Active"),
        ("Bob Johnson", "Designer", "Away"),
        ("Carol White", "Manager", "Busy"),
    ];

    for (i, (name, role, status)) in data.iter().enumerate() {
        let row = format!("{:<20} {:<15} {:<10}", name, role, status);
        if i % 2 == 1 {
            println!("{}", row.style(alt_row_style));
        } else {
            println!("{}", row);
        }
    }

    Ok(())
}
