#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub(crate) name: &'static str,
    pub(crate) hex: &'static str,
    pub(crate) rgb: (u8, u8, u8),
}

impl Color {
    #[inline]
    pub fn name(&self) -> &'static str {
        self.name
    }
    #[inline]
    pub fn hex(&self) -> &'static str {
        self.hex
    }
    #[inline]
    pub fn rgb(&self) -> (u8, u8, u8) {
        self.rgb
    }

    pub fn ansi(&self) -> Ansi {
        Ansi { rgb: self.rgb }
    }
}

#[cfg(feature = "owo")]
impl From<Color> for owo_colors::Rgb {
    fn from(c: Color) -> Self {
        let (r, g, b) = c.rgb;
        owo_colors::Rgb(r, g, b)
    }
}

#[cfg(feature = "termcolor")]
impl From<Color> for termcolor::Color {
    fn from(c: Color) -> Self {
        let (r, g, b) = c.rgb;
        termcolor::Color::Rgb(r, g, b)
    }
}

#[cfg(feature = "colored")]
impl From<Color> for colored::Color {
    fn from(c: Color) -> Self {
        let (r, g, b) = c.rgb;
        colored::Color::TrueColor { r, g, b }
    }
}

#[cfg(feature = "anstyle")]
impl From<Color> for anstyle::Color {
    fn from(c: Color) -> Self {
        let (r, g, b) = c.rgb;
        anstyle::Color::Rgb(anstyle::RgbColor(r, g, b))
    }
}

#[cfg(feature = "nu-ansi-term")]
impl From<Color> for nu_ansi_term::Color {
    fn from(c: Color) -> Self {
        let (r, g, b) = c.rgb;
        nu_ansi_term::Color::Rgb(r, g, b)
    }
}

#[cfg(feature = "yansi")]
impl From<Color> for yansi::Color {
    fn from(c: Color) -> Self {
        let (r, g, b) = c.rgb;
        yansi::Color::Rgb(r, g, b)
    }
}

#[cfg(feature = "crossterm")]
impl From<Color> for crossterm::style::Color {
    fn from(c: Color) -> Self {
        let (r, g, b) = c.rgb;
        crossterm::style::Color::Rgb { r, g, b }
    }
}

