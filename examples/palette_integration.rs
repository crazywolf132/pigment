#[cfg(feature = "palette")]
use palette::Srgb;
#[cfg(feature = "palette")]
use pigment::color;

fn main() {
    #[cfg(feature = "palette")]
    {
        println!("This example requires the 'palette' feature to be enabled.");
        println!("Run with: cargo run --example palette_integration --features palette\n");

        // Get a color from pigment
        let azure = color("Azure").unwrap();

        // Convert to palette's Srgb<u8>
        let palette_color: Srgb<u8> = azure.into();

        println!("Azure color:");
        println!("  Pigment RGB: {:?}", azure.rgb());
        println!("  Palette RGB: ({}, {}, {})", palette_color.red, palette_color.green, palette_color.blue);

        // Demonstrate some palette functionality

        // Access components
        println!("\nComponents:");
        println!("  Red: {}", palette_color.red);
        println!("  Green: {}", palette_color.green);
        println!("  Blue: {}", palette_color.blue);

        // Convert to array
        let pixel_data = [palette_color.red, palette_color.green, palette_color.blue];
        println!("\nAs array: {:?}", pixel_data);

        // Try a few more colors
        println!("\nMore colors:");
        let colors = ["Crimson", "ForestGreen", "DeepSkyBlue", "Gold", "Purple"];

        for color_name in colors {
            if let Some(c) = color(color_name) {
                let palette_color: Srgb<u8> = c.into();
                println!("  {}: ({}, {}, {})",
                    color_name,
                    palette_color.red,
                    palette_color.green,
                    palette_color.blue
                );
            }
        }
    }

    #[cfg(not(feature = "palette"))]
    {
        println!("This example requires the 'palette' feature to be enabled.");
        println!("Run with: cargo run --example palette_integration --features palette");
    }
}
