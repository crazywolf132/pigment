#[cfg(feature = "nu-ansi-term")]
use nu_ansi_term::Style;
#[cfg(feature = "nu-ansi-term")]
use pigment::color;

fn main() {
    #[cfg(feature = "nu-ansi-term")]
    {
        println!("This example requires the 'nu-ansi-term' feature to be enabled.");
        println!("Run with: cargo run --example nu_ansi_term_integration --features nu-ansi-term\n");

        let azure = color("Azure").unwrap();

        // Convert to nu_ansi_term Color
        let nat_color: nu_ansi_term::Color = azure.into();

        // Use with nu_ansi_term
        println!("{}", Style::new().fg(nat_color).paint("Azure colored text"));

        // Try a few more colors
        let colors = ["Crimson", "ForestGreen", "DeepSkyBlue", "Gold", "Purple"];

        for color_name in colors {
            if let Some(c) = color(color_name) {
                let nat_color: nu_ansi_term::Color = c.into();
                println!("{}", Style::new().fg(nat_color).paint(color_name));
            }
        }
    }
    
    #[cfg(not(feature = "nu-ansi-term"))]
    {
        println!("This example requires the 'nu-ansi-term' feature to be enabled.");
        println!("Run with: cargo run --example nu_ansi_term_integration --features nu-ansi-term");
    }
}
