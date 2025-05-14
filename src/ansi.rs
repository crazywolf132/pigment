#[derive(Debug, Clone, Copy)]
pub struct Ansi {
    pub(crate) rgb: (u8, u8, u8),
}

impl Ansi {
    #[inline]
    pub fn fg(&self) -> String {
        let (r, g, b) = self.rgb;
        format!("\x1b[38;2;{r};{g};{b}m")
    }
    #[inline]
    pub fn bg(&self) -> String {
        let (r, g, b) = self.rgb;
        format!("\x1b[48;2;{r};{g};{b}m")
    }
    #[inline]
    pub fn reset() -> &'static str {
        "\x1b[0m"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create test ANSI instances
    fn create_ansi(r: u8, g: u8, b: u8) -> Ansi {
        Ansi { rgb: (r, g, b) }
    }

    mod foreground {
        use super::*;

        #[test]
        fn test_fg_format() {
            // Test the basic format of the foreground ANSI code
            let ansi = create_ansi(255, 0, 128);
            assert_eq!(ansi.fg(), "\x1b[38;2;255;0;128m");
        }

        #[test]
        fn test_fg_with_zero_values() {
            // Test with all zeros (black)
            let black = create_ansi(0, 0, 0);
            assert_eq!(black.fg(), "\x1b[38;2;0;0;0m");
        }

        #[test]
        fn test_fg_with_max_values() {
            // Test with all max values (white)
            let white = create_ansi(255, 255, 255);
            assert_eq!(white.fg(), "\x1b[38;2;255;255;255m");
        }

        #[test]
        fn test_fg_with_primary_colors() {
            // Test with primary colors
            let red = create_ansi(255, 0, 0);
            let green = create_ansi(0, 255, 0);
            let blue = create_ansi(0, 0, 255);

            assert_eq!(red.fg(), "\x1b[38;2;255;0;0m");
            assert_eq!(green.fg(), "\x1b[38;2;0;255;0m");
            assert_eq!(blue.fg(), "\x1b[38;2;0;0;255m");
        }

        #[test]
        fn test_fg_with_mixed_values() {
            // Test with mixed values
            let mixed = create_ansi(123, 45, 67);
            assert_eq!(mixed.fg(), "\x1b[38;2;123;45;67m");
        }
    }

    mod background {
        use super::*;

        #[test]
        fn test_bg_format() {
            // Test the basic format of the background ANSI code
            let ansi = create_ansi(0, 128, 255);
            assert_eq!(ansi.bg(), "\x1b[48;2;0;128;255m");
        }

        #[test]
        fn test_bg_with_zero_values() {
            // Test with all zeros (black)
            let black = create_ansi(0, 0, 0);
            assert_eq!(black.bg(), "\x1b[48;2;0;0;0m");
        }

        #[test]
        fn test_bg_with_max_values() {
            // Test with all max values (white)
            let white = create_ansi(255, 255, 255);
            assert_eq!(white.bg(), "\x1b[48;2;255;255;255m");
        }

        #[test]
        fn test_bg_with_primary_colors() {
            // Test with primary colors
            let red = create_ansi(255, 0, 0);
            let green = create_ansi(0, 255, 0);
            let blue = create_ansi(0, 0, 255);

            assert_eq!(red.bg(), "\x1b[48;2;255;0;0m");
            assert_eq!(green.bg(), "\x1b[48;2;0;255;0m");
            assert_eq!(blue.bg(), "\x1b[48;2;0;0;255m");
        }

        #[test]
        fn test_bg_with_mixed_values() {
            // Test with mixed values
            let mixed = create_ansi(123, 45, 67);
            assert_eq!(mixed.bg(), "\x1b[48;2;123;45;67m");
        }
    }

    mod reset {
        use super::*;

        #[test]
        fn test_reset_value() {
            // Test the reset ANSI code
            assert_eq!(Ansi::reset(), "\x1b[0m");
        }

        #[test]
        fn test_reset_is_static() {
            // Ensure reset is always the same
            assert_eq!(Ansi::reset(), Ansi::reset());
        }
    }

    mod combined {
        use super::*;

        #[test]
        fn test_fg_and_bg_different() {
            // Ensure fg and bg codes are different for the same color
            let ansi = create_ansi(100, 150, 200);
            assert_ne!(ansi.fg(), ansi.bg());
        }

        #[test]
        fn test_ansi_sequence() {
            // Test a typical ANSI color sequence
            let red = create_ansi(255, 0, 0);
            let text = format!("{}Red Text{}", red.fg(), Ansi::reset());

            assert_eq!(text, "\x1b[38;2;255;0;0mRed Text\x1b[0m");
        }

        #[test]
        fn test_bg_sequence() {
            // Test a background color sequence
            let blue = create_ansi(0, 0, 255);
            let text = format!("{}Blue Background{}", blue.bg(), Ansi::reset());

            assert_eq!(text, "\x1b[48;2;0;0;255mBlue Background\x1b[0m");
        }
    }
}
