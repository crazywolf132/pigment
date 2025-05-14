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
