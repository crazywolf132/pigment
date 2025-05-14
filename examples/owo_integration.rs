#[cfg(feature = "owo")]
use owo_colors::OwoColorize;
#[cfg(feature = "owo")]
use pigment::color;

fn main() {
    #[cfg(feature = "owo")]
    {
        println!("This example requires the 'owo' feature to be enabled.");
        println!("Run with: cargo run --example owo_integration --features owo\n");

        let azure = color("Azure").unwrap();

        // Convert to owo-colors Rgb
        let owo_color: owo_colors::Rgb = azure.into();

        // Use with owo-colors
        println!("{}", "Azure colored text".color(owo_color));

        // Try a few more colors
        let colors = ["Crimson", "ForestGreen", "DeepSkyBlue", "Gold", "Purple"];

        for color_name in colors {
            if let Some(c) = color(color_name) {
                let owo_color: owo_colors::Rgb = c.into();
                println!("{}", color_name.color(owo_color));
            }
        }
    }

    #[cfg(not(feature = "owo"))]
    {
        println!("This example requires the 'owo' feature to be enabled.");
        println!("Run with: cargo run --example owo_integration --features owo");
    }
}
