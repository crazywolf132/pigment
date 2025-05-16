#[cfg(feature = "yansi")]
use yansi::Paint;
#[cfg(feature = "yansi")]
use pigment::color;

fn main() {
    #[cfg(feature = "yansi")]
    {
        println!("This example requires the 'yansi' feature to be enabled.");
        println!("Run with: cargo run --example yansi_integration --features yansi\n");

        let azure = color("Azure").unwrap();

        // Convert to yansi Color
        let y_color: yansi::Color = azure.into();

        // Use with yansi
        println!("{}", Paint::new("Azure colored text").fg(y_color));

        // Try a few more colors
        let colors = ["Crimson", "ForestGreen", "DeepSkyBlue", "Gold", "Purple"];

        for color_name in colors {
            if let Some(c) = color(color_name) {
                let y_color: yansi::Color = c.into();
                println!("{}", Paint::new(color_name).fg(y_color));
            }
        }
    }
    
    #[cfg(not(feature = "yansi"))]
    {
        println!("This example requires the 'yansi' feature to be enabled.");
        println!("Run with: cargo run --example yansi_integration --features yansi");
    }
}
