//! Example demonstrating styled error messages

use inksac::{Color, Style, Styleable};

fn print_status(status: &str, message: &str) {
    match status {
        "error" => {
            let style = Style::builder()
                .foreground(Color::Red)
                .bold()
                .build();
            println!("{}: {}", "ERROR".style(style), message);
        }
        "warning" => {
            let style = Style::builder()
                .foreground(Color::Yellow)
                .bold()
                .build();
            println!("{}: {}", "WARNING".style(style), message);
        }
        "success" => {
            let style = Style::builder()
                .foreground(Color::Green)
                .bold()
                .build();
            println!("{}: {}", "SUCCESS".style(style), message);
        }
        _ => println!("{}: {}", status, message),
    }
}

fn main() {
    print_status("error", "Failed to connect to database");
    print_status("warning", "Configuration file not found, using defaults");
    print_status("success", "Data successfully imported");
} 