use pigment::color;

fn main() {
    // Print a rainbow of colors
    let colors = [
        "Red", "Orange", "Yellow", "Green", "Blue", "Indigo", "Violet",
    ];

    println!("ANSI Color Demo:");

    for color_name in colors {
        if let Some(c) = color(color_name) {
            // Print colored text
            println!(
                "{}■ {}{}",
                c.ansi().fg(),
                color_name,
                pigment::ansi::Ansi::reset()
            );
        }
    }

    // Background colors
    println!("\nBackground colors:");
    for color_name in colors {
        if let Some(c) = color(color_name) {
            // Print text with colored background
            println!(
                "{}  {} {}",
                c.ansi().bg(),
                color_name,
                pigment::ansi::Ansi::reset()
            );
        }
    }
}
