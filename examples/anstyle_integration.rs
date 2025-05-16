#[cfg(feature = "anstyle")]
use anstyle::Style;
#[cfg(feature = "anstyle")]
use pigment::color;

fn main() {
    #[cfg(feature = "anstyle")]
    {
        println!("This example requires the 'anstyle' feature to be enabled.");
        println!("Run with: cargo run --example anstyle_integration --features anstyle\n");

        let azure = color("Azure").unwrap();

        // Convert to anstyle Color
        let a_color: anstyle::Color = azure.into();

        // Use with anstyle
        let style = Style::new().fg_color(Some(a_color));
        println!("{}Azure colored text{}", style.render(), style.render_reset());

        // Try a few more colors
        let colors = ["Crimson", "ForestGreen", "DeepSkyBlue", "Gold", "Purple"];

        for color_name in colors {
            if let Some(c) = color(color_name) {
                let a_color: anstyle::Color = c.into();
                let style = Style::new().fg_color(Some(a_color));
                println!("{}{}{}", style.render(), color_name, style.render_reset());
            }
        }
    }

    #[cfg(not(feature = "anstyle"))]
    {
        println!("This example requires the 'anstyle' feature to be enabled.");
        println!("Run with: cargo run --example anstyle_integration --features anstyle");
    }
}
