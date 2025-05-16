#[derive(Debug, Clone, Copy)]
pub struct Ansi {
    pub(crate) rgb: (u8, u8, u8),
}

impl Ansi {
    // Constructor methods
    /// Creates a new Ansi instance from RGB values
    #[inline]
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { rgb: (r, g, b) }
    }

    /// Creates a new Ansi instance from a hex color code
    ///
    /// # Arguments
    ///
    /// * `hex` - A hex color code string (e.g., "#FF0000", "FF0000", "#f00", "f00", "#FF0000FF")
    ///
    /// # Returns
    ///
    /// An `Option<Ansi>` which is `None` if the hex code is invalid
    ///
    /// # Supported Formats
    ///
    /// - 3-digit hex: "#RGB" or "RGB" (e.g., "#F00" or "F00")
    /// - 6-digit hex: "#RRGGBB" or "RRGGBB" (e.g., "#FF0000" or "FF0000")
    /// - 8-digit hex: "#RRGGBBAA" or "RRGGBBAA" (e.g., "#FF0000FF" or "FF0000FF") - alpha channel is ignored
    ///
    /// # Invalid Cases
    ///
    /// The following cases will return `None`:
    /// - Invalid length (not 3, 6, or 8 characters after removing the # prefix)
    /// - Invalid characters (not hexadecimal digits)
    /// - Multiple # symbols
    /// - Empty string
    /// - Whitespace in the hex code
    /// - Special characters in the hex code
    ///
    /// # Examples
    ///
    /// ```
    /// use pigment::ansi::Ansi;
    ///
    /// // Create from full hex code with #
    /// let red = Ansi::from_hex("#FF0000").unwrap();
    /// assert_eq!(red.fg(), "\x1b[38;2;255;0;0m");
    ///
    /// // Create from full hex code without #
    /// let green = Ansi::from_hex("00FF00").unwrap();
    /// assert_eq!(green.fg(), "\x1b[38;2;0;255;0m");
    ///
    /// // Create from short hex code with #
    /// let blue = Ansi::from_hex("#00F").unwrap();
    /// assert_eq!(blue.fg(), "\x1b[38;2;0;0;255m");
    ///
    /// // Create from short hex code without #
    /// let white = Ansi::from_hex("FFF").unwrap();
    /// assert_eq!(white.fg(), "\x1b[38;2;255;255;255m");
    ///
    /// // Create from 8-digit hex code with alpha (alpha is ignored)
    /// let purple = Ansi::from_hex("#800080FF").unwrap();
    /// assert_eq!(purple.fg(), "\x1b[38;2;128;0;128m");
    /// ```

    /// Creates a new Ansi instance from an RGB color code string
    ///
    /// # Arguments
    ///
    /// * `rgb_str` - An RGB color code string (e.g., "rgb(255, 0, 0)", "255,0,0", "255 0 0")
    ///
    /// # Returns
    ///
    /// An `Option<Ansi>` which is `None` if the RGB code is invalid
    ///
    /// # Supported Formats
    ///
    /// - CSS-style: "rgb(255, 0, 0)" or "rgb(255,0,0)"
    /// - Comma-separated: "255,0,0"
    /// - Space-separated: "255 0 0"
    /// - Extra whitespace is allowed: "  255  ,  0  ,  0  " or "  rgb  (  255  ,  0  ,  0  )  "
    ///
    /// # Invalid Cases
    ///
    /// The following cases will return `None`:
    /// - Invalid format (not matching any of the supported formats)
    /// - Invalid values (not in the range 0-255)
    /// - Missing values (not exactly 3 values)
    /// - Non-numeric values
    /// - Empty string
    /// - Decimal values (e.g., "255.5, 0, 0")
    /// - Percentage values (e.g., "100%, 0%, 0%")
    /// - Negative values (e.g., "-255, 0, 0")
    /// - Values greater than 255 (e.g., "256, 0, 0")
    ///
    /// # Examples
    ///
    /// ```
    /// use pigment::ansi::Ansi;
    ///
    /// // Create from CSS-style RGB
    /// let red = Ansi::from_rgb_str("rgb(255, 0, 0)").unwrap();
    /// assert_eq!(red.fg(), "\x1b[38;2;255;0;0m");
    ///
    /// // Create from comma-separated RGB
    /// let green = Ansi::from_rgb_str("0,255,0").unwrap();
    /// assert_eq!(green.fg(), "\x1b[38;2;0;255;0m");
    ///
    /// // Create from space-separated RGB
    /// let blue = Ansi::from_rgb_str("0 0 255").unwrap();
    /// assert_eq!(blue.fg(), "\x1b[38;2;0;0;255m");
    ///
    /// // Create from RGB with extra whitespace
    /// let purple = Ansi::from_rgb_str("  128  ,  0  ,  128  ").unwrap();
    /// assert_eq!(purple.fg(), "\x1b[38;2;128;0;128m");
    ///
    /// // Create from CSS-style RGB with extra whitespace
    /// let cyan = Ansi::from_rgb_str("  rgb  (  0  ,  255  ,  255  )  ").unwrap();
    /// assert_eq!(cyan.fg(), "\x1b[38;2;0;255;255m");
    /// ```
    pub fn from_hex(hex: &str) -> Option<Self> {
        // Check for invalid input with multiple # symbols
        if hex.matches('#').count() > 1 {
            return None;
        }

        // Remove # if present
        let hex = hex.trim_start_matches('#');

        // Check if the hex string contains only valid hex characters (0-9, A-F, a-f)
        if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }

        // Handle different hex formats
        let (r, g, b) = match hex.len() {
            // Full hex code (e.g., "FF0000")
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                (r, g, b)
            },
            // Short hex code (e.g., "F00")
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                (r, g, b)
            },
            // 8-digit hex code with alpha (e.g., "FF0000FF")
            // We'll ignore the alpha channel (last 2 digits)
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                // Alpha channel (hex[6..8]) is ignored
                (r, g, b)
            },
            // Invalid hex code
            _ => return None,
        };

        Some(Self { rgb: (r, g, b) })
    }

    pub fn from_rgb_str(rgb_str: &str) -> Option<Self> {
        // Handle empty string
        if rgb_str.trim().is_empty() {
            return None;
        }

        // Trim the input string
        let rgb_str = rgb_str.trim();

        // Try to parse as CSS-style RGB: "rgb(255, 0, 0)" or "rgb(255,0,0)"
        if rgb_str.to_lowercase().trim().starts_with("rgb") {
            // Find the opening and closing parentheses
            let open_paren = rgb_str.find('(')?;
            let close_paren = rgb_str.rfind(')')?;

            // Make sure the closing parenthesis comes after the opening one
            if close_paren <= open_paren {
                return None;
            }

            // Extract the content inside the parentheses
            let content = &rgb_str[open_paren + 1..close_paren].trim();
            return Self::parse_rgb_components(content);
        }

        // Try to parse as comma-separated or space-separated values
        Self::parse_rgb_components(rgb_str)
    }

    // Helper method to parse RGB components from a string
    fn parse_rgb_components(s: &str) -> Option<Self> {
        // First, normalize the string by replacing commas with spaces
        let normalized = s.replace(',', " ");

        // Split by whitespace
        let components: Vec<&str> = normalized.split_whitespace().collect();

        // Check if we have exactly 3 components
        if components.len() != 3 {
            return None;
        }

        // Parse each component as a u8
        let r = components[0].parse::<u8>().ok()?;
        let g = components[1].parse::<u8>().ok()?;
        let b = components[2].parse::<u8>().ok()?;

        Some(Self { rgb: (r, g, b) })
    }

    // Color methods
    /// Returns the RGB values as a tuple (r, g, b)
    #[inline]
    pub fn get_rgb(&self) -> (u8, u8, u8) {
        self.rgb
    }

    /// Returns the foreground ANSI escape sequence for this color
    #[inline]
    pub fn fg(&self) -> String {
        let (r, g, b) = self.rgb;
        format!("\x1b[38;2;{r};{g};{b}m")
    }

    /// Returns the background ANSI escape sequence for this color
    #[inline]
    pub fn bg(&self) -> String {
        let (r, g, b) = self.rgb;
        format!("\x1b[48;2;{r};{g};{b}m")
    }

    // Reset methods
    #[inline]
    pub fn reset() -> &'static str {
        "\x1b[0m"
    }
    #[inline]
    pub fn reset_bold() -> &'static str {
        "\x1b[22m"
    }
    #[inline]
    pub fn reset_underline() -> &'static str {
        "\x1b[24m"
    }
    #[inline]
    pub fn reset_italic() -> &'static str {
        "\x1b[23m"
    }
    #[inline]
    pub fn reset_formatting() -> &'static str {
        "\x1b[22;23;24;25;27;28;29m"
    }

    // Text style methods
    #[inline]
    pub fn bold() -> &'static str {
        "\x1b[1m"
    }
    #[inline]
    pub fn dim() -> &'static str {
        "\x1b[2m"
    }
    #[inline]
    pub fn italic() -> &'static str {
        "\x1b[3m"
    }
    #[inline]
    pub fn underline() -> &'static str {
        "\x1b[4m"
    }
    #[inline]
    pub fn blink() -> &'static str {
        "\x1b[5m"
    }
    #[inline]
    pub fn fast_blink() -> &'static str {
        "\x1b[6m"
    }
    #[inline]
    pub fn inverse() -> &'static str {
        "\x1b[7m"
    }
    #[inline]
    pub fn hidden() -> &'static str {
        "\x1b[8m"
    }
    #[inline]
    pub fn strikethrough() -> &'static str {
        "\x1b[9m"
    }
    #[inline]
    pub fn double_underline() -> &'static str {
        "\x1b[21m"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create test ANSI instances
    fn create_ansi(r: u8, g: u8, b: u8) -> Ansi {
        Ansi::rgb(r, g, b)
    }

    mod constructors {
        use super::*;

        // Basic RGB constructor tests
        #[test]
        fn test_rgb_constructor() {
            let ansi = Ansi::rgb(255, 0, 0);
            assert_eq!(ansi.get_rgb(), (255, 0, 0));

            let ansi = Ansi::rgb(0, 255, 0);
            assert_eq!(ansi.get_rgb(), (0, 255, 0));

            let ansi = Ansi::rgb(0, 0, 255);
            assert_eq!(ansi.get_rgb(), (0, 0, 255));
        }

        #[test]
        fn test_rgb_constructor_edge_values() {
            // Test with minimum values
            let black = Ansi::rgb(0, 0, 0);
            assert_eq!(black.get_rgb(), (0, 0, 0));

            // Test with maximum values
            let white = Ansi::rgb(255, 255, 255);
            assert_eq!(white.get_rgb(), (255, 255, 255));

            // Test with mixed values
            let mixed = Ansi::rgb(128, 64, 32);
            assert_eq!(mixed.get_rgb(), (128, 64, 32));
        }

        // Full hex code tests with hash
        #[test]
        fn test_from_hex_full_with_hash() {
            // Test with uppercase hex
            let red = Ansi::from_hex("#FF0000").unwrap();
            assert_eq!(red.get_rgb(), (255, 0, 0));

            // Test with lowercase hex
            let green = Ansi::from_hex("#00ff00").unwrap();
            assert_eq!(green.get_rgb(), (0, 255, 0));

            // Test with mixed case
            let blue = Ansi::from_hex("#0000FF").unwrap();
            assert_eq!(blue.get_rgb(), (0, 0, 255));

            // Test with mixed values
            let purple = Ansi::from_hex("#800080").unwrap();
            assert_eq!(purple.get_rgb(), (128, 0, 128));
        }

        // Full hex code tests without hash
        #[test]
        fn test_from_hex_full_without_hash() {
            // Test with uppercase hex
            let red = Ansi::from_hex("FF0000").unwrap();
            assert_eq!(red.get_rgb(), (255, 0, 0));

            // Test with lowercase hex
            let green = Ansi::from_hex("00ff00").unwrap();
            assert_eq!(green.get_rgb(), (0, 255, 0));

            // Test with mixed case
            let blue = Ansi::from_hex("0000FF").unwrap();
            assert_eq!(blue.get_rgb(), (0, 0, 255));
        }

        // Short hex code tests with hash
        #[test]
        fn test_from_hex_short_with_hash() {
            // Test with uppercase hex
            let red = Ansi::from_hex("#F00").unwrap();
            assert_eq!(red.get_rgb(), (255, 0, 0));

            // Test with lowercase hex
            let green = Ansi::from_hex("#0f0").unwrap();
            assert_eq!(green.get_rgb(), (0, 255, 0));

            // Test with mixed case
            let blue = Ansi::from_hex("#00F").unwrap();
            assert_eq!(blue.get_rgb(), (0, 0, 255));
        }

        // Short hex code tests without hash
        #[test]
        fn test_from_hex_short_without_hash() {
            // Test with uppercase hex
            let red = Ansi::from_hex("F00").unwrap();
            assert_eq!(red.get_rgb(), (255, 0, 0));

            // Test with lowercase hex
            let green = Ansi::from_hex("0f0").unwrap();
            assert_eq!(green.get_rgb(), (0, 255, 0));

            // Test with mixed case
            let blue = Ansi::from_hex("00F").unwrap();
            assert_eq!(blue.get_rgb(), (0, 0, 255));
        }

        // Test get_rgb method
        #[test]
        fn test_get_rgb() {
            let ansi = Ansi::rgb(123, 45, 67);
            assert_eq!(ansi.get_rgb(), (123, 45, 67));

            let ansi = Ansi::from_hex("#FF00FF").unwrap();
            assert_eq!(ansi.get_rgb(), (255, 0, 255));
        }

        // Invalid hex code tests
        #[test]
        fn test_from_hex_invalid() {
            // Test with invalid length
            assert!(Ansi::from_hex("1234").is_none());
            assert!(Ansi::from_hex("#1234").is_none());
            assert!(Ansi::from_hex("12345").is_none());
            assert!(Ansi::from_hex("#12345").is_none());
            assert!(Ansi::from_hex("1234567").is_none());
            assert!(Ansi::from_hex("#1234567").is_none());
            assert!(Ansi::from_hex("123456789").is_none());
            assert!(Ansi::from_hex("#123456789").is_none());

            // Test with invalid characters
            assert!(Ansi::from_hex("GGGGGG").is_none());
            assert!(Ansi::from_hex("#GGGGGG").is_none());
            assert!(Ansi::from_hex("GGG").is_none());
            assert!(Ansi::from_hex("#GGG").is_none());
            assert!(Ansi::from_hex("GGGGGGGG").is_none());
            assert!(Ansi::from_hex("#GGGGGGGG").is_none());

            // Test with empty string
            assert!(Ansi::from_hex("").is_none());
            assert!(Ansi::from_hex("#").is_none());
        }

        // More invalid hex code tests
        #[test]
        fn test_from_hex_more_invalid_cases() {
            // Test with special characters
            assert!(Ansi::from_hex("!@#$%^").is_none());
            assert!(Ansi::from_hex("#!@#").is_none());

            // Test with spaces
            assert!(Ansi::from_hex("FF 00 00").is_none());
            assert!(Ansi::from_hex("F 0 0").is_none());
            assert!(Ansi::from_hex(" FF0000").is_none());
            assert!(Ansi::from_hex("FF0000 ").is_none());

            // Test with mixed valid and invalid characters
            assert!(Ansi::from_hex("FF00ZZ").is_none());
            assert!(Ansi::from_hex("FZ0").is_none());
        }

        // Edge case hex code tests
        #[test]
        fn test_from_hex_edge_cases() {
            // Test with black
            let black = Ansi::from_hex("#000000").unwrap();
            assert_eq!(black.get_rgb(), (0, 0, 0));

            // Test with white
            let white = Ansi::from_hex("#FFFFFF").unwrap();
            assert_eq!(white.get_rgb(), (255, 255, 255));

            // Test with gray values
            let gray = Ansi::from_hex("#808080").unwrap();
            assert_eq!(gray.get_rgb(), (128, 128, 128));
        }

        // More edge case hex code tests
        #[test]
        fn test_from_hex_more_edge_cases() {
            // Test with all zeros in short form
            let black_short = Ansi::from_hex("#000").unwrap();
            assert_eq!(black_short.get_rgb(), (0, 0, 0));

            // Test with all Fs in short form
            let white_short = Ansi::from_hex("#FFF").unwrap();
            assert_eq!(white_short.get_rgb(), (255, 255, 255));

            // Test with mixed values in short form
            let mixed_short = Ansi::from_hex("#F80").unwrap();
            assert_eq!(mixed_short.get_rgb(), (255, 136, 0));

            // Test with single digit values
            let single_digit = Ansi::from_hex("#123").unwrap();
            assert_eq!(single_digit.get_rgb(), (17, 34, 51));
        }

        #[test]
        fn test_from_hex_with_alpha() {
            // Test 8-digit hex codes with alpha channel
            let red_alpha = Ansi::from_hex("#FF0000FF").unwrap();
            assert_eq!(red_alpha.get_rgb(), (255, 0, 0));

            let green_alpha = Ansi::from_hex("#00FF0080").unwrap();
            assert_eq!(green_alpha.get_rgb(), (0, 255, 0));

            let blue_alpha = Ansi::from_hex("#0000FF40").unwrap();
            assert_eq!(blue_alpha.get_rgb(), (0, 0, 255));

            // Test without hash
            let red_alpha_no_hash = Ansi::from_hex("FF0000FF").unwrap();
            assert_eq!(red_alpha_no_hash.get_rgb(), (255, 0, 0));

            // Test with different alpha values
            let transparent = Ansi::from_hex("#FF000000").unwrap(); // Alpha = 00 (transparent)
            let semi = Ansi::from_hex("#FF000080").unwrap();        // Alpha = 80 (semi-transparent)
            let opaque = Ansi::from_hex("#FF0000FF").unwrap();      // Alpha = FF (opaque)

            // All should have the same RGB values regardless of alpha
            assert_eq!(transparent.get_rgb(), (255, 0, 0));
            assert_eq!(semi.get_rgb(), (255, 0, 0));
            assert_eq!(opaque.get_rgb(), (255, 0, 0));
        }

        // Test hex to ANSI foreground conversion
        #[test]
        fn test_hex_to_ansi_fg() {
            // Test foreground color from hex
            let red = Ansi::from_hex("#FF0000").unwrap();
            assert_eq!(red.fg(), "\x1b[38;2;255;0;0m");

            let green = Ansi::from_hex("#00FF00").unwrap();
            assert_eq!(green.fg(), "\x1b[38;2;0;255;0m");

            let blue = Ansi::from_hex("#0000FF").unwrap();
            assert_eq!(blue.fg(), "\x1b[38;2;0;0;255m");
        }

        // Test hex to ANSI background conversion
        #[test]
        fn test_hex_to_ansi_bg() {
            // Test background color from hex
            let red = Ansi::from_hex("#FF0000").unwrap();
            assert_eq!(red.bg(), "\x1b[48;2;255;0;0m");

            let green = Ansi::from_hex("#00FF00").unwrap();
            assert_eq!(green.bg(), "\x1b[48;2;0;255;0m");

            let blue = Ansi::from_hex("#0000FF").unwrap();
            assert_eq!(blue.bg(), "\x1b[48;2;0;0;255m");
        }

        // Test hex with formatting
        #[test]
        fn test_hex_with_formatting() {
            // Test combining hex colors with formatting
            let red = Ansi::from_hex("#FF0000").unwrap();
            let formatted_text = format!(
                "{}{}Bold Red Text{}",
                Ansi::bold(),
                red.fg(),
                Ansi::reset()
            );
            assert_eq!(formatted_text, "\x1b[1m\x1b[38;2;255;0;0mBold Red Text\x1b[0m");

            // Test with background color
            let blue = Ansi::from_hex("#0000FF").unwrap();
            let formatted_text = format!(
                "{}{}Bold Text on Blue Background{}",
                Ansi::bold(),
                blue.bg(),
                Ansi::reset()
            );
            assert_eq!(formatted_text, "\x1b[1m\x1b[48;2;0;0;255mBold Text on Blue Background\x1b[0m");
        }

        // Test hex with multiple styles
        #[test]
        fn test_hex_with_multiple_styles() {
            // Test combining hex colors with multiple styles
            let purple = Ansi::from_hex("#800080").unwrap();
            let formatted_text = format!(
                "{}{}{}Purple Bold Italic Text{}",
                Ansi::bold(),
                Ansi::italic(),
                purple.fg(),
                Ansi::reset()
            );
            assert_eq!(formatted_text, "\x1b[1m\x1b[3m\x1b[38;2;128;0;128mPurple Bold Italic Text\x1b[0m");
        }

        // Test hex with selective reset
        #[test]
        fn test_hex_with_selective_reset() {
            let cyan = Ansi::from_hex("#00FFFF").unwrap();
            let formatted_text = format!(
                "{}{}{}Cyan Bold Italic{}{}",
                Ansi::bold(),
                Ansi::italic(),
                cyan.fg(),
                Ansi::reset_italic(),
                " Still Bold Cyan"
            );
            assert_eq!(
                formatted_text,
                "\x1b[1m\x1b[3m\x1b[38;2;0;255;255mCyan Bold Italic\x1b[23m Still Bold Cyan"
            );
        }

        // Test RGB string parsing - CSS style
        #[test]
        fn test_from_rgb_str_css_style() {
            // Test with CSS-style RGB
            let red = Ansi::from_rgb_str("rgb(255, 0, 0)").unwrap();
            assert_eq!(red.get_rgb(), (255, 0, 0));

            // Test with no spaces
            let green = Ansi::from_rgb_str("rgb(0,255,0)").unwrap();
            assert_eq!(green.get_rgb(), (0, 255, 0));

            // Test with extra spaces
            let blue = Ansi::from_rgb_str("rgb( 0 , 0 , 255 )").unwrap();
            assert_eq!(blue.get_rgb(), (0, 0, 255));

            // Test with mixed case
            let purple = Ansi::from_rgb_str("RGB(128, 0, 128)").unwrap();
            assert_eq!(purple.get_rgb(), (128, 0, 128));
        }

        // Test RGB string parsing - comma-separated
        #[test]
        fn test_from_rgb_str_comma_separated() {
            // Test with comma-separated values
            let red = Ansi::from_rgb_str("255,0,0").unwrap();
            assert_eq!(red.get_rgb(), (255, 0, 0));

            // Test with spaces after commas
            let green = Ansi::from_rgb_str("0, 255, 0").unwrap();
            assert_eq!(green.get_rgb(), (0, 255, 0));

            // Test with spaces before and after commas
            let blue = Ansi::from_rgb_str("0 , 0 , 255").unwrap();
            assert_eq!(blue.get_rgb(), (0, 0, 255));
        }

        // Test RGB string parsing - space-separated
        #[test]
        fn test_from_rgb_str_space_separated() {
            // Test with space-separated values
            let red = Ansi::from_rgb_str("255 0 0").unwrap();
            assert_eq!(red.get_rgb(), (255, 0, 0));

            // Test with multiple spaces
            let green = Ansi::from_rgb_str("0  255  0").unwrap();
            assert_eq!(green.get_rgb(), (0, 255, 0));

            // Test with tabs
            let blue = Ansi::from_rgb_str("0\t0\t255").unwrap();
            assert_eq!(blue.get_rgb(), (0, 0, 255));
        }

        // Test RGB string parsing - invalid inputs
        #[test]
        fn test_from_rgb_str_invalid() {
            // Test with invalid format
            assert!(Ansi::from_rgb_str("rgb(255, 0)").is_none());
            assert!(Ansi::from_rgb_str("rgb(255, 0, 0, 0)").is_none());
            assert!(Ansi::from_rgb_str("rgb[255, 0, 0]").is_none());

            // Test with invalid values
            assert!(Ansi::from_rgb_str("256, 0, 0").is_none());
            assert!(Ansi::from_rgb_str("0, 256, 0").is_none());
            assert!(Ansi::from_rgb_str("0, 0, 256").is_none());

            // Test with non-numeric values
            assert!(Ansi::from_rgb_str("red, 0, 0").is_none());
            assert!(Ansi::from_rgb_str("0, green, 0").is_none());
            assert!(Ansi::from_rgb_str("0, 0, blue").is_none());

            // Test with empty string
            assert!(Ansi::from_rgb_str("").is_none());
            assert!(Ansi::from_rgb_str(" ").is_none());

            // Test with incomplete values
            assert!(Ansi::from_rgb_str("255").is_none());
            assert!(Ansi::from_rgb_str("255, 0").is_none());
            assert!(Ansi::from_rgb_str("rgb(255)").is_none());
            assert!(Ansi::from_rgb_str("rgb(255, 0)").is_none());
        }

        // Test RGB string to ANSI conversion
        #[test]
        fn test_rgb_str_to_ansi() {
            // Test foreground color from RGB string
            let red = Ansi::from_rgb_str("255, 0, 0").unwrap();
            assert_eq!(red.fg(), "\x1b[38;2;255;0;0m");

            let green = Ansi::from_rgb_str("0, 255, 0").unwrap();
            assert_eq!(green.fg(), "\x1b[38;2;0;255;0m");

            let blue = Ansi::from_rgb_str("0, 0, 255").unwrap();
            assert_eq!(blue.fg(), "\x1b[38;2;0;0;255m");

            // Test background color from RGB string
            let red = Ansi::from_rgb_str("255, 0, 0").unwrap();
            assert_eq!(red.bg(), "\x1b[48;2;255;0;0m");

            let green = Ansi::from_rgb_str("0, 255, 0").unwrap();
            assert_eq!(green.bg(), "\x1b[48;2;0;255;0m");

            let blue = Ansi::from_rgb_str("0, 0, 255").unwrap();
            assert_eq!(blue.bg(), "\x1b[48;2;0;0;255m");
        }

        // Test RGB string with formatting
        #[test]
        fn test_rgb_str_with_formatting() {
            // Test combining RGB string colors with formatting
            let red = Ansi::from_rgb_str("255, 0, 0").unwrap();
            let formatted_text = format!(
                "{}{}Bold Red Text{}",
                Ansi::bold(),
                red.fg(),
                Ansi::reset()
            );
            assert_eq!(formatted_text, "\x1b[1m\x1b[38;2;255;0;0mBold Red Text\x1b[0m");

            // Test with background color
            let blue = Ansi::from_rgb_str("0, 0, 255").unwrap();
            let formatted_text = format!(
                "{}{}Bold Text on Blue Background{}",
                Ansi::bold(),
                blue.bg(),
                Ansi::reset()
            );
            assert_eq!(formatted_text, "\x1b[1m\x1b[48;2;0;0;255mBold Text on Blue Background\x1b[0m");
        }

        // Test multiple hex colors in sequence
        #[test]
        fn test_multiple_hex_colors() {
            let red = Ansi::from_hex("#FF0000").unwrap();
            let green = Ansi::from_hex("#00FF00").unwrap();
            let blue = Ansi::from_hex("#0000FF").unwrap();

            let formatted_text = format!(
                "{}Red{} {}Green{} {}Blue{}",
                red.fg(),
                Ansi::reset(),
                green.fg(),
                Ansi::reset(),
                blue.fg(),
                Ansi::reset()
            );

            assert_eq!(
                formatted_text,
                "\x1b[38;2;255;0;0mRed\x1b[0m \x1b[38;2;0;255;0mGreen\x1b[0m \x1b[38;2;0;0;255mBlue\x1b[0m"
            );
        }

        // Test foreground and background together with hex
        #[test]
        fn test_hex_fg_and_bg_together() {
            let red = Ansi::from_hex("#FF0000").unwrap();
            let blue = Ansi::from_hex("#0000FF").unwrap();

            let formatted_text = format!(
                "{}{}Red on Blue{}",
                red.fg(),
                blue.bg(),
                Ansi::reset()
            );

            assert_eq!(
                formatted_text,
                "\x1b[38;2;255;0;0m\x1b[48;2;0;0;255mRed on Blue\x1b[0m"
            );
        }

        // Test hex colors with different formatting combinations
        #[test]
        fn test_hex_with_different_formatting() {
            let colors = [
                Ansi::from_hex("#FF0000").unwrap(), // Red
                Ansi::from_hex("#00FF00").unwrap(), // Green
                Ansi::from_hex("#0000FF").unwrap(), // Blue
            ];

            let styles = [
                Ansi::bold(),
                Ansi::italic(),
                Ansi::underline(),
            ];

            for (i, color) in colors.iter().enumerate() {
                let style = styles[i];
                let formatted = format!("{}{}{}", style, color.fg(), "Text");
                assert!(formatted.contains("Text"));
                assert!(formatted.contains(style));
                assert!(formatted.contains(&color.fg()));
            }
        }
    }

    // Module for RGB string specific tests
    mod rgb_string_specific {
        use super::*;

        #[test]
        fn test_rgb_str_edge_cases() {
            // Test with minimum values
            let black = Ansi::from_rgb_str("0, 0, 0").unwrap();
            assert_eq!(black.get_rgb(), (0, 0, 0));

            // Test with maximum values
            let white = Ansi::from_rgb_str("255, 255, 255").unwrap();
            assert_eq!(white.get_rgb(), (255, 255, 255));

            // Test with mixed values
            let gray = Ansi::from_rgb_str("128, 128, 128").unwrap();
            assert_eq!(gray.get_rgb(), (128, 128, 128));
        }

        #[test]
        fn test_rgb_str_boundary_values() {
            // Test with boundary values
            let almost_white = Ansi::from_rgb_str("254, 254, 254").unwrap();
            assert_eq!(almost_white.get_rgb(), (254, 254, 254));

            let almost_black = Ansi::from_rgb_str("1, 1, 1").unwrap();
            assert_eq!(almost_black.get_rgb(), (1, 1, 1));

            // Test with mixed boundary values
            let mixed = Ansi::from_rgb_str("0, 255, 1").unwrap();
            assert_eq!(mixed.get_rgb(), (0, 255, 1));
        }

        #[test]
        fn test_rgb_str_with_leading_zeros() {
            // Test with leading zeros
            let red = Ansi::from_rgb_str("0255, 000, 000").unwrap();
            assert_eq!(red.get_rgb(), (255, 0, 0));

            let green = Ansi::from_rgb_str("000, 0255, 000").unwrap();
            assert_eq!(green.get_rgb(), (0, 255, 0));

            let blue = Ansi::from_rgb_str("000, 000, 0255").unwrap();
            assert_eq!(blue.get_rgb(), (0, 0, 255));
        }

        #[test]
        fn test_rgb_str_with_different_separators() {
            // Test with different combinations of separators
            let mixed1 = Ansi::from_rgb_str("255,0 255").unwrap();
            assert_eq!(mixed1.get_rgb(), (255, 0, 255));

            let mixed2 = Ansi::from_rgb_str("255 0,255").unwrap();
            assert_eq!(mixed2.get_rgb(), (255, 0, 255));
        }

        #[test]
        fn test_rgb_str_real_world_examples() {
            // Test with real-world examples
            let coral = Ansi::from_rgb_str("255, 127, 80").unwrap();
            assert_eq!(coral.get_rgb(), (255, 127, 80));

            let teal = Ansi::from_rgb_str("0, 128, 128").unwrap();
            assert_eq!(teal.get_rgb(), (0, 128, 128));

            let gold = Ansi::from_rgb_str("255, 215, 0").unwrap();
            assert_eq!(gold.get_rgb(), (255, 215, 0));

            let indigo = Ansi::from_rgb_str("75, 0, 130").unwrap();
            assert_eq!(indigo.get_rgb(), (75, 0, 130));
        }

        #[test]
        fn test_rgb_str_css_variants() {
            // Test with CSS rgb function variants
            let red1 = Ansi::from_rgb_str("rgb(255, 0, 0)").unwrap();
            let red2 = Ansi::from_rgb_str("rgb(255,0,0)").unwrap();
            let red3 = Ansi::from_rgb_str("RGB(255, 0, 0)").unwrap();
            let red4 = Ansi::from_rgb_str("Rgb(255, 0, 0)").unwrap();

            assert_eq!(red1.get_rgb(), (255, 0, 0));
            assert_eq!(red2.get_rgb(), (255, 0, 0));
            assert_eq!(red3.get_rgb(), (255, 0, 0));
            assert_eq!(red4.get_rgb(), (255, 0, 0));
        }

        #[test]
        fn test_rgb_str_with_extra_whitespace() {
            // Test with extra whitespace
            let red1 = Ansi::from_rgb_str("  255  ,  0  ,  0  ").unwrap();
            let red2 = Ansi::from_rgb_str("\t255\t0\t0\t").unwrap();
            let red3 = Ansi::from_rgb_str("rgb(  255  ,  0  ,  0  )").unwrap();
            let red4 = Ansi::from_rgb_str("  rgb  (  255  ,  0  ,  0  )  ").unwrap();

            assert_eq!(red1.get_rgb(), (255, 0, 0));
            assert_eq!(red2.get_rgb(), (255, 0, 0));
            assert_eq!(red3.get_rgb(), (255, 0, 0));
            assert_eq!(red4.get_rgb(), (255, 0, 0));
        }

        #[test]
        fn test_rgb_str_with_unusual_separators() {
            // Test with unusual separator combinations
            let color1 = Ansi::from_rgb_str("255, 0 0").unwrap();
            let color2 = Ansi::from_rgb_str("255 , 0 , 0").unwrap();
            let color3 = Ansi::from_rgb_str("255,,0,,0").unwrap();
            let color4 = Ansi::from_rgb_str("255  0  0").unwrap();

            assert_eq!(color1.get_rgb(), (255, 0, 0));
            assert_eq!(color2.get_rgb(), (255, 0, 0));
            assert_eq!(color3.get_rgb(), (255, 0, 0));
            assert_eq!(color4.get_rgb(), (255, 0, 0));
        }

        #[test]
        fn test_rgb_str_with_decimal_values() {
            // Test with decimal values (should truncate to integers)
            let color1 = Ansi::from_rgb_str("255.5, 0.7, 0.2");
            let color2 = Ansi::from_rgb_str("255.99, 0.99, 0.99");

            // These should fail as we don't support decimal values
            assert!(color1.is_none());
            assert!(color2.is_none());
        }

        #[test]
        fn test_rgb_str_with_percentage_values() {
            // Test with percentage values (not supported)
            let color1 = Ansi::from_rgb_str("100%, 0%, 0%");
            let color2 = Ansi::from_rgb_str("rgb(100%, 0%, 0%)");

            // These should fail as we don't support percentage values
            assert!(color1.is_none());
            assert!(color2.is_none());
        }

        #[test]
        fn test_rgb_str_with_hex_in_rgb_function() {
            // Test with hex values in rgb function (not supported)
            let color = Ansi::from_rgb_str("rgb(FF, 00, 00)");

            // This should fail as we don't support hex values in rgb function
            assert!(color.is_none());
        }

        #[test]
        fn test_rgb_str_with_negative_values() {
            // Test with negative values (not supported)
            let color1 = Ansi::from_rgb_str("-255, 0, 0");
            let color2 = Ansi::from_rgb_str("255, -10, 0");
            let color3 = Ansi::from_rgb_str("255, 0, -20");

            // These should fail as we don't support negative values
            assert!(color1.is_none());
            assert!(color2.is_none());
            assert!(color3.is_none());
        }

        #[test]
        fn test_rgb_str_with_very_large_values() {
            // Test with values > 255 (not supported)
            let color1 = Ansi::from_rgb_str("256, 0, 0");
            let color2 = Ansi::from_rgb_str("255, 300, 0");
            let color3 = Ansi::from_rgb_str("255, 0, 1000");

            // These should fail as values must be in range 0-255
            assert!(color1.is_none());
            assert!(color2.is_none());
            assert!(color3.is_none());
        }

        #[test]
        fn test_rgb_str_with_mixed_notations() {
            // Test with mixed notations (not supported)
            let color1 = Ansi::from_rgb_str("rgb(255, 0, #00)");
            let color2 = Ansi::from_rgb_str("rgb(#FF, 0, 0)");

            // These should fail as we don't support mixed notations
            assert!(color1.is_none());
            assert!(color2.is_none());
        }

        #[test]
        fn test_rgb_str_performance() {
            // Test parsing the same RGB string multiple times
            let rgb_str = "rgb(255, 0, 0)";

            // Parse the same RGB string multiple times
            for _ in 0..100 {
                let color = Ansi::from_rgb_str(rgb_str).unwrap();
                assert_eq!(color.get_rgb(), (255, 0, 0));
            }
        }

        #[test]
        fn test_rgb_str_many_different_formats() {
            // Test many different valid formats
            let formats = [
                "255,0,0",
                "255, 0, 0",
                "255 0 0",
                "rgb(255,0,0)",
                "rgb(255, 0, 0)",
                "RGB(255,0,0)",
                "Rgb(255, 0, 0)",
                "  255  ,  0  ,  0  ",
                "\t255\t0\t0\t",
                "255,,0,,0",
                "255 , 0 , 0",
            ];

            for format in formats.iter() {
                let color = Ansi::from_rgb_str(format).unwrap();
                assert_eq!(color.get_rgb(), (255, 0, 0));
            }
        }
    }

    // Module for combining RGB string and hex methods
    mod combining_methods {
        use super::*;

        #[test]
        fn test_hex_and_rgb_str_equivalence() {
            // Test that hex and RGB string methods produce the same result
            let red_hex = Ansi::from_hex("#FF0000").unwrap();
            let red_rgb = Ansi::from_rgb_str("255, 0, 0").unwrap();
            assert_eq!(red_hex.get_rgb(), red_rgb.get_rgb());

            let green_hex = Ansi::from_hex("#00FF00").unwrap();
            let green_rgb = Ansi::from_rgb_str("0, 255, 0").unwrap();
            assert_eq!(green_hex.get_rgb(), green_rgb.get_rgb());

            let blue_hex = Ansi::from_hex("#0000FF").unwrap();
            let blue_rgb = Ansi::from_rgb_str("0, 0, 255").unwrap();
            assert_eq!(blue_hex.get_rgb(), blue_rgb.get_rgb());
        }

        #[test]
        fn test_hex_and_rgb_str_ansi_equivalence() {
            // Test that hex and RGB string methods produce the same ANSI codes
            let red_hex = Ansi::from_hex("#FF0000").unwrap();
            let red_rgb = Ansi::from_rgb_str("255, 0, 0").unwrap();
            assert_eq!(red_hex.fg(), red_rgb.fg());
            assert_eq!(red_hex.bg(), red_rgb.bg());

            let green_hex = Ansi::from_hex("#00FF00").unwrap();
            let green_rgb = Ansi::from_rgb_str("0, 255, 0").unwrap();
            assert_eq!(green_hex.fg(), green_rgb.fg());
            assert_eq!(green_hex.bg(), green_rgb.bg());

            let blue_hex = Ansi::from_hex("#0000FF").unwrap();
            let blue_rgb = Ansi::from_rgb_str("0, 0, 255").unwrap();
            assert_eq!(blue_hex.fg(), blue_rgb.fg());
            assert_eq!(blue_hex.bg(), blue_rgb.bg());
        }

        #[test]
        fn test_combining_hex_and_rgb_str() {
            // Test combining hex and RGB string colors
            let red_hex = Ansi::from_hex("#FF0000").unwrap();
            let blue_rgb = Ansi::from_rgb_str("0, 0, 255").unwrap();

            let formatted_text = format!(
                "{}Red{}{}Blue{}",
                red_hex.fg(),
                Ansi::reset(),
                blue_rgb.fg(),
                Ansi::reset()
            );

            assert_eq!(
                formatted_text,
                "\x1b[38;2;255;0;0mRed\x1b[0m\x1b[38;2;0;0;255mBlue\x1b[0m"
            );
        }

        #[test]
        fn test_rgb_constructor_and_parsers() {
            // Test that direct RGB constructor and parsers produce the same result
            let red_direct = Ansi::rgb(255, 0, 0);
            let red_hex = Ansi::from_hex("#FF0000").unwrap();
            let red_rgb = Ansi::from_rgb_str("255, 0, 0").unwrap();

            assert_eq!(red_direct.get_rgb(), red_hex.get_rgb());
            assert_eq!(red_direct.get_rgb(), red_rgb.get_rgb());
            assert_eq!(red_direct.fg(), red_hex.fg());
            assert_eq!(red_direct.fg(), red_rgb.fg());
            assert_eq!(red_direct.bg(), red_hex.bg());
            assert_eq!(red_direct.bg(), red_rgb.bg());
        }

        #[test]
        fn test_complex_color_combinations() {
            // Test combining multiple colors from different sources
            let red_hex = Ansi::from_hex("#FF0000").unwrap();
            let green_rgb = Ansi::from_rgb_str("0, 255, 0").unwrap();
            let blue_direct = Ansi::rgb(0, 0, 255);

            let text = format!(
                "{}Red{} {}Green{} {}Blue{}",
                red_hex.fg(),
                Ansi::reset(),
                green_rgb.fg(),
                Ansi::reset(),
                blue_direct.fg(),
                Ansi::reset()
            );

            assert!(text.contains("\x1b[38;2;255;0;0m"));
            assert!(text.contains("\x1b[38;2;0;255;0m"));
            assert!(text.contains("\x1b[38;2;0;0;255m"));
        }

        #[test]
        fn test_nested_color_combinations() {
            // Test nested color combinations
            let red_hex = Ansi::from_hex("#FF0000").unwrap();
            let green_rgb = Ansi::from_rgb_str("0, 255, 0").unwrap();

            let text = format!(
                "{}Red {}Green{}{}",
                red_hex.fg(),
                green_rgb.fg(),
                red_hex.fg(),
                Ansi::reset()
            );

            assert_eq!(
                text,
                "\x1b[38;2;255;0;0mRed \x1b[38;2;0;255;0mGreen\x1b[38;2;255;0;0m\x1b[0m"
            );
        }

        #[test]
        fn test_formatting_with_different_color_sources() {
            // Test formatting with colors from different sources
            let red_hex = Ansi::from_hex("#FF0000").unwrap();
            let green_rgb = Ansi::from_rgb_str("0, 255, 0").unwrap();
            let blue_direct = Ansi::rgb(0, 0, 255);

            let text1 = format!(
                "{}{}Bold Red{}",
                Ansi::bold(),
                red_hex.fg(),
                Ansi::reset()
            );

            let text2 = format!(
                "{}{}Italic Green{}",
                Ansi::italic(),
                green_rgb.fg(),
                Ansi::reset()
            );

            let text3 = format!(
                "{}{}Underlined Blue{}",
                Ansi::underline(),
                blue_direct.fg(),
                Ansi::reset()
            );

            assert_eq!(text1, "\x1b[1m\x1b[38;2;255;0;0mBold Red\x1b[0m");
            assert_eq!(text2, "\x1b[3m\x1b[38;2;0;255;0mItalic Green\x1b[0m");
            assert_eq!(text3, "\x1b[4m\x1b[38;2;0;0;255mUnderlined Blue\x1b[0m");
        }
    }

    // Module for real-world RGB string usage
    mod rgb_string_real_world {
        use super::*;

        #[test]
        fn test_rgb_terminal_prompt() {
            // Test creating a terminal prompt with RGB colors
            let username = "user";
            let hostname = "host";
            let directory = "~/projects";

            let user_color = Ansi::from_rgb_str("0, 255, 0").unwrap();
            let dir_color = Ansi::from_rgb_str("0, 128, 255").unwrap();

            let prompt = format!(
                "{}{}{}@{}{}:{}{}{}$ ",
                Ansi::bold(),
                user_color.fg(),
                username,
                hostname,
                Ansi::reset_bold(),
                dir_color.fg(),
                directory,
                Ansi::reset()
            );

            assert_eq!(
                prompt,
                "\x1b[1m\x1b[38;2;0;255;0muser@host\x1b[22m:\x1b[38;2;0;128;255m~/projects\x1b[0m$ "
            );
        }

        #[test]
        fn test_rgb_syntax_highlighting() {
            // Test syntax highlighting with RGB colors
            let keyword = Ansi::from_rgb_str("0, 0, 255").unwrap();
            let string = Ansi::from_rgb_str("0, 128, 0").unwrap();
            let comment = Ansi::from_rgb_str("128, 128, 128").unwrap();

            let code = format!(
                "{}{} {}{}({}{}{}) {{\n    {}{}// This is a comment{}\n    {}{}{}{}{}{}\n}}",
                keyword.fg(),
                "function",
                "greet",
                Ansi::reset(),
                keyword.fg(),
                "string",
                Ansi::reset(),
                comment.fg(),
                Ansi::italic(),
                Ansi::reset(),
                keyword.fg(),
                "return ",
                Ansi::reset(),
                string.fg(),
                "\"Hello, World!\"",
                Ansi::reset()
            );

            assert!(code.contains("\x1b[38;2;0;0;255mfunction"));
            assert!(code.contains("\x1b[38;2;0;128;0m\"Hello, World!\""));
            assert!(code.contains("\x1b[38;2;128;128;128m\x1b[3m// This is a comment"));
        }

        #[test]
        fn test_rgb_error_messages() {
            // Test error messages with RGB colors
            let error_color = Ansi::from_rgb_str("255, 0, 0").unwrap();
            let warning_color = Ansi::from_rgb_str("255, 165, 0").unwrap();
            let info_color = Ansi::from_rgb_str("0, 128, 255").unwrap();

            let error = format!(
                "{}{}ERROR:{} {}\n{}{}WARNING:{} {}\n{}{}INFO:{} {}",
                Ansi::bold(),
                error_color.fg(),
                Ansi::reset_bold(),
                "Failed to connect to database",
                Ansi::bold(),
                warning_color.fg(),
                Ansi::reset_bold(),
                "Connection timeout may occur",
                Ansi::bold(),
                info_color.fg(),
                Ansi::reset_bold(),
                "Retrying in 5 seconds"
            );

            assert!(error.contains("\x1b[1m\x1b[38;2;255;0;0mERROR:"));
            assert!(error.contains("\x1b[1m\x1b[38;2;255;165;0mWARNING:"));
            assert!(error.contains("\x1b[1m\x1b[38;2;0;128;255mINFO:"));
        }

        #[test]
        fn test_rgb_progress_bar() {
            // Test progress bar with RGB colors
            let progress_color = Ansi::from_rgb_str("0, 255, 0").unwrap();
            let remaining_color = Ansi::from_rgb_str("200, 200, 200").unwrap();

            let progress = 7;
            let total = 10;

            let mut bar = String::new();
            bar.push_str(&format!("{}", progress_color.fg()));
            for _ in 0..progress {
                bar.push('█');
            }
            bar.push_str(&format!("{}", remaining_color.fg()));
            for _ in progress..total {
                bar.push('█');
            }
            bar.push_str(&format!("{} {}/{}",
                Ansi::reset(),
                progress,
                total
            ));

            assert!(bar.contains("\x1b[38;2;0;255;0m"));
            assert!(bar.contains("\x1b[38;2;200;200;200m"));
            assert!(bar.contains("7/10"));
            assert_eq!(bar.chars().filter(|&c| c == '█').count(), 10);
        }
    }

    // Module for complex combinations
    mod complex_combinations {
        use super::*;

        #[test]
        fn test_rainbow_text() {
            // Test creating rainbow text with hex colors
            let colors = [
                "#FF0000", // Red
                "#FF7F00", // Orange
                "#FFFF00", // Yellow
                "#00FF00", // Green
                "#0000FF", // Blue
                "#4B0082", // Indigo
                "#9400D3", // Violet
            ];

            let text = "RAINBOW";
            let mut rainbow = String::new();

            for (i, c) in text.chars().enumerate() {
                let color = Ansi::from_hex(colors[i % colors.len()]).unwrap();
                rainbow.push_str(&format!("{}{}", color.fg(), c));
            }

            rainbow.push_str(&format!("{}", Ansi::reset()));

            // Don't assert exact length as it depends on implementation details
            assert!(rainbow.contains("\x1b[38;2;255;0;0mR"));
            assert!(rainbow.contains("\x1b[38;2;255;127;0mA"));
            assert!(rainbow.contains("\x1b[38;2;255;255;0mI"));
            assert!(rainbow.contains("\x1b[38;2;0;255;0mN"));
            assert!(rainbow.contains("\x1b[38;2;0;0;255mB"));
            assert!(rainbow.contains("\x1b[38;2;75;0;130mO"));
            assert!(rainbow.contains("\x1b[38;2;148;0;211mW"));
        }

        #[test]
        fn test_nested_formatting_with_hex() {
            // Test nested formatting with hex colors
            let outer = Ansi::from_hex("#FF0000").unwrap(); // Red
            let middle = Ansi::from_hex("#00FF00").unwrap(); // Green
            let inner = Ansi::from_hex("#0000FF").unwrap(); // Blue

            let nested = format!(
                "{}Outer {}Middle {}Inner{} Back to Middle{} Back to Outer{}",
                outer.fg(),
                middle.fg(),
                inner.fg(),
                middle.fg(),
                outer.fg(),
                Ansi::reset()
            );

            assert!(nested.contains("\x1b[38;2;255;0;0mOuter "));
            assert!(nested.contains("\x1b[38;2;0;255;0mMiddle "));
            assert!(nested.contains("\x1b[38;2;0;0;255mInner"));
            assert!(nested.contains("\x1b[38;2;0;255;0m Back to Middle"));
            assert!(nested.contains("\x1b[38;2;255;0;0m Back to Outer"));
        }

        #[test]
        fn test_gradient_text() {
            // Test creating gradient text with hex colors
            let text = "GRADIENT";
            let start_color = (255, 0, 0); // Red
            let end_color = (0, 0, 255);   // Blue

            let mut gradient = String::new();

            for (i, c) in text.chars().enumerate() {
                let factor = i as f32 / (text.len() - 1) as f32;
                let r = (start_color.0 as f32 * (1.0 - factor) + end_color.0 as f32 * factor) as u8;
                let g = (start_color.1 as f32 * (1.0 - factor) + end_color.1 as f32 * factor) as u8;
                let b = (start_color.2 as f32 * (1.0 - factor) + end_color.2 as f32 * factor) as u8;

                let color = Ansi::rgb(r, g, b);
                gradient.push_str(&format!("{}{}", color.fg(), c));
            }

            gradient.push_str(&format!("{}", Ansi::reset()));

            assert!(gradient.contains("\x1b[38;2;255;0;0mG"));
            assert!(gradient.contains("\x1b[38;2;0;0;255mT"));
        }

        #[test]
        fn test_all_formatting_with_hex() {
            // Test all formatting options with hex color
            let color = Ansi::from_hex("#FF00FF").unwrap(); // Magenta

            let styles = [
                Ansi::bold(),
                Ansi::dim(),
                Ansi::italic(),
                Ansi::underline(),
                Ansi::blink(),
                Ansi::inverse(),
                Ansi::strikethrough(),
            ];

            let mut formatted = color.fg();
            for style in styles.iter() {
                formatted.push_str(style);
            }
            formatted.push_str("All Styles");
            formatted.push_str(Ansi::reset());

            for style in styles.iter() {
                assert!(formatted.contains(style));
            }
            assert!(formatted.contains("\x1b[38;2;255;0;255m"));
            assert!(formatted.contains("All Styles"));
        }

        #[test]
        fn test_foreground_background_combinations() {
            // Test all combinations of foreground and background colors
            let colors = [
                "#FF0000", // Red
                "#00FF00", // Green
                "#0000FF", // Blue
            ];

            for fg_hex in colors.iter() {
                let fg = Ansi::from_hex(fg_hex).unwrap();

                for bg_hex in colors.iter() {
                    let bg = Ansi::from_hex(bg_hex).unwrap();

                    let formatted = format!(
                        "{}{}Text{}",
                        fg.fg(),
                        bg.bg(),
                        Ansi::reset()
                    );

                    assert!(formatted.contains(&fg.fg()));
                    assert!(formatted.contains(&bg.bg()));
                    assert!(formatted.contains("Text"));
                }
            }
        }
    }

    // New module for hex-specific tests
    mod hex_specific {
        use super::*;

        // Test CSS color names converted to hex
        #[test]
        fn test_css_color_names_as_hex() {
            // Common CSS color names and their hex values
            let color_map = [
                ("red", "#FF0000"),
                ("green", "#008000"),
                ("blue", "#0000FF"),
                ("yellow", "#FFFF00"),
                ("cyan", "#00FFFF"),
                ("magenta", "#FF00FF"),
                ("black", "#000000"),
                ("white", "#FFFFFF"),
            ];

            for (name, hex) in color_map.iter() {
                let color = Ansi::from_hex(hex).unwrap();
                let formatted = format!("{}{}{}", color.fg(), name, Ansi::reset());
                assert!(formatted.contains(name));
            }
        }

        // Test web-safe colors
        #[test]
        fn test_web_safe_colors() {
            // Test a few web-safe colors (multiples of 33 or 51)
            let web_safe_colors = [
                ("#000", (0, 0, 0)),
                ("#333", (51, 51, 51)),
                ("#666", (102, 102, 102)),
                ("#999", (153, 153, 153)),
                ("#CCC", (204, 204, 204)),
                ("#FFF", (255, 255, 255)),
                ("#F00", (255, 0, 0)),
                ("#0F0", (0, 255, 0)),
                ("#00F", (0, 0, 255)),
                ("#FF0", (255, 255, 0)),
                ("#0FF", (0, 255, 255)),
                ("#F0F", (255, 0, 255)),
            ];

            for (hex, rgb) in web_safe_colors.iter() {
                let color = Ansi::from_hex(hex).unwrap();
                assert_eq!(color.get_rgb(), *rgb);
            }
        }

        // Test hex color gradients
        #[test]
        fn test_hex_color_gradients() {
            // Test a simple gradient from black to white
            let steps = 5;
            let mut colors = Vec::with_capacity(steps);

            for i in 0..steps {
                let value = (i * 255) / (steps - 1);
                let hex = format!("#{:02X}{:02X}{:02X}", value, value, value);
                let color = Ansi::from_hex(&hex).unwrap();
                colors.push(color);
            }

            assert_eq!(colors[0].get_rgb(), (0, 0, 0)); // Black
            assert_eq!(colors[steps-1].get_rgb(), (255, 255, 255)); // White

            // Check intermediate values
            for i in 1..steps-1 {
                let (r, g, b) = colors[i].get_rgb();
                assert_eq!(r, g);
                assert_eq!(g, b);
                assert!(r > 0 && r < 255);
            }
        }

        // Test hex color with alpha channel (should handle and ignore alpha)
        #[test]
        fn test_hex_with_alpha_channel() {
            // 8-digit hex codes (RRGGBBAA) should be valid but ignore alpha
            let red_with_alpha = Ansi::from_hex("#FF0000FF").unwrap();
            assert_eq!(red_with_alpha.get_rgb(), (255, 0, 0));

            let green_with_alpha = Ansi::from_hex("00FF0080").unwrap();
            assert_eq!(green_with_alpha.get_rgb(), (0, 255, 0));

            let blue_with_alpha = Ansi::from_hex("#0000FF00").unwrap();
            assert_eq!(blue_with_alpha.get_rgb(), (0, 0, 255));

            // Test with different alpha values - should all produce the same RGB
            let colors = [
                Ansi::from_hex("#FF000000").unwrap(), // Alpha = 00
                Ansi::from_hex("#FF000080").unwrap(), // Alpha = 80
                Ansi::from_hex("#FF0000FF").unwrap(), // Alpha = FF
            ];

            for color in colors.iter() {
                assert_eq!(color.get_rgb(), (255, 0, 0));
            }
        }

        // Test hex color with whitespace (should be invalid)
        #[test]
        fn test_hex_with_whitespace() {
            assert!(Ansi::from_hex(" #FF0000").is_none());
            assert!(Ansi::from_hex("#FF0000 ").is_none());
            assert!(Ansi::from_hex("#FF 00 00").is_none());
        }

        // Test hex color with special characters (should be invalid)
        #[test]
        fn test_hex_with_special_chars() {
            assert!(Ansi::from_hex("#FF-00-00").is_none());
            assert!(Ansi::from_hex("#FF,00,00").is_none());
            assert!(Ansi::from_hex("#FF.00.00").is_none());
        }

        // Test hex color with multiple hash symbols (should be invalid)
        #[test]
        fn test_hex_with_multiple_hashes() {
            assert!(Ansi::from_hex("##FF0000").is_none());
            assert!(Ansi::from_hex("#FF#0000").is_none());
        }

        // Test hex color with unicode characters (should be invalid)
        #[test]
        fn test_hex_with_unicode() {
            // Use Unicode characters that won't cause indexing issues
            assert!(Ansi::from_hex("#FF00A\u{1F534}").is_none());
            assert!(Ansi::from_hex("#\u{1F534}0000").is_none());
            assert!(Ansi::from_hex("FF\u{1F534}00").is_none());
        }
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

        #[test]
        fn test_reset_bold_value() {
            assert_eq!(Ansi::reset_bold(), "\x1b[22m");
        }

        #[test]
        fn test_reset_italic_value() {
            assert_eq!(Ansi::reset_italic(), "\x1b[23m");
        }

        #[test]
        fn test_reset_underline_value() {
            assert_eq!(Ansi::reset_underline(), "\x1b[24m");
        }

        #[test]
        fn test_reset_formatting_value() {
            assert_eq!(Ansi::reset_formatting(), "\x1b[22;23;24;25;27;28;29m");
        }

        #[test]
        fn test_reset_after_multiple_styles() {
            // Test reset after applying multiple styles
            let text = format!(
                "{}{}{}Styled Text{}",
                Ansi::bold(),
                Ansi::italic(),
                Ansi::underline(),
                Ansi::reset()
            );
            assert_eq!(text, "\x1b[1m\x1b[3m\x1b[4mStyled Text\x1b[0m");
        }

        #[test]
        fn test_reset_bold_effect() {
            // Test that reset_bold only resets bold
            let text = format!(
                "{}{}{}Bold and Italic{} Just Italic",
                Ansi::bold(),
                Ansi::italic(),
                " - ",
                Ansi::reset_bold()
            );
            assert_eq!(text, "\x1b[1m\x1b[3m - Bold and Italic\x1b[22m Just Italic");
        }

        #[test]
        fn test_reset_formatting_keeps_colors() {
            // Test that reset_formatting keeps colors
            let blue = create_ansi(0, 0, 255);
            let text = format!(
                "{}{}{}Blue Bold Text{} Still Blue",
                blue.fg(),
                Ansi::bold(),
                " - ",
                Ansi::reset_formatting()
            );
            assert_eq!(
                text,
                "\x1b[38;2;0;0;255m\x1b[1m - Blue Bold Text\x1b[22;23;24;25;27;28;29m Still Blue"
            );
        }

        #[test]
        fn test_reset_vs_reset_formatting() {
            // Test difference between reset and reset_formatting
            let blue = create_ansi(0, 0, 255);
            let text1 = format!(
                "{}{}Blue Bold{}",
                blue.fg(),
                Ansi::bold(),
                Ansi::reset()
            );
            let text2 = format!(
                "{}{}Blue Bold{}",
                blue.fg(),
                Ansi::bold(),
                Ansi::reset_formatting()
            );

            assert_eq!(text1, "\x1b[38;2;0;0;255m\x1b[1mBlue Bold\x1b[0m");
            assert_eq!(text2, "\x1b[38;2;0;0;255m\x1b[1mBlue Bold\x1b[22;23;24;25;27;28;29m");
            assert_ne!(text1, text2);
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

        #[test]
        fn test_fg_and_bg_together() {
            // Test foreground and background colors together
            let red = create_ansi(255, 0, 0);
            let blue = create_ansi(0, 0, 255);
            let text = format!(
                "{}{}Red Text on Blue Background{}",
                red.fg(),
                blue.bg(),
                Ansi::reset()
            );

            assert_eq!(
                text,
                "\x1b[38;2;255;0;0m\x1b[48;2;0;0;255mRed Text on Blue Background\x1b[0m"
            );
        }

        #[test]
        fn test_multiple_colors_in_sequence() {
            // Test multiple colors in sequence
            let red = create_ansi(255, 0, 0);
            let green = create_ansi(0, 255, 0);
            let blue = create_ansi(0, 0, 255);

            let text = format!(
                "{}Red{} {}Green{} {}Blue{}",
                red.fg(),
                Ansi::reset(),
                green.fg(),
                Ansi::reset(),
                blue.fg(),
                Ansi::reset()
            );

            assert_eq!(
                text,
                "\x1b[38;2;255;0;0mRed\x1b[0m \x1b[38;2;0;255;0mGreen\x1b[0m \x1b[38;2;0;0;255mBlue\x1b[0m"
            );
        }

        #[test]
        fn test_nested_formatting() {
            // Test nested formatting (later formatting overrides earlier)
            let red = create_ansi(255, 0, 0);
            let green = create_ansi(0, 255, 0);

            let text = format!(
                "{}Red {}Green inside Red{} Back to Red{}",
                red.fg(),
                green.fg(),
                red.fg(),
                Ansi::reset()
            );

            assert_eq!(
                text,
                "\x1b[38;2;255;0;0mRed \x1b[38;2;0;255;0mGreen inside Red\x1b[38;2;255;0;0m Back to Red\x1b[0m"
            );
        }

        #[test]
        fn test_complex_formatting_combination() {
            // Test a complex combination of colors and formatting
            let red = create_ansi(255, 0, 0);
            let blue = create_ansi(0, 0, 255);

            let text = format!(
                "{}{}Bold Red{} {}{}{}Italic Blue Underlined{} {}Normal Text",
                Ansi::bold(),
                red.fg(),
                Ansi::reset(),
                Ansi::italic(),
                blue.fg(),
                Ansi::underline(),
                Ansi::reset(),
                "- "
            );

            assert_eq!(
                text,
                "\x1b[1m\x1b[38;2;255;0;0mBold Red\x1b[0m \x1b[3m\x1b[38;2;0;0;255m\x1b[4mItalic Blue Underlined\x1b[0m - Normal Text"
            );
        }

        #[test]
        fn test_selective_reset_in_complex_sequence() {
            // Test selective resets in a complex sequence
            let red = create_ansi(255, 0, 0);

            let text = format!(
                "{}{}{}Bold Red Underlined{}{} Bold Red{}",
                Ansi::bold(),
                red.fg(),
                Ansi::underline(),
                Ansi::reset_underline(),
                " - ",
                Ansi::reset()
            );

            assert_eq!(
                text,
                "\x1b[1m\x1b[38;2;255;0;0m\x1b[4mBold Red Underlined\x1b[24m -  Bold Red\x1b[0m"
            );
        }

        #[test]
        fn test_formatting_with_multiple_colors() {
            // Test formatting with multiple colors
            let colors = [
                create_ansi(255, 0, 0),    // Red
                create_ansi(0, 255, 0),    // Green
                create_ansi(0, 0, 255),    // Blue
                create_ansi(255, 255, 0),  // Yellow
                create_ansi(255, 0, 255),  // Magenta
            ];

            let mut text = String::from("");

            for (i, color) in colors.iter().enumerate() {
                text.push_str(&format!(
                    "{}{}Color {}{} ",
                    Ansi::bold(),
                    color.fg(),
                    i + 1,
                    Ansi::reset()
                ));
            }

            assert_eq!(
                text,
                "\x1b[1m\x1b[38;2;255;0;0mColor 1\x1b[0m \x1b[1m\x1b[38;2;0;255;0mColor 2\x1b[0m \x1b[1m\x1b[38;2;0;0;255mColor 3\x1b[0m \x1b[1m\x1b[38;2;255;255;0mColor 4\x1b[0m \x1b[1m\x1b[38;2;255;0;255mColor 5\x1b[0m "
            );
        }
    }

    mod formatting {
        use super::*;

        // Text style tests
        #[test]
        fn test_bold() {
            assert_eq!(Ansi::bold(), "\x1b[1m");
        }

        #[test]
        fn test_dim() {
            assert_eq!(Ansi::dim(), "\x1b[2m");
        }

        #[test]
        fn test_italic() {
            assert_eq!(Ansi::italic(), "\x1b[3m");
        }

        #[test]
        fn test_underline() {
            assert_eq!(Ansi::underline(), "\x1b[4m");
        }

        #[test]
        fn test_blink() {
            assert_eq!(Ansi::blink(), "\x1b[5m");
        }

        #[test]
        fn test_fast_blink() {
            assert_eq!(Ansi::fast_blink(), "\x1b[6m");
        }

        #[test]
        fn test_inverse() {
            assert_eq!(Ansi::inverse(), "\x1b[7m");
        }

        #[test]
        fn test_hidden() {
            assert_eq!(Ansi::hidden(), "\x1b[8m");
        }

        #[test]
        fn test_strikethrough() {
            assert_eq!(Ansi::strikethrough(), "\x1b[9m");
        }

        #[test]
        fn test_double_underline() {
            assert_eq!(Ansi::double_underline(), "\x1b[21m");
        }

        // Reset tests
        #[test]
        fn test_reset_bold() {
            assert_eq!(Ansi::reset_bold(), "\x1b[22m");
        }

        #[test]
        fn test_reset_italic() {
            assert_eq!(Ansi::reset_italic(), "\x1b[23m");
        }

        #[test]
        fn test_reset_underline() {
            assert_eq!(Ansi::reset_underline(), "\x1b[24m");
        }

        #[test]
        fn test_reset_formatting() {
            assert_eq!(Ansi::reset_formatting(), "\x1b[22;23;24;25;27;28;29m");
        }

        // Combination tests
        #[test]
        fn test_combined_formatting() {
            // Test combining multiple formatting options
            let formatted_text = format!(
                "{}{}Bold and Underlined{}",
                Ansi::bold(),
                Ansi::underline(),
                Ansi::reset()
            );
            assert_eq!(formatted_text, "\x1b[1m\x1b[4mBold and Underlined\x1b[0m");
        }

        #[test]
        fn test_formatting_with_color() {
            // Test combining formatting with color
            let red = create_ansi(255, 0, 0);
            let formatted_text = format!(
                "{}{}Bold Red Text{}",
                Ansi::bold(),
                red.fg(),
                Ansi::reset()
            );
            assert_eq!(formatted_text, "\x1b[1m\x1b[38;2;255;0;0mBold Red Text\x1b[0m");
        }

        #[test]
        fn test_selective_reset() {
            // Test selectively resetting formatting
            let formatted_text = format!(
                "{}{}Bold and Italic{}{}",
                Ansi::bold(),
                Ansi::italic(),
                Ansi::reset_italic(),
                " Still Bold"
            );
            assert_eq!(formatted_text, "\x1b[1m\x1b[3mBold and Italic\x1b[23m Still Bold");
        }

        #[test]
        fn test_multiple_selective_resets() {
            // Test multiple selective resets
            let formatted_text = format!(
                "{}{}{}Bold, Italic, and Underlined{}{}{} Only Bold",
                Ansi::bold(),
                Ansi::italic(),
                Ansi::underline(),
                Ansi::reset_underline(),
                Ansi::reset_italic(),
                " -"
            );
            assert_eq!(
                formatted_text,
                "\x1b[1m\x1b[3m\x1b[4mBold, Italic, and Underlined\x1b[24m\x1b[23m - Only Bold"
            );
        }

        #[test]
        fn test_reset_all_formatting_but_keep_colors() {
            // Test resetting all formatting but keeping colors
            let blue = create_ansi(0, 0, 255);
            let formatted_text = format!(
                "{}{}{}Blue Bold Italic Text{}{}",
                blue.fg(),
                Ansi::bold(),
                Ansi::italic(),
                Ansi::reset_formatting(),
                " Still Blue"
            );
            assert_eq!(
                formatted_text,
                "\x1b[38;2;0;0;255m\x1b[1m\x1b[3mBlue Bold Italic Text\x1b[22;23;24;25;27;28;29m Still Blue"
            );
        }

        #[test]
        fn test_all_text_styles_together() {
            // Test all text styles together
            let formatted_text = format!(
                "{}{}{}{}{}{}{}{}{}{}All Styles{}",
                Ansi::bold(),
                Ansi::dim(),
                Ansi::italic(),
                Ansi::underline(),
                Ansi::blink(),
                Ansi::fast_blink(),
                Ansi::inverse(),
                Ansi::hidden(),
                Ansi::strikethrough(),
                Ansi::double_underline(),
                Ansi::reset()
            );
            assert_eq!(
                formatted_text,
                "\x1b[1m\x1b[2m\x1b[3m\x1b[4m\x1b[5m\x1b[6m\x1b[7m\x1b[8m\x1b[9m\x1b[21mAll Styles\x1b[0m"
            );
        }

        #[test]
        fn test_background_with_formatting() {
            // Test background color with formatting
            let green = create_ansi(0, 255, 0);
            let formatted_text = format!(
                "{}{}{}Bold Text on Green Background{}",
                green.bg(),
                Ansi::bold(),
                Ansi::underline(),
                Ansi::reset()
            );
            assert_eq!(
                formatted_text,
                "\x1b[48;2;0;255;0m\x1b[1m\x1b[4mBold Text on Green Background\x1b[0m"
            );
        }

        #[test]
        fn test_foreground_background_with_formatting() {
            // Test foreground and background colors with formatting
            let red = create_ansi(255, 0, 0);
            let blue = create_ansi(0, 0, 255);
            let formatted_text = format!(
                "{}{}{}{}Red Bold Text on Blue Background{}",
                red.fg(),
                blue.bg(),
                Ansi::bold(),
                Ansi::italic(),
                Ansi::reset()
            );
            assert_eq!(
                formatted_text,
                "\x1b[38;2;255;0;0m\x1b[48;2;0;0;255m\x1b[1m\x1b[3mRed Bold Text on Blue Background\x1b[0m"
            );
        }

        #[test]
        fn test_inverse_with_colors() {
            // Test inverse with colors
            let red = create_ansi(255, 0, 0);
            let formatted_text = format!(
                "{}{}Normal Red{}{}Inverse Red{}",
                red.fg(),
                "Text - ",
                Ansi::inverse(),
                "Text - ",
                Ansi::reset()
            );
            assert_eq!(
                formatted_text,
                "\x1b[38;2;255;0;0mText - Normal Red\x1b[7mText - Inverse Red\x1b[0m"
            );
        }

        #[test]
        fn test_hidden_text() {
            // Test hidden text
            let formatted_text = format!(
                "Visible {}Hidden{} Visible Again",
                Ansi::hidden(),
                Ansi::reset()
            );
            assert_eq!(formatted_text, "Visible \x1b[8mHidden\x1b[0m Visible Again");
        }

        #[test]
        fn test_strikethrough_with_other_formatting() {
            // Test strikethrough with other formatting
            let formatted_text = format!(
                "{}{}{}Bold Italic Strikethrough{}",
                Ansi::bold(),
                Ansi::italic(),
                Ansi::strikethrough(),
                Ansi::reset()
            );
            assert_eq!(
                formatted_text,
                "\x1b[1m\x1b[3m\x1b[9mBold Italic Strikethrough\x1b[0m"
            );
        }

        #[test]
        fn test_double_underline_with_color() {
            // Test double underline with color
            let purple = create_ansi(128, 0, 128);
            let formatted_text = format!(
                "{}{}Purple Double Underlined{}",
                purple.fg(),
                Ansi::double_underline(),
                Ansi::reset()
            );
            assert_eq!(
                formatted_text,
                "\x1b[38;2;128;0;128m\x1b[21mPurple Double Underlined\x1b[0m"
            );
        }

        #[test]
        fn test_reset_specific_then_all() {
            // Test resetting specific formatting then all
            let formatted_text = format!(
                "{}{}{}Bold Italic Underlined{}{} Just Bold{}",
                Ansi::bold(),
                Ansi::italic(),
                Ansi::underline(),
                Ansi::reset_italic(),
                Ansi::reset_underline(),
                Ansi::reset()
            );
            assert_eq!(
                formatted_text,
                "\x1b[1m\x1b[3m\x1b[4mBold Italic Underlined\x1b[23m\x1b[24m Just Bold\x1b[0m"
            );
        }

        // Advanced formatting tests
        #[test]
        fn test_chained_formatting_operations() {
            // Test a chain of formatting operations
            let formatted_text = format!(
                "{}{}{}{}{}{}",
                Ansi::bold(),
                "Bold",
                Ansi::reset_bold(),
                " Normal ",
                Ansi::italic(),
                "Italic"
            );
            assert_eq!(formatted_text, "\x1b[1mBold\x1b[22m Normal \x1b[3mItalic");
        }

        #[test]
        fn test_alternating_styles() {
            // Test alternating between different styles
            let formatted_text = format!(
                "{}A{} {}B{} {}C{} {}D{}",
                Ansi::bold(),
                Ansi::reset(),
                Ansi::italic(),
                Ansi::reset(),
                Ansi::underline(),
                Ansi::reset(),
                Ansi::strikethrough(),
                Ansi::reset()
            );
            assert_eq!(
                formatted_text,
                "\x1b[1mA\x1b[0m \x1b[3mB\x1b[0m \x1b[4mC\x1b[0m \x1b[9mD\x1b[0m"
            );
        }

        #[test]
        fn test_nested_styles_with_selective_reset() {
            // Test nested styles with selective reset
            let formatted_text = format!(
                "{}Outer {}Inner{}{}",
                Ansi::bold(),
                Ansi::italic(),
                Ansi::reset_italic(),
                " Still Bold"
            );
            assert_eq!(formatted_text, "\x1b[1mOuter \x1b[3mInner\x1b[23m Still Bold");
        }

        #[test]
        fn test_multiple_style_combinations() {
            // Test various combinations of styles
            let combinations = [
                (Ansi::bold(), Ansi::italic(), "Bold+Italic"),
                (Ansi::bold(), Ansi::underline(), "Bold+Underline"),
                (Ansi::italic(), Ansi::strikethrough(), "Italic+Strikethrough"),
                (Ansi::underline(), Ansi::dim(), "Underline+Dim"),
                (Ansi::strikethrough(), Ansi::blink(), "Strikethrough+Blink"),
            ];

            for (style1, style2, text) in combinations.iter() {
                let formatted = format!("{}{}{}", style1, style2, text);
                assert!(formatted.contains(text));
                assert_eq!(formatted.len(), text.len() + style1.len() + style2.len());
            }
        }

        #[test]
        fn test_reset_formatting_chain() {
            // Test a chain of reset operations
            let formatted_text = format!(
                "{}{}{}{}{}{}{}{}Normal",
                Ansi::bold(),
                Ansi::italic(),
                Ansi::underline(),
                "Styled",
                Ansi::reset_bold(),
                Ansi::reset_italic(),
                Ansi::reset_underline(),
                " "
            );
            assert_eq!(
                formatted_text,
                "\x1b[1m\x1b[3m\x1b[4mStyled\x1b[22m\x1b[23m\x1b[24m Normal"
            );
        }

        #[test]
        fn test_style_overriding() {
            // Test that later styles override earlier ones
            let formatted_text = format!(
                "{}{}{}{}{}",
                Ansi::bold(),
                "Bold ",
                Ansi::reset_bold(),
                Ansi::italic(),
                "Italic"
            );
            assert_eq!(formatted_text, "\x1b[1mBold \x1b[22m\x1b[3mItalic");
        }
    }

    // New test module for real-world usage scenarios
    mod real_world_scenarios {
        use super::*;

        #[test]
        fn test_terminal_prompt_styling() {
            // Test styling similar to a terminal prompt
            let username = "user";
            let hostname = "host";
            let directory = "~/projects";

            // Using hex colors instead of RGB
            let green = Ansi::from_hex("#00FF00").unwrap();
            let blue = Ansi::from_hex("#0080FF").unwrap();

            let prompt = format!(
                "{}{}{}@{}{}:{}{}{}$ ",
                Ansi::bold(),
                green.fg(),
                username,
                hostname,
                Ansi::reset_bold(),
                blue.fg(),
                directory,
                Ansi::reset()
            );

            assert_eq!(
                prompt,
                "\x1b[1m\x1b[38;2;0;255;0muser@host\x1b[22m:\x1b[38;2;0;128;255m~/projects\x1b[0m$ "
            );
        }

        #[test]
        fn test_syntax_highlighting() {
            // Test styling similar to syntax highlighting
            // Using hex colors for syntax highlighting
            let keyword = Ansi::from_hex("#0000FF").unwrap();    // Blue
            let string = Ansi::from_hex("#008000").unwrap();     // Green
            let comment = Ansi::from_hex("#808080").unwrap();    // Gray

            let code = format!(
                "{}{} {}{}({}{}{}) {{\n    {}{}// This is a comment{}\n    {}{}{}{}{}{}\n}}",
                keyword.fg(),
                "function",
                "greet",
                Ansi::reset(),
                keyword.fg(),
                "string",
                Ansi::reset(),
                comment.fg(),
                Ansi::italic(),
                Ansi::reset(),
                keyword.fg(),
                "return ",
                Ansi::reset(),
                string.fg(),
                "\"Hello, World!\"",
                Ansi::reset()
            );

            assert!(code.contains("\x1b[38;2;0;0;255mfunction"));
            assert!(code.contains("\x1b[38;2;0;128;0m\"Hello, World!\""));
            assert!(code.contains("\x1b[38;2;128;128;128m\x1b[3m// This is a comment"));
        }

        #[test]
        fn test_error_message_formatting() {
            // Test styling similar to error messages
            // Using hex colors for error levels
            let error_color = Ansi::from_hex("#FF0000").unwrap();
            let warning_color = Ansi::from_hex("#FFA500").unwrap();
            let info_color = Ansi::from_hex("#0080FF").unwrap();

            let error = format!(
                "{}{}ERROR:{} {}\n{}{}WARNING:{} {}\n{}{}INFO:{} {}",
                Ansi::bold(),
                error_color.fg(),
                Ansi::reset_bold(),
                "Failed to connect to database",
                Ansi::bold(),
                warning_color.fg(),
                Ansi::reset_bold(),
                "Connection timeout may occur",
                Ansi::bold(),
                info_color.fg(),
                Ansi::reset_bold(),
                "Retrying in 5 seconds"
            );

            assert!(error.contains("\x1b[1m\x1b[38;2;255;0;0mERROR:"));
            assert!(error.contains("\x1b[1m\x1b[38;2;255;165;0mWARNING:"));
            assert!(error.contains("\x1b[1m\x1b[38;2;0;128;255mINFO:"));
        }

        #[test]
        fn test_progress_bar_styling() {
            // Test styling similar to a progress bar
            // Using hex colors for progress bar
            let progress_color = Ansi::from_hex("#00FF00").unwrap();
            let remaining_color = Ansi::from_hex("#C8C8C8").unwrap();

            let progress = 7;
            let total = 10;

            let mut bar = String::new();
            bar.push_str(&format!("{}", progress_color.fg()));
            for _ in 0..progress {
                bar.push('█');
            }
            bar.push_str(&format!("{}", remaining_color.fg()));
            for _ in progress..total {
                bar.push('█');
            }
            bar.push_str(&format!("{} {}/{}",
                Ansi::reset(),
                progress,
                total
            ));

            assert!(bar.contains("\x1b[38;2;0;255;0m"));
            assert!(bar.contains("\x1b[38;2;200;200;200m"));
            assert!(bar.contains("7/10"));
            assert_eq!(bar.chars().filter(|&c| c == '█').count(), 10);
        }

        #[test]
        fn test_git_diff_styling() {
            // Test styling similar to git diff output
            let added = Ansi::from_hex("#00FF00").unwrap();      // Green
            let removed = Ansi::from_hex("#FF0000").unwrap();    // Red
            let context = Ansi::from_hex("#808080").unwrap();    // Gray

            let diff = format!(
                "{}diff --git a/file.txt b/file.txt{}\n{}--- a/file.txt{}\n{}+++ b/file.txt{}\n{}@@ -1,3 +1,4 @@{}\n{} Line 1{}\n{}-Line 2{}\n{}+Line 2 modified{}\n{} Line 3{}\n{}+Line 4 added{}",
                context.fg(),
                Ansi::reset(),
                context.fg(),
                Ansi::reset(),
                context.fg(),
                Ansi::reset(),
                context.fg(),
                Ansi::reset(),
                context.fg(),
                Ansi::reset(),
                removed.fg(),
                Ansi::reset(),
                added.fg(),
                Ansi::reset(),
                context.fg(),
                Ansi::reset(),
                added.fg(),
                Ansi::reset()
            );

            assert!(diff.contains("\x1b[38;2;255;0;0m-Line 2"));
            assert!(diff.contains("\x1b[38;2;0;255;0m+Line 2 modified"));
            assert!(diff.contains("\x1b[38;2;0;255;0m+Line 4 added"));
        }

        #[test]
        fn test_log_level_styling() {
            // Test styling similar to log levels
            let levels = [
                ("TRACE", Ansi::from_hex("#808080").unwrap()),  // Gray
                ("DEBUG", Ansi::from_hex("#0080FF").unwrap()),  // Blue
                ("INFO", Ansi::from_hex("#00FF00").unwrap()),   // Green
                ("WARN", Ansi::from_hex("#FFFF00").unwrap()),   // Yellow
                ("ERROR", Ansi::from_hex("#FF0000").unwrap()),  // Red
                ("FATAL", Ansi::from_hex("#FF00FF").unwrap()),  // Magenta
            ];

            let mut log = String::new();

            for (level, color) in levels.iter() {
                log.push_str(&format!(
                    "{}{}[{}]{} Message at {} level\n",
                    Ansi::bold(),
                    color.fg(),
                    level,
                    Ansi::reset(),
                    level
                ));
            }

            for (level, _) in levels.iter() {
                assert!(log.contains(&format!("Message at {} level", level)));
            }

            assert!(log.contains("\x1b[1m\x1b[38;2;128;128;128m[TRACE]"));
            assert!(log.contains("\x1b[1m\x1b[38;2;0;128;255m[DEBUG]"));
            assert!(log.contains("\x1b[1m\x1b[38;2;0;255;0m[INFO]"));
            assert!(log.contains("\x1b[1m\x1b[38;2;255;255;0m[WARN]"));
            assert!(log.contains("\x1b[1m\x1b[38;2;255;0;0m[ERROR]"));
            assert!(log.contains("\x1b[1m\x1b[38;2;255;0;255m[FATAL]"));
        }

        #[test]
        fn test_markdown_styling() {
            // Test styling similar to markdown rendering
            let heading = Ansi::from_hex("#0000FF").unwrap();    // Blue
            let code = Ansi::from_hex("#FF0000").unwrap();       // Red
            let link = Ansi::from_hex("#00FF00").unwrap();       // Green
            let quote = Ansi::from_hex("#808080").unwrap();      // Gray

            let markdown = format!(
                "{}# Heading{}\n\nNormal text\n\n{}> This is a quote{}\n\n{}```\ncode block\n```{}\n\n{}[Link](https://example.com){}",
                heading.fg(),
                Ansi::reset(),
                quote.fg(),
                Ansi::reset(),
                code.fg(),
                Ansi::reset(),
                link.fg(),
                Ansi::reset()
            );

            assert!(markdown.contains("\x1b[38;2;0;0;255m# Heading"));
            assert!(markdown.contains("\x1b[38;2;128;128;128m> This is a quote"));
            assert!(markdown.contains("\x1b[38;2;255;0;0m```\ncode block\n```"));
            assert!(markdown.contains("\x1b[38;2;0;255;0m[Link](https://example.com)"));
        }

        #[test]
        fn test_calendar_styling() {
            // Test styling similar to a calendar
            let weekend = Ansi::from_hex("#FF0000").unwrap();    // Red
            let today = Ansi::from_hex("#00FF00").unwrap();      // Green
            let normal = Ansi::from_hex("#0000FF").unwrap();     // Blue
            let header = Ansi::from_hex("#FF00FF").unwrap();     // Magenta

            let calendar = format!(
                "{}  Mo Tu We Th Fr Sa Su{}\n{}   1  2  3  4  5 {}{} 6{}{} 7{}\n{}   8  9 {}10{} 11 12 {}13 14{}\n{} 15 16 17 18 19 {}20 21{}\n{} 22 23 24 25 26 {}27 28{}\n{} 29 30 31{}",
                header.fg(),
                Ansi::reset(),
                normal.fg(),
                Ansi::reset(),
                weekend.fg(),
                Ansi::reset(),
                weekend.fg(),
                Ansi::reset(),
                normal.fg(),
                today.fg(),
                Ansi::reset(),
                weekend.fg(),
                Ansi::reset(),
                normal.fg(),
                weekend.fg(),
                Ansi::reset(),
                normal.fg(),
                weekend.fg(),
                Ansi::reset(),
                normal.fg(),
                Ansi::reset()
            );

            assert!(calendar.contains("\x1b[38;2;255;0;255m  Mo Tu We Th Fr Sa Su"));
            assert!(calendar.contains("\x1b[38;2;255;0;0m 6"));
            assert!(calendar.contains("\x1b[38;2;0;255;0m10"));
        }
    }

    // New test module for edge cases
    mod edge_cases {
        use super::*;

        #[test]
        fn test_empty_string_with_formatting() {
            // Test formatting applied to empty strings
            let formatted = format!("{}{}{}", Ansi::bold(), "", Ansi::reset());
            assert_eq!(formatted, "\x1b[1m\x1b[0m");
        }

        #[test]
        fn test_multiple_consecutive_styles() {
            // Test applying multiple consecutive styles without text in between
            let formatted = format!(
                "{}{}{}{}{}Text{}",
                Ansi::bold(),
                Ansi::italic(),
                Ansi::underline(),
                Ansi::strikethrough(),
                Ansi::dim(),
                Ansi::reset()
            );
            assert_eq!(
                formatted,
                "\x1b[1m\x1b[3m\x1b[4m\x1b[9m\x1b[2mText\x1b[0m"
            );
        }

        #[test]
        fn test_multiple_consecutive_resets() {
            // Test applying multiple consecutive resets
            let formatted = format!(
                "{}{}Bold{}{}{}{}",
                Ansi::bold(),
                Ansi::italic(),
                Ansi::reset_bold(),
                Ansi::reset_italic(),
                Ansi::reset_formatting(),
                Ansi::reset()
            );
            assert_eq!(
                formatted,
                "\x1b[1m\x1b[3mBold\x1b[22m\x1b[23m\x1b[22;23;24;25;27;28;29m\x1b[0m"
            );
        }

        #[test]
        fn test_unicode_with_formatting() {
            // Test formatting with Unicode characters
            let formatted = format!(
                "{}{}{}{}{}",
                Ansi::bold(),
                "こんにちは",
                Ansi::reset_bold(),
                Ansi::italic(),
                "世界"
            );
            assert_eq!(formatted, "\x1b[1mこんにちは\x1b[22m\x1b[3m世界");
        }

        #[test]
        fn test_emoji_with_formatting() {
            // Test formatting with emoji
            let formatted = format!(
                "{}{}{}{}{}",
                Ansi::bold(),
                "🚀",
                Ansi::reset_bold(),
                Ansi::italic(),
                "🌟"
            );
            assert_eq!(formatted, "\x1b[1m🚀\x1b[22m\x1b[3m🌟");
        }

        #[test]
        fn test_newlines_with_formatting() {
            // Test formatting with newlines
            let formatted = format!(
                "{}\nLine 1\n{}\nLine 2\n{}",
                Ansi::bold(),
                Ansi::italic(),
                Ansi::reset()
            );
            assert_eq!(formatted, "\x1b[1m\nLine 1\n\x1b[3m\nLine 2\n\x1b[0m");
        }

        #[test]
        fn test_special_characters_with_formatting() {
            // Test formatting with special characters
            let formatted = format!(
                "{}{}{}{}{}",
                Ansi::bold(),
                "!@#$%^&*()",
                Ansi::reset_bold(),
                Ansi::italic(),
                "+-*/=<>?"
            );
            assert_eq!(formatted, "\x1b[1m!@#$%^&*()\x1b[22m\x1b[3m+-*/=<>?");
        }

        #[test]
        fn test_tab_characters_with_formatting() {
            // Test formatting with tab characters
            let formatted = format!(
                "{}\tTabbed\t{}\tText\t{}",
                Ansi::bold(),
                Ansi::italic(),
                Ansi::reset()
            );
            assert_eq!(formatted, "\x1b[1m\tTabbed\t\x1b[3m\tText\t\x1b[0m");
        }

        #[test]
        fn test_zero_width_characters_with_formatting() {
            // Test formatting with zero-width characters
            let formatted = format!(
                "{}A\u{200B}B{}",
                Ansi::bold(),
                Ansi::reset()
            );
            assert_eq!(formatted, "\x1b[1mA\u{200B}B\x1b[0m");
        }
    }

    // Module for RGB string edge cases
    mod rgb_string_edge_cases {
        use super::*;

        #[test]
        fn test_rgb_str_with_unicode_characters() {
            // Test with Unicode characters (should fail)
            let color1 = Ansi::from_rgb_str("255, 0, 0\u{1F534}");
            let color2 = Ansi::from_rgb_str("\u{1F534}255, 0, 0");
            let color3 = Ansi::from_rgb_str("255, \u{1F534}, 0");

            assert!(color1.is_none());
            assert!(color2.is_none());
            assert!(color3.is_none());
        }

        #[test]
        fn test_rgb_str_with_special_characters() {
            // Test with special characters (should fail)
            let color1 = Ansi::from_rgb_str("255, 0, 0!");
            let color2 = Ansi::from_rgb_str("@255, 0, 0");
            let color3 = Ansi::from_rgb_str("255, $, 0");

            assert!(color1.is_none());
            assert!(color2.is_none());
            assert!(color3.is_none());
        }

        #[test]
        fn test_rgb_str_with_empty_components() {
            // Test with empty components (should fail)
            let color1 = Ansi::from_rgb_str("255, , 0");
            let color2 = Ansi::from_rgb_str(", 0, 0");
            let color3 = Ansi::from_rgb_str("255, 0, ");

            assert!(color1.is_none());
            assert!(color2.is_none());
            assert!(color3.is_none());
        }

        #[test]
        fn test_rgb_str_with_too_many_components() {
            // Test with too many components (should fail)
            let color1 = Ansi::from_rgb_str("255, 0, 0, 0");
            let color2 = Ansi::from_rgb_str("255, 0, 0, 0, 0");
            let color3 = Ansi::from_rgb_str("rgb(255, 0, 0, 0)");

            assert!(color1.is_none());
            assert!(color2.is_none());
            assert!(color3.is_none());
        }

        #[test]
        fn test_rgb_str_with_too_few_components() {
            // Test with too few components (should fail)
            let color1 = Ansi::from_rgb_str("255");
            let color2 = Ansi::from_rgb_str("255, 0");
            let color3 = Ansi::from_rgb_str("rgb(255)");
            let color4 = Ansi::from_rgb_str("rgb(255, 0)");

            assert!(color1.is_none());
            assert!(color2.is_none());
            assert!(color3.is_none());
            assert!(color4.is_none());
        }

        #[test]
        fn test_rgb_str_with_non_numeric_components() {
            // Test with non-numeric components (should fail)
            let color1 = Ansi::from_rgb_str("red, 0, 0");
            let color2 = Ansi::from_rgb_str("255, green, 0");
            let color3 = Ansi::from_rgb_str("255, 0, blue");

            assert!(color1.is_none());
            assert!(color2.is_none());
            assert!(color3.is_none());
        }

        #[test]
        fn test_rgb_str_with_malformed_rgb_function() {
            // Test with malformed rgb function (should fail)
            let color1 = Ansi::from_rgb_str("rgb 255, 0, 0");
            let color2 = Ansi::from_rgb_str("rgb[255, 0, 0]");
            let color3 = Ansi::from_rgb_str("rgb{255, 0, 0}");
            let color4 = Ansi::from_rgb_str("rgb(255, 0, 0");
            let color5 = Ansi::from_rgb_str("rgb255, 0, 0)");

            assert!(color1.is_none());
            assert!(color2.is_none());
            assert!(color3.is_none());
            assert!(color4.is_none());
            assert!(color5.is_none());
        }

        #[test]
        fn test_rgb_str_with_extreme_values() {
            // Test with extreme values (boundary testing)
            let black = Ansi::from_rgb_str("0, 0, 0").unwrap();
            let white = Ansi::from_rgb_str("255, 255, 255").unwrap();
            let almost_black = Ansi::from_rgb_str("1, 1, 1").unwrap();
            let almost_white = Ansi::from_rgb_str("254, 254, 254").unwrap();

            assert_eq!(black.get_rgb(), (0, 0, 0));
            assert_eq!(white.get_rgb(), (255, 255, 255));
            assert_eq!(almost_black.get_rgb(), (1, 1, 1));
            assert_eq!(almost_white.get_rgb(), (254, 254, 254));
        }
    }

    // Module for performance-related tests
    mod performance {
        use super::*;

        #[test]
        fn test_repeated_hex_parsing() {
            // Test parsing the same hex code multiple times
            let hex = "#FF0000";

            // Parse the same hex code multiple times
            for _ in 0..100 {
                let color = Ansi::from_hex(hex).unwrap();
                assert_eq!(color.get_rgb(), (255, 0, 0));
            }
        }

        #[test]
        fn test_many_different_hex_codes() {
            // Test parsing many different hex codes
            let mut hex_codes = Vec::new();

            // Generate 100 different hex codes
            for r in (0..=255).step_by(51) {
                for g in (0..=255).step_by(51) {
                    for b in (0..=255).step_by(51) {
                        hex_codes.push(format!("#{:02X}{:02X}{:02X}", r, g, b));
                        if hex_codes.len() >= 100 {
                            break;
                        }
                    }
                    if hex_codes.len() >= 100 {
                        break;
                    }
                }
                if hex_codes.len() >= 100 {
                    break;
                }
            }

            // Parse all the hex codes
            for hex in hex_codes.iter() {
                let color = Ansi::from_hex(hex).unwrap();
                assert!(color.fg().contains("\x1b[38;2;"));
            }
        }

        #[test]
        fn test_large_text_formatting() {
            // Test formatting a large text with hex colors
            let text = "This is a test string that will be repeated multiple times to create a large text. ";
            let large_text = text.repeat(10); // 10 repetitions

            let color = Ansi::from_hex("#FF0000").unwrap();
            let formatted = format!("{}{}{}", color.fg(), large_text, Ansi::reset());

            assert!(formatted.starts_with("\x1b[38;2;255;0;0m"));
            assert!(formatted.ends_with("\x1b[0m"));
            assert_eq!(formatted.len(), large_text.len() + color.fg().len() + Ansi::reset().len());
        }

        #[test]
        fn test_many_color_changes() {
            // Test many color changes in a single string
            let colors = [
                "#FF0000", // Red
                "#00FF00", // Green
                "#0000FF", // Blue
            ];

            let mut formatted = String::new();

            // Apply 100 color changes
            for i in 0..100 {
                let color = Ansi::from_hex(colors[i % colors.len()]).unwrap();
                formatted.push_str(&color.fg());
                formatted.push_str("X");
            }

            formatted.push_str(Ansi::reset());

            // Count the number of color changes
            let color_changes = formatted.matches("\x1b[38;2;").count();
            assert_eq!(color_changes, 100);
        }

        #[test]
        fn test_hex_parsing_edge_cases_performance() {
            // Test parsing edge case hex codes many times
            let edge_cases = [
                "#000000", // Black
                "#FFFFFF", // White
                "#F00",    // Red (short)
                "#0F0",    // Green (short)
                "#00F",    // Blue (short)
            ];

            for hex in edge_cases.iter() {
                for _ in 0..20 {
                    let color = Ansi::from_hex(hex).unwrap();
                    assert!(color.fg().contains("\x1b[38;2;"));
                }
            }
        }

        #[test]
        fn test_many_different_rgb_strings() {
            // Test parsing many different RGB strings
            let mut rgb_strings = Vec::new();

            // Generate different RGB strings
            for r in (0..=255).step_by(51) {
                for g in (0..=255).step_by(51) {
                    for b in (0..=255).step_by(51) {
                        rgb_strings.push(format!("{}, {}, {}", r, g, b));
                        if rgb_strings.len() >= 50 {
                            break;
                        }
                    }
                    if rgb_strings.len() >= 50 {
                        break;
                    }
                }
                if rgb_strings.len() >= 50 {
                    break;
                }
            }

            // Parse all the RGB strings
            for rgb_str in rgb_strings.iter() {
                let color = Ansi::from_rgb_str(rgb_str).unwrap();
                assert!(color.fg().contains("\x1b[38;2;"));
            }
        }

        #[test]
        fn test_rgb_str_parsing_performance() {
            // Test parsing performance with different RGB string formats
            let formats = [
                "255, 0, 0",
                "rgb(255, 0, 0)",
                "255 0 0",
                "  255  ,  0  ,  0  ",
            ];

            for format in formats.iter() {
                for _ in 0..50 {
                    let color = Ansi::from_rgb_str(format).unwrap();
                    assert_eq!(color.get_rgb(), (255, 0, 0));
                }
            }
        }

        #[test]
        fn test_rgb_str_vs_hex_performance() {
            // Test performance comparison between RGB string and hex parsing
            let rgb_str = "255, 0, 0";
            let hex_str = "#FF0000";

            // Alternate between RGB string and hex parsing
            for _ in 0..50 {
                let color1 = Ansi::from_rgb_str(rgb_str).unwrap();
                let color2 = Ansi::from_hex(hex_str).unwrap();

                assert_eq!(color1.get_rgb(), (255, 0, 0));
                assert_eq!(color2.get_rgb(), (255, 0, 0));
                assert_eq!(color1.get_rgb(), color2.get_rgb());
            }
        }

        #[test]
        fn test_rgb_str_with_many_formats_performance() {
            // Test performance with many different RGB string formats
            let formats = [
                "255,0,0",
                "255, 0, 0",
                "255 0 0",
                "rgb(255,0,0)",
                "rgb(255, 0, 0)",
                "RGB(255,0,0)",
                "Rgb(255, 0, 0)",
                "  255  ,  0  ,  0  ",
                "\t255\t0\t0\t",
                "255,,0,,0",
                "255 , 0 , 0",
            ];

            for _ in 0..10 {
                for format in formats.iter() {
                    let color = Ansi::from_rgb_str(format).unwrap();
                    assert_eq!(color.get_rgb(), (255, 0, 0));
                }
            }
        }
    }

    mod table_formatting {
        use super::*;

        #[test]
        fn test_table_header_formatting() {
            // Test formatting for table headers
            let header_color = create_ansi(0, 0, 255);

            let header = format!(
                "{}{}| ID | Name | Role |{}",
                Ansi::bold(),
                header_color.fg(),
                Ansi::reset()
            );

            assert_eq!(
                header,
                "\x1b[1m\x1b[38;2;0;0;255m| ID | Name | Role |\x1b[0m"
            );
        }

        #[test]
        fn test_alternating_row_colors() {
            // Test alternating row colors in a table
            let even_row_color = create_ansi(240, 240, 240);
            let odd_row_color = create_ansi(255, 255, 255);

            let rows = [
                "| 1 | Alice | Admin |",
                "| 2 | Bob | User |",
                "| 3 | Charlie | Developer |",
            ];

            let mut table = String::new();

            for (i, row) in rows.iter().enumerate() {
                if i % 2 == 0 {
                    table.push_str(&format!("{}{}{}", odd_row_color.fg(), row, Ansi::reset()));
                } else {
                    table.push_str(&format!("{}{}{}", even_row_color.fg(), row, Ansi::reset()));
                }
                table.push('\n');
            }

            assert!(table.contains("\x1b[38;2;255;255;255m| 1 | Alice | Admin |"));
            assert!(table.contains("\x1b[38;2;240;240;240m| 2 | Bob | User |"));
            assert!(table.contains("\x1b[38;2;255;255;255m| 3 | Charlie | Developer |"));
        }

        #[test]
        fn test_cell_highlighting() {
            // Test highlighting specific cells in a table
            let highlight_color = create_ansi(255, 255, 0);

            let cell_data = [
                ("Alice", false),
                ("Bob", true),
                ("Charlie", false),
            ];

            let mut table = String::new();

            for (name, highlight) in cell_data.iter() {
                if *highlight {
                    table.push_str(&format!("| {}{}{} |", highlight_color.fg(), name, Ansi::reset()));
                } else {
                    table.push_str(&format!("| {} |", name));
                }
                table.push('\n');
            }

            assert!(table.contains("| Alice |"));
            assert!(table.contains("| \x1b[38;2;255;255;0mBob\x1b[0m |"));
            assert!(table.contains("| Charlie |"));
        }
    }
}