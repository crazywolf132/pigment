#[cfg(feature = "colored")]
use colored::Colorize;
#[cfg(feature = "colored")]
use pigment::color;

fn main() {
    #[cfg(feature = "colored")]
    {
        println!("This example requires the 'colored' feature to be enabled.");
        println!("Run with: cargo run --example colored_integration --features colored\n");

        let azure = color("Azure").unwrap();

        // Convert to colored Color
        let c_color: colored::Color = azure.into();

        // Use with colored
        println!("{}", "Azure colored text".color(c_color));

        // Try a few more colors
        let colors = ["Crimson", "ForestGreen", "DeepSkyBlue", "Gold", "Purple"];

        for color_name in colors {
            if let Some(c) = color(color_name) {
                let c_color: colored::Color = c.into();
                println!("{}", color_name.color(c_color));
            }
        }
    }
    
    #[cfg(not(feature = "colored"))]
    {
        println!("This example requires the 'colored' feature to be enabled.");
        println!("Run with: cargo run --example colored_integration --features colored");
    }
}
