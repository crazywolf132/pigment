#[cfg(feature = "rgb")]
use rgb::Rgb;
#[cfg(feature = "rgb")]
use pigment::color;

fn main() {
    #[cfg(feature = "rgb")]
    {
        println!("This example requires the 'rgb' feature to be enabled.");
        println!("Run with: cargo run --example rgb_integration --features rgb\n");

        // Get a color from pigment
        let azure = color("Azure").unwrap();
        
        // Convert to rgb::Rgb<u8>
        let rgb_color: Rgb<u8> = azure.into();
        
        println!("Azure color:");
        println!("  Pigment RGB: {:?}", azure.rgb());
        println!("  rgb crate: {:?}", rgb_color);
        
        // Demonstrate some rgb crate functionality
        
        // Access individual components
        println!("\nComponents:");
        println!("  Red: {}", rgb_color.r);
        println!("  Green: {}", rgb_color.g);
        println!("  Blue: {}", rgb_color.b);
        
        // Convert to array
        let array: [u8; 3] = rgb_color.into();
        println!("\nAs array: {:?}", array);
        
        // Try a few more colors
        println!("\nMore colors:");
        let colors = ["Crimson", "ForestGreen", "DeepSkyBlue", "Gold", "Purple"];
        
        for color_name in colors {
            if let Some(c) = color(color_name) {
                let rgb_color: Rgb<u8> = c.into();
                println!("  {}: {:?}", color_name, rgb_color);
            }
        }
    }

    #[cfg(not(feature = "rgb"))]
    {
        println!("This example requires the 'rgb' feature to be enabled.");
        println!("Run with: cargo run --example rgb_integration --features rgb");
    }
}
