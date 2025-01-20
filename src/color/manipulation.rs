use super::basic::Color;
use crate::error::ColorError;

impl Color {
    /// Lighten a color by a percentage
    ///
    /// # Arguments
    /// * `percent` - Amount to lighten (0-100)
    ///
    /// # Returns
    /// * `Ok(Color)` - Lightened color
    /// * `Err(ColorError)` - If color manipulation fails
    ///
    /// # Examples
    /// ```rust
    /// use inksac::Color;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let color = Color::new_rgb(255, 100, 0)?;
    ///     let lighter = color.lighten(30)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn lighten(self, percent: u8) -> Result<Self, ColorError> {
        match self {
            Color::RGB(r, g, b) => {
                let percent = f32::from(percent.min(100)) / 100.0;
                let r = ((255.0 - f32::from(r)) * percent + f32::from(r)) as u8;
                let g = ((255.0 - f32::from(g)) * percent + f32::from(g)) as u8;
                let b = ((255.0 - f32::from(b)) * percent + f32::from(b)) as u8;
                Color::new_rgb(r, g, b)
            }
            Color::HEX(hex) => {
                let (r, g, b) = Self::validate_hex(hex)?;
                Color::RGB(r, g, b).lighten(percent)
            }
            _ => Ok(self),
        }
    }

    /// Darken a color by a percentage
    ///
    /// # Arguments
    /// * `percent` - Amount to darken (0-100)
    ///
    /// # Returns
    /// * `Ok(Color)` - Darkened color
    /// * `Err(ColorError)` - If color manipulation fails
    ///
    /// # Examples
    /// ```rust
    /// use inksac::Color;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let color = Color::new_rgb(255, 100, 0)?;
    ///     let darker = color.darken(30)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn darken(self, percent: u8) -> Result<Self, ColorError> {
        match self {
            Color::RGB(r, g, b) => {
                let percent = f32::from(percent.min(100)) / 100.0;
                let r = (f32::from(r) * (1.0 - percent)) as u8;
                let g = (f32::from(g) * (1.0 - percent)) as u8;
                let b = (f32::from(b) * (1.0 - percent)) as u8;
                Color::new_rgb(r, g, b)
            }
            Color::HEX(hex) => {
                let (r, g, b) = Self::validate_hex(hex)?;
                Color::RGB(r, g, b).darken(percent)
            }
            _ => Ok(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::tests::run_with_env_vars;

    #[test]
    fn test_lighten() {
        run_with_env_vars(
            &[
                ("COLORTERM", Some("truecolor")),
                ("TERM", Some("xterm-256color")),
                ("NO_COLOR", None),
            ],
            || {
                let color = Color::new_rgb(100, 100, 100).unwrap();
                let lightened = color.lighten(50).unwrap();
                if let Color::RGB(r, g, b) = lightened {
                    assert!(r > 100);
                    assert!(g > 100);
                    assert!(b > 100);
                }
            },
        );
    }

    #[test]
    fn test_darken() {
        run_with_env_vars(
            &[
                ("COLORTERM", Some("truecolor")),
                ("TERM", Some("xterm-256color")),
                ("NO_COLOR", None),
            ],
            || {
                let color = Color::new_rgb(100, 100, 100).unwrap();
                let darkened = color.darken(50).unwrap();
                if let Color::RGB(r, g, b) = darkened {
                    assert!(r < 100);
                    assert!(g < 100);
                    assert!(b < 100);
                }
            },
        );
    }
}
