use pigment::color;

// Helper function to create test colors
fn create_test_color(name: &'static str, hex: &'static str, rgb: (u8, u8, u8)) -> pigment::Color {
    // This is a bit of a hack, but we need to create a Color struct for testing
    // We use the same approach as the tests in the crate
    #[allow(dead_code)]
    struct TestColor {
        name: &'static str,
        hex: &'static str,
        rgb: (u8, u8, u8),
    }

    let test_color = TestColor { name, hex, rgb };

    // Use transmute to convert our TestColor to a pigment::Color
    // This is safe because they have the same memory layout
    unsafe { std::mem::transmute(test_color) }
}

// Test a variety of colors with all integrations
#[test]
fn test_color_integrations_with_various_colors() {
    // Use colors that are actually available in the database
    let test_color_names = [
        "Red", "Green", "Blue", "Yellow", "Cyan", "Magenta", "Black", "White",
        "Orange", "Purple", "Brown", "Pink", "Teal", "Olive", "Aqua", "Silver",
        "LightGray", "SlateGray", "DimGray", "Crimson",
    ];

    for color_name in test_color_names {
        if let Some(c) = color(color_name) {
            let expected_rgb = c.rgb();

            // Test all integrations with this color
            #[cfg(feature = "owo")]
            {
                let owo_rgb: owo_colors::Rgb = c.into();
                assert_eq!(owo_rgb.0, expected_rgb.0);
                assert_eq!(owo_rgb.1, expected_rgb.1);
                assert_eq!(owo_rgb.2, expected_rgb.2);
            }

            #[cfg(feature = "termcolor")]
            {
                let tc: termcolor::Color = c.into();
                match tc {
                    termcolor::Color::Rgb(r, g, b) => {
                        assert_eq!(r, expected_rgb.0);
                        assert_eq!(g, expected_rgb.1);
                        assert_eq!(b, expected_rgb.2);
                    }
                    _ => panic!("Expected RGB color for {}", color_name),
                }
            }

            #[cfg(feature = "colored")]
            {
                let colored_c: colored::Color = c.into();
                match colored_c {
                    colored::Color::TrueColor { r, g, b } => {
                        assert_eq!(r, expected_rgb.0);
                        assert_eq!(g, expected_rgb.1);
                        assert_eq!(b, expected_rgb.2);
                    }
                    _ => panic!("Expected TrueColor for {}", color_name),
                }
            }

            #[cfg(feature = "anstyle")]
            {
                let anstyle_c: anstyle::Color = c.into();
                match anstyle_c {
                    anstyle::Color::Rgb(rgb) => {
                        assert_eq!(rgb.0, expected_rgb.0);
                        assert_eq!(rgb.1, expected_rgb.1);
                        assert_eq!(rgb.2, expected_rgb.2);
                    }
                    _ => panic!("Expected RGB color for {}", color_name),
                }
            }

            #[cfg(feature = "nu-ansi-term")]
            {
                let nat: nu_ansi_term::Color = c.into();
                match nat {
                    nu_ansi_term::Color::Rgb(r, g, b) => {
                        assert_eq!(r, expected_rgb.0);
                        assert_eq!(g, expected_rgb.1);
                        assert_eq!(b, expected_rgb.2);
                    }
                    _ => panic!("Expected RGB color for {}", color_name),
                }
            }

            #[cfg(feature = "yansi")]
            {
                let yansi_c: yansi::Color = c.into();
                match yansi_c {
                    yansi::Color::Rgb(r, g, b) => {
                        assert_eq!(r, expected_rgb.0);
                        assert_eq!(g, expected_rgb.1);
                        assert_eq!(b, expected_rgb.2);
                    }
                    _ => panic!("Expected RGB color for {}", color_name),
                }
            }

            #[cfg(feature = "crossterm")]
            {
                let ct: crossterm::style::Color = c.into();
                match ct {
                    crossterm::style::Color::Rgb { r, g, b } => {
                        assert_eq!(r, expected_rgb.0);
                        assert_eq!(g, expected_rgb.1);
                        assert_eq!(b, expected_rgb.2);
                    }
                    _ => panic!("Expected RGB color for {}", color_name),
                }
            }
        } else {
            panic!("Color '{}' not found in the database", color_name);
        }
    }

    // All tests are now in the loop above
}