use crate::ansi::Ansi;

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create test colors
    fn create_test_color(name: &'static str, hex: &'static str, rgb: (u8, u8, u8)) -> Color {
        Color { name, hex, rgb }
    }

    mod getters {
        use super::*;

        #[test]
        fn test_name_getter() {
            let color = create_test_color("Test Color", "#123456", (18, 52, 86));
            assert_eq!(color.name(), "Test Color");

            // Test with empty name
            let empty_name = create_test_color("", "#000000", (0, 0, 0));
            assert_eq!(empty_name.name(), "");

            // Test with special characters
            let special_chars = create_test_color("Test-Color_123!", "#FFFFFF", (255, 255, 255));
            assert_eq!(special_chars.name(), "Test-Color_123!");
        }

        #[test]
        fn test_hex_getter() {
            let color = create_test_color("Test Color", "#123456", (18, 52, 86));
            assert_eq!(color.hex(), "#123456");

            // Test with uppercase hex
            let uppercase_hex = create_test_color("Black", "#000000", (0, 0, 0));
            assert_eq!(uppercase_hex.hex(), "#000000");

            // Test with lowercase hex
            let lowercase_hex = create_test_color("White", "#ffffff", (255, 255, 255));
            assert_eq!(lowercase_hex.hex(), "#ffffff");
        }

        #[test]
        fn test_rgb_getter() {
            let color = create_test_color("Test Color", "#123456", (18, 52, 86));
            assert_eq!(color.rgb(), (18, 52, 86));

            // Test with zero values
            let black = create_test_color("Black", "#000000", (0, 0, 0));
            assert_eq!(black.rgb(), (0, 0, 0));

            // Test with max values
            let white = create_test_color("White", "#FFFFFF", (255, 255, 255));
            assert_eq!(white.rgb(), (255, 255, 255));

            // Test with mixed values
            let red = create_test_color("Red", "#FF0000", (255, 0, 0));
            assert_eq!(red.rgb(), (255, 0, 0));
        }
    }

    mod ansi_conversion {
        use super::*;

        #[test]
        fn test_ansi_conversion() {
            let color = create_test_color("Test Color", "#123456", (18, 52, 86));
            let ansi = color.ansi();
            assert_eq!(ansi.rgb, (18, 52, 86));

            // Test with black
            let black = create_test_color("Black", "#000000", (0, 0, 0));
            let black_ansi = black.ansi();
            assert_eq!(black_ansi.rgb, (0, 0, 0));

            // Test with white
            let white = create_test_color("White", "#FFFFFF", (255, 255, 255));
            let white_ansi = white.ansi();
            assert_eq!(white_ansi.rgb, (255, 255, 255));
        }

        #[test]
        fn test_ansi_output_format() {
            let red = create_test_color("Red", "#FF0000", (255, 0, 0));
            let ansi = red.ansi();

            // Check that the ANSI format is correct
            assert_eq!(ansi.fg(), "\x1b[38;2;255;0;0m");
            assert_eq!(ansi.bg(), "\x1b[48;2;255;0;0m");
        }
    }

    #[cfg(feature = "owo")]
    mod owo_integration {
        use super::*;

        #[test]
        fn test_color_to_owo_rgb() {
            let color = create_test_color("Test Color", "#123456", (18, 52, 86));
            let owo_rgb: owo_colors::Rgb = color.into();
            assert_eq!(owo_rgb.0, 18);
            assert_eq!(owo_rgb.1, 52);
            assert_eq!(owo_rgb.2, 86);
        }

        #[test]
        fn test_color_to_owo_rgb_extremes() {
            // Test with black
            let black = create_test_color("Black", "#000000", (0, 0, 0));
            let black_owo: owo_colors::Rgb = black.into();
            assert_eq!(black_owo.0, 0);
            assert_eq!(black_owo.1, 0);
            assert_eq!(black_owo.2, 0);

            // Test with white
            let white = create_test_color("White", "#FFFFFF", (255, 255, 255));
            let white_owo: owo_colors::Rgb = white.into();
            assert_eq!(white_owo.0, 255);
            assert_eq!(white_owo.1, 255);
            assert_eq!(white_owo.2, 255);
        }
    }

    #[cfg(feature = "termcolor")]
    mod termcolor_integration {
        use super::*;

        #[test]
        fn test_color_to_termcolor() {
            let color = create_test_color("Test Color", "#123456", (18, 52, 86));
            let tc: termcolor::Color = color.into();
            match tc {
                termcolor::Color::Rgb(r, g, b) => {
                    assert_eq!(r, 18);
                    assert_eq!(g, 52);
                    assert_eq!(b, 86);
                }
                _ => panic!("Expected RGB color"),
            }
        }

        #[test]
        fn test_color_to_termcolor_extremes() {
            // Test with black
            let black = create_test_color("Black", "#000000", (0, 0, 0));
            let black_tc: termcolor::Color = black.into();
            match black_tc {
                termcolor::Color::Rgb(r, g, b) => {
                    assert_eq!(r, 0);
                    assert_eq!(g, 0);
                    assert_eq!(b, 0);
                }
                _ => panic!("Expected RGB color"),
            }

            // Test with white
            let white = create_test_color("White", "#FFFFFF", (255, 255, 255));
            let white_tc: termcolor::Color = white.into();
            match white_tc {
                termcolor::Color::Rgb(r, g, b) => {
                    assert_eq!(r, 255);
                    assert_eq!(g, 255);
                    assert_eq!(b, 255);
                }
                _ => panic!("Expected RGB color"),
            }
        }
    }

    #[cfg(feature = "colored")]
    mod colored_integration {
        use super::*;

        #[test]
        fn test_color_to_colored() {
            let color = create_test_color("Test Color", "#123456", (18, 52, 86));
            let c: colored::Color = color.into();
            match c {
                colored::Color::TrueColor { r, g, b } => {
                    assert_eq!(r, 18);
                    assert_eq!(g, 52);
                    assert_eq!(b, 86);
                }
                _ => panic!("Expected TrueColor"),
            }
        }

        #[test]
        fn test_color_to_colored_extremes() {
            // Test with black
            let black = create_test_color("Black", "#000000", (0, 0, 0));
            let black_c: colored::Color = black.into();
            match black_c {
                colored::Color::TrueColor { r, g, b } => {
                    assert_eq!(r, 0);
                    assert_eq!(g, 0);
                    assert_eq!(b, 0);
                }
                _ => panic!("Expected TrueColor"),
            }

            // Test with white
            let white = create_test_color("White", "#FFFFFF", (255, 255, 255));
            let white_c: colored::Color = white.into();
            match white_c {
                colored::Color::TrueColor { r, g, b } => {
                    assert_eq!(r, 255);
                    assert_eq!(g, 255);
                    assert_eq!(b, 255);
                }
                _ => panic!("Expected TrueColor"),
            }
        }
    }

    #[cfg(feature = "anstyle")]
    mod anstyle_integration {
        use super::*;

        #[test]
        fn test_color_to_anstyle() {
            let color = create_test_color("Test Color", "#123456", (18, 52, 86));
            let a: anstyle::Color = color.into();
            match a {
                anstyle::Color::Rgb(rgb) => {
                    assert_eq!(rgb.0, 18);
                    assert_eq!(rgb.1, 52);
                    assert_eq!(rgb.2, 86);
                }
                _ => panic!("Expected RGB color"),
            }
        }

        #[test]
        fn test_color_to_anstyle_extremes() {
            // Test with black
            let black = create_test_color("Black", "#000000", (0, 0, 0));
            let black_a: anstyle::Color = black.into();
            match black_a {
                anstyle::Color::Rgb(rgb) => {
                    assert_eq!(rgb.0, 0);
                    assert_eq!(rgb.1, 0);
                    assert_eq!(rgb.2, 0);
                }
                _ => panic!("Expected RGB color"),
            }

            // Test with white
            let white = create_test_color("White", "#FFFFFF", (255, 255, 255));
            let white_a: anstyle::Color = white.into();
            match white_a {
                anstyle::Color::Rgb(rgb) => {
                    assert_eq!(rgb.0, 255);
                    assert_eq!(rgb.1, 255);
                    assert_eq!(rgb.2, 255);
                }
                _ => panic!("Expected RGB color"),
            }
        }
    }

    #[cfg(feature = "nu-ansi-term")]
    mod nu_ansi_term_integration {
        use super::*;

        #[test]
        fn test_color_to_nu_ansi_term() {
            let color = create_test_color("Test Color", "#123456", (18, 52, 86));
            let nat: nu_ansi_term::Color = color.into();
            match nat {
                nu_ansi_term::Color::Rgb(r, g, b) => {
                    assert_eq!(r, 18);
                    assert_eq!(g, 52);
                    assert_eq!(b, 86);
                }
                _ => panic!("Expected RGB color"),
            }
        }

        #[test]
        fn test_color_to_nu_ansi_term_extremes() {
            // Test with black
            let black = create_test_color("Black", "#000000", (0, 0, 0));
            let black_nat: nu_ansi_term::Color = black.into();
            match black_nat {
                nu_ansi_term::Color::Rgb(r, g, b) => {
                    assert_eq!(r, 0);
                    assert_eq!(g, 0);
                    assert_eq!(b, 0);
                }
                _ => panic!("Expected RGB color"),
            }

            // Test with white
            let white = create_test_color("White", "#FFFFFF", (255, 255, 255));
            let white_nat: nu_ansi_term::Color = white.into();
            match white_nat {
                nu_ansi_term::Color::Rgb(r, g, b) => {
                    assert_eq!(r, 255);
                    assert_eq!(g, 255);
                    assert_eq!(b, 255);
                }
                _ => panic!("Expected RGB color"),
            }
        }
    }

    #[cfg(feature = "yansi")]
    mod yansi_integration {
        use super::*;

        #[test]
        fn test_color_to_yansi() {
            let color = create_test_color("Test Color", "#123456", (18, 52, 86));
            let y: yansi::Color = color.into();
            match y {
                yansi::Color::Rgb(r, g, b) => {
                    assert_eq!(r, 18);
                    assert_eq!(g, 52);
                    assert_eq!(b, 86);
                }
                _ => panic!("Expected RGB color"),
            }
        }

        #[test]
        fn test_color_to_yansi_extremes() {
            // Test with black
            let black = create_test_color("Black", "#000000", (0, 0, 0));
            let black_y: yansi::Color = black.into();
            match black_y {
                yansi::Color::Rgb(r, g, b) => {
                    assert_eq!(r, 0);
                    assert_eq!(g, 0);
                    assert_eq!(b, 0);
                }
                _ => panic!("Expected RGB color"),
            }

            // Test with white
            let white = create_test_color("White", "#FFFFFF", (255, 255, 255));
            let white_y: yansi::Color = white.into();
            match white_y {
                yansi::Color::Rgb(r, g, b) => {
                    assert_eq!(r, 255);
                    assert_eq!(g, 255);
                    assert_eq!(b, 255);
                }
                _ => panic!("Expected RGB color"),
            }
        }
    }

    #[cfg(feature = "crossterm")]
    mod crossterm_integration {
        use super::*;

        #[test]
        fn test_color_to_crossterm() {
            let color = create_test_color("Test Color", "#123456", (18, 52, 86));
            let ct: crossterm::style::Color = color.into();
            match ct {
                crossterm::style::Color::Rgb { r, g, b } => {
                    assert_eq!(r, 18);
                    assert_eq!(g, 52);
                    assert_eq!(b, 86);
                }
                _ => panic!("Expected RGB color"),
            }
        }

        #[test]
        fn test_color_to_crossterm_extremes() {
            // Test with black
            let black = create_test_color("Black", "#000000", (0, 0, 0));
            let black_ct: crossterm::style::Color = black.into();
            match black_ct {
                crossterm::style::Color::Rgb { r, g, b } => {
                    assert_eq!(r, 0);
                    assert_eq!(g, 0);
                    assert_eq!(b, 0);
                }
                _ => panic!("Expected RGB color"),
            }

            // Test with white
            let white = create_test_color("White", "#FFFFFF", (255, 255, 255));
            let white_ct: crossterm::style::Color = white.into();
            match white_ct {
                crossterm::style::Color::Rgb { r, g, b } => {
                    assert_eq!(r, 255);
                    assert_eq!(g, 255);
                    assert_eq!(b, 255);
                }
                _ => panic!("Expected RGB color"),
            }
        }
    }

    mod equality {
        use super::*;

        #[test]
        fn test_color_equality() {
            let color1 = create_test_color("Test Color", "#123456", (18, 52, 86));
            let color2 = create_test_color("Test Color", "#123456", (18, 52, 86));

            assert_eq!(color1, color2);
        }

        #[test]
        fn test_color_inequality_by_name() {
            let color1 = create_test_color("Test Color", "#123456", (18, 52, 86));
            let color2 = create_test_color("Different Color", "#123456", (18, 52, 86));

            assert_ne!(color1, color2);
        }

        #[test]
        fn test_color_inequality_by_hex() {
            let color1 = create_test_color("Test Color", "#123456", (18, 52, 86));
            let color2 = create_test_color("Test Color", "#654321", (18, 52, 86));

            assert_ne!(color1, color2);
        }

        #[test]
        fn test_color_inequality_by_rgb() {
            let color1 = create_test_color("Test Color", "#123456", (18, 52, 86));
            let color2 = create_test_color("Test Color", "#123456", (86, 52, 18));

            assert_ne!(color1, color2);
        }
    }
}
