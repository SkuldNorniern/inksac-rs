pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const ITALIC: &str = "\x1b[3m";
pub const UNDERLINE: &str = "\x1b[4m";

// FIX!: ASAP: what the actual fucking fuck just return boolean
/// Check if the terminal supports ANSI colors
pub fn is_color_available() -> Result<(), &'static str> {
    if std::env::var("TERM").is_ok() {
        return Ok(());
    }
    Err("Terminal does not support ANSI colors")
}
