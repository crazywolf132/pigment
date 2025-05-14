use pigment::color;

fn main() {
    // Look up a color by name
    let azure = color("Azure").unwrap();

    // Access color properties
    println!("Name: {}", azure.name()); // "Azure"
    println!("Hex: {}", azure.hex()); // "#007FFF"
    println!("RGB: {:?}", azure.rgb()); // (0, 127, 255)

    // Forgiving lookups - these all return the same color
    assert_eq!(color("Azure"), color("azure"));
    assert_eq!(color("Azure"), color("AZURE"));
    assert_eq!(color("Azure"), color("a z u r e"));
    assert_eq!(color("Azure"), color("a-z-u-r-e"));
}