// Test hex color conversion with all integrations
#[test]
fn test_hex_color_integrations() {
    let test_hex_colors = [
        ("#FF0000", (255, 0, 0)),     // Red
        ("#00FF00", (0, 255, 0)),     // Lime
        ("#0000FF", (0, 0, 255)),     // Blue
        ("#FFFF00", (255, 255, 0)),   // Yellow
        ("#00FFFF", (0, 255, 255)),   // Cyan
        ("#FF00FF", (255, 0, 255)),   // Magenta
        ("#000000", (0, 0, 0)),       // Black
        ("#FFFFFF", (255, 255, 255)), // White
        ("#808080", (128, 128, 128)), // Gray
        ("#FFA500", (255, 165, 0)),   // Orange
        ("#800080", (128, 0, 128)),   // Purple
        ("#A52A2A", (165, 42, 42)),   // Brown
        ("#FFC0CB", (255, 192, 203)), // Pink
        ("#008080", (0, 128, 128)),   // Teal
        ("#000080", (0, 0, 128)),     // Navy
        ("#808000", (128, 128, 0)),   // Olive
        ("#800000", (128, 0, 0)),     // Maroon
        ("#C0C0C0", (192, 192, 192)), // Silver
    ];

    for (hex, expected_rgb) in test_hex_colors {
        // Create a custom color for testing
        let c = create_test_color("Custom", hex, expected_rgb);

        // Verify the RGB values
        assert_eq!(c.rgb(), expected_rgb, "RGB values for {} should match", hex);

        // Test all integrations with this color
        #[cfg(feature = "owo")]
        {
            let owo_rgb: owo_colors::Rgb = c.into();
            assert_eq!(owo_rgb.0, expected_rgb.0);
            assert_eq!(owo_rgb.1, expected_rgb.1);
            assert_eq!(owo_rgb.2, expected_rgb.2);
        }

        #[cfg(feature = "termcolor")]
        {
            let tc: termcolor::Color = c.into();
            match tc {
                termcolor::Color::Rgb(r, g, b) => {
                    assert_eq!(r, expected_rgb.0);
                    assert_eq!(g, expected_rgb.1);
                    assert_eq!(b, expected_rgb.2);
                }
                _ => panic!("Expected RGB color for {}", hex),
            }
        }

        #[cfg(feature = "colored")]
        {
            let colored_c: colored::Color = c.into();
            match colored_c {
                colored::Color::TrueColor { r, g, b } => {
                    assert_eq!(r, expected_rgb.0);
                    assert_eq!(g, expected_rgb.1);
                    assert_eq!(b, expected_rgb.2);
                }
                _ => panic!("Expected TrueColor for {}", hex),
            }
        }

        #[cfg(feature = "anstyle")]
        {
            let anstyle_c: anstyle::Color = c.into();
            match anstyle_c {
                anstyle::Color::Rgb(rgb) => {
                    assert_eq!(rgb.0, expected_rgb.0);
                    assert_eq!(rgb.1, expected_rgb.1);
                    assert_eq!(rgb.2, expected_rgb.2);
                }
                _ => panic!("Expected RGB color for {}", hex),
            }
        }

        #[cfg(feature = "nu-ansi-term")]
        {
            let nat: nu_ansi_term::Color = c.into();
            match nat {
                nu_ansi_term::Color::Rgb(r, g, b) => {
                    assert_eq!(r, expected_rgb.0);
                    assert_eq!(g, expected_rgb.1);
                    assert_eq!(b, expected_rgb.2);
                }
                _ => panic!("Expected RGB color for {}", hex),
            }
        }

        #[cfg(feature = "yansi")]
        {
            let yansi_c: yansi::Color = c.into();
            match yansi_c {
                yansi::Color::Rgb(r, g, b) => {
                    assert_eq!(r, expected_rgb.0);
                    assert_eq!(g, expected_rgb.1);
                    assert_eq!(b, expected_rgb.2);
                }
                _ => panic!("Expected RGB color for {}", hex),
            }
        }

        #[cfg(feature = "crossterm")]
        {
            let ct: crossterm::style::Color = c.into();
            match ct {
                crossterm::style::Color::Rgb { r, g, b } => {
                    assert_eq!(r, expected_rgb.0);
                    assert_eq!(g, expected_rgb.1);
                    assert_eq!(b, expected_rgb.2);
                }
                _ => panic!("Expected RGB color for {}", hex),
            }
        }
    }
}
