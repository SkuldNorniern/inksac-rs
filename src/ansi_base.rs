#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";
#[allow(dead_code)]
pub const BOLD: &str = "\x1b[1m";
#[allow(dead_code)]
pub const DIM: &str = "\x1b[2m";
#[allow(dead_code)]
pub const ITALIC: &str = "\x1b[3m";
#[allow(dead_code)]
pub const UNDERLINE: &str = "\x1b[4m";

#[allow(dead_code)]
pub fn check_color_available() -> Result<&'static str, &'static str> {
    // Ckeck if the terminal supports ANSI colors
    // https://en.wikipedia.org/wiki/ANSI_escape_code

    if std::env::var("TERM").is_ok() {
        return Ok("Terminal supports ANSI colors");
    }
    Err("Terminal does not support ANSI colors")
}
