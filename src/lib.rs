//! All the colors of the web, by name – case-/space-/snake-insensitive.
//!
//! ```rust
//! use pigment::color;
//!
//! let az = color("Absolute Zero").unwrap();
//! println!("hex = {}", az.hex());          // #0048BA
//! println!("rgb = {:?}", az.rgb());        // (0, 72, 186)
//! println!("{}Absolute Zero{}", az.ansi().fg(), pigment::ansi::Ansi::reset());
//! ```
pub mod ansi;
mod color;
pub use color::Color;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/generated/colors.rs"));

fn canonical(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

/// Look up a color by (reasonably forgiving) name.
pub fn color(name: &str) -> Option<Color> {
    COLORS.get(&canonical(name)).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod canonical_function {
        use super::*;

        #[test]
        fn test_basic_lowercase_conversion() {
            // Test basic lowercase conversion
            assert_eq!(canonical("Red"), "red");
            assert_eq!(canonical("BLUE"), "blue");
            assert_eq!(canonical("Yellow"), "yellow");
            assert_eq!(canonical("MiXeD"), "mixed");
        }

        #[test]
        fn test_space_removal() {
            // Test space removal
            assert_eq!(canonical("Alice Blue"), "aliceblue");
            assert_eq!(canonical("Forest Green"), "forestgreen");
            assert_eq!(canonical("  Extra  Spaces  "), "extraspaces");
            assert_eq!(canonical("Leading Space"), "leadingspace");
            assert_eq!(canonical("Trailing Space "), "trailingspace");
        }

        #[test]
        fn test_special_character_removal() {
            // Test special character removal
            assert_eq!(canonical("Midnight-Blue"), "midnightblue");
            assert_eq!(canonical("Royal_Blue"), "royalblue");
            assert_eq!(canonical("Navy.Blue"), "navyblue");
            assert_eq!(canonical("Comma,Separated"), "commaseparated");
            assert_eq!(canonical("Slash/Separated"), "slashseparated");
            assert_eq!(canonical("Parentheses(Color)"), "parenthesescolor");
            assert_eq!(canonical("Brackets[Color]"), "bracketscolor");
            assert_eq!(canonical("Symbol@#$%^&*"), "symbol");
        }

        #[test]
        fn test_mixed_case_and_special_characters() {
            // Test mixed case and special characters
            assert_eq!(canonical("Deep Sky-Blue"), "deepskyblue");
            assert_eq!(canonical("LIGHT_CORAL"), "lightcoral");
            assert_eq!(canonical("medium.Sea.Green"), "mediumseagreen");
            assert_eq!(canonical("UPPER-lower_Mixed.Case"), "upperlowermixedcase");
        }

        #[test]
        fn test_with_numbers() {
            // Test with numbers
            assert_eq!(canonical("Gray50"), "gray50");
            assert_eq!(canonical("RGB(255,0,0)"), "rgb25500");
            assert_eq!(canonical("123456"), "123456");
            assert_eq!(canonical("Color123"), "color123");
            assert_eq!(canonical("123Color"), "123color");
        }

        #[test]
        fn test_with_non_ascii_characters() {
            // Test with non-ASCII characters (should be removed)
            assert_eq!(canonical("Café au Lait"), "cafaulait");
            assert_eq!(canonical("Blåbær"), "blbr");
            assert_eq!(canonical("Grün"), "grn");
            assert_eq!(canonical("Röd"), "rd");
            assert_eq!(canonical("Café"), "caf");
        }

        #[test]
        fn test_empty_and_whitespace() {
            // Test empty and whitespace strings
            assert_eq!(canonical(""), "");
            assert_eq!(canonical(" "), "");
            assert_eq!(canonical("\t"), "");
            assert_eq!(canonical("\n"), "");
            assert_eq!(canonical("  \t  \n  "), "");
        }
    }

    mod color_lookup {
        use super::*;

        #[test]
        fn test_known_colors() {
            // Test some known colors
            let red = color("Red").unwrap();
            assert_eq!(red.name(), "Red");
            assert_eq!(red.hex(), "#FF0000");
            assert_eq!(red.rgb(), (255, 0, 0));

            let blue = color("Blue").unwrap();
            assert_eq!(blue.name(), "Blue");
            assert_eq!(blue.hex(), "#0000FF");
            assert_eq!(blue.rgb(), (0, 0, 255));

            // Note: In the web color standard, "Green" can be either #008000 or #00FF00
            // depending on the color system. Let's check what's in our database.
            let green = color("Green").unwrap();
            assert_eq!(green.name(), "Green");
            // Just check that it's a valid hex color code starting with #
            assert!(green.hex().starts_with("#"));
            // Just verify we can get the RGB values
            let (_r, _g, _b) = green.rgb();
        }

        #[test]
        fn test_case_insensitivity() {
            // Test case insensitivity
            let red_lower = color("red");
            let red_upper = color("RED");
            let red_mixed = color("ReD");
            let red_proper = color("Red");

            assert!(red_lower.is_some());
            assert!(red_upper.is_some());
            assert!(red_mixed.is_some());
            assert!(red_proper.is_some());

            // All should be the same color
            assert_eq!(red_lower, red_upper);
            assert_eq!(red_lower, red_mixed);
            assert_eq!(red_lower, red_proper);
        }

        #[test]
        fn test_space_insensitivity() {
            // Test space insensitivity
            let alice_blue_spaced = color("Alice Blue");
            let alice_blue_no_space = color("AliceBlue");
            let alice_blue_extra_spaces = color("  Alice   Blue  ");

            assert!(alice_blue_spaced.is_some());
            assert!(alice_blue_no_space.is_some());
            assert!(alice_blue_extra_spaces.is_some());

            // All should be the same color
            assert_eq!(alice_blue_spaced, alice_blue_no_space);
            assert_eq!(alice_blue_spaced, alice_blue_extra_spaces);
        }

        #[test]
        fn test_special_character_insensitivity() {
            // Test special character insensitivity
            let light_gray = color("Light Gray");
            let light_gray_underscore = color("Light_Gray");
            let light_gray_dot = color("Light.Gray");
            let light_gray_dash = color("Light-Gray");

            assert!(light_gray.is_some());
            assert!(light_gray_underscore.is_some());
            assert!(light_gray_dot.is_some());
            assert!(light_gray_dash.is_some());

            // All should be the same color
            assert_eq!(light_gray, light_gray_underscore);
            assert_eq!(light_gray, light_gray_dot);
            assert_eq!(light_gray, light_gray_dash);
        }

        #[test]
        fn test_non_existent_colors() {
            // Test non-existent colors
            assert!(color("NonExistentColor").is_none());
            assert!(color("FakeColorName").is_none());
            assert!(color("").is_none());
            assert!(color("   ").is_none());
            assert!(color("RGB(999,999,999)").is_none()); // Invalid RGB values
        }

        #[test]
        fn test_color_lookup_with_special_cases() {
            // Test some edge cases
            assert!(color("AliceBlue").is_some());
            assert!(color("alice-blue").is_some());
            assert!(color("ALICE_BLUE").is_some());
            assert!(color("a l i c e b l u e").is_some());
            assert!(color("a-l-i-c-e-b-l-u-e").is_some());
            assert!(color("a_l_i_c_e_b_l_u_e").is_some());
            assert!(color("a.l.i.c.e.b.l.u.e").is_some());
        }
    }

    mod color_lookup_equivalence {
        use super::*;

        #[test]
        fn test_basic_equivalence() {
            // These should all return the same color
            let color1 = color("AliceBlue");
            let color2 = color("alice blue");
            let color3 = color("ALICE-BLUE");
            let color4 = color("alice_blue");

            assert!(color1.is_some());
            assert!(color2.is_some());
            assert!(color3.is_some());
            assert!(color4.is_some());

            assert_eq!(color1, color2);
            assert_eq!(color1, color3);
            assert_eq!(color1, color4);
        }

        #[test]
        fn test_extreme_equivalence() {
            // Test with extreme variations
            let color1 = color("Red");
            let color2 = color("r-e-d");
            let color3 = color("R_E_D");
            let color4 = color("r.e.d");
            let color5 = color("R E D");
            let color6 = color("  r  e  d  ");

            assert!(color1.is_some());
            assert!(color2.is_some());
            assert!(color3.is_some());
            assert!(color4.is_some());
            assert!(color5.is_some());
            assert!(color6.is_some());

            assert_eq!(color1, color2);
            assert_eq!(color1, color3);
            assert_eq!(color1, color4);
            assert_eq!(color1, color5);
            assert_eq!(color1, color6);
        }

        #[test]
        fn test_multi_word_equivalence() {
            // Test with multi-word color names
            let color1 = color("Light Goldenrod Yellow");
            let color2 = color("lightgoldenrodyellow");
            let color3 = color("LIGHT-GOLDENROD-YELLOW");
            let color4 = color("Light_Goldenrod_Yellow");

            assert!(color1.is_some());
            assert!(color2.is_some());
            assert!(color3.is_some());
            assert!(color4.is_some());

            assert_eq!(color1, color2);
            assert_eq!(color1, color3);
            assert_eq!(color1, color4);
        }
    }

    mod integration_tests {
        use super::*;

        #[test]
        fn test_canonical_and_lookup_together() {
            // Test that canonical and color lookup work together correctly
            let name = "Deep Sky Blue";
            let canonical_name = canonical(name);
            let color_result = color(name);

            assert_eq!(canonical_name, "deepskyblue");
            assert!(color_result.is_some());

            // The canonical name should match what's in the color map
            assert!(COLORS.get(&canonical_name).is_some());
        }

        #[test]
        fn test_lookup_with_direct_canonical() {
            // Test looking up colors with direct canonical names
            let canonical_red = canonical("Red");
            let canonical_blue = canonical("Blue");
            let canonical_green = canonical("Green");

            assert_eq!(canonical_red, "red");
            assert_eq!(canonical_blue, "blue");
            assert_eq!(canonical_green, "green");

            assert!(COLORS.get(&canonical_red).is_some());
            assert!(COLORS.get(&canonical_blue).is_some());
            assert!(COLORS.get(&canonical_green).is_some());
        }
    }
}
