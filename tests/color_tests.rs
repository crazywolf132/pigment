use pigment::color;

#[test]
fn test_color_lookup() {
    // Test basic lookup
    let red = color("Red").unwrap();
    assert_eq!(red.name(), "Red");
    assert_eq!(red.hex(), "#FF0000");
    assert_eq!(red.rgb(), (255, 0, 0));
    
    // Test case insensitivity
    assert_eq!(color("Red"), color("red"));
    assert_eq!(color("Red"), color("RED"));
    
    // Test space insensitivity
    assert_eq!(color("AliceBlue"), color("Alice Blue"));
    assert_eq!(color("AliceBlue"), color("Alice  Blue"));
    
    // Test special character insensitivity
    assert_eq!(color("AliceBlue"), color("Alice-Blue"));
    assert_eq!(color("AliceBlue"), color("Alice_Blue"));
    
    // Test non-existent color
    assert_eq!(color("NonExistentColor"), None);
}

#[test]
fn test_ansi_output() {
    let red = color("Red").unwrap();
    
    // Test ANSI foreground
    let fg = red.ansi().fg();
    assert_eq!(fg, "\x1b[38;2;255;0;0m");
    
    // Test ANSI background
    let bg = red.ansi().bg();
    assert_eq!(bg, "\x1b[48;2;255;0;0m");
    
    // Test ANSI reset
    assert_eq!(pigment::ansi::Ansi::reset(), "\x1b[0m");
}

#[cfg(feature = "owo")]
#[test]
fn test_owo_integration() {
    use owo_colors::Rgb;
    
    let red = color("Red").unwrap();
    let owo_rgb: Rgb = red.into();
    
    // Check that the conversion works correctly
    assert_eq!(owo_rgb.0, 255);
    assert_eq!(owo_rgb.1, 0);
    assert_eq!(owo_rgb.2, 0);
}
