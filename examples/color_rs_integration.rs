#[cfg(feature = "color-rs")]
use color::Rgba8;
#[cfg(feature = "color-rs")]
use pigment::color;

fn main() {
    #[cfg(feature = "color-rs")]
    {
        println!("This example requires the 'color-rs' feature to be enabled.");
        println!("Run with: cargo run --example color_rs_integration --features color-rs\n");

        // Get a color from pigment
        let azure = color("Azure").unwrap();
        
        // Convert to color::Rgba8
        let color_rs: Rgba8 = azure.into();
        
        println!("Azure color:");
        println!("  Pigment RGB: {:?}", azure.rgb());
        println!("  color-rs: rgba({}, {}, {}, {})", color_rs.r, color_rs.g, color_rs.b, color_rs.a);
        
        // Demonstrate some color-rs functionality
        
        // Access individual components
        println!("\nComponents:");
        println!("  Red: {}", color_rs.r);
        println!("  Green: {}", color_rs.g);
        println!("  Blue: {}", color_rs.b);
        println!("  Alpha: {}", color_rs.a);
        
        // Try a few more colors
        println!("\nMore colors:");
        let colors = ["Crimson", "ForestGreen", "DeepSkyBlue", "Gold", "Purple"];
        
        for color_name in colors {
            if let Some(c) = color(color_name) {
                let color_rs: Rgba8 = c.into();
                println!("  {}: rgba({}, {}, {}, {})", 
                    color_name, 
                    color_rs.r, 
                    color_rs.g, 
                    color_rs.b, 
                    color_rs.a
                );
            }
        }
    }

    #[cfg(not(feature = "color-rs"))]
    {
        println!("This example requires the 'color-rs' feature to be enabled.");
        println!("Run with: cargo run --example color_rs_integration --features color-rs");
    }
}
