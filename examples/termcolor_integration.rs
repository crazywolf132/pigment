#[cfg(feature = "termcolor")]
use std::io::Write;
#[cfg(feature = "termcolor")]
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};
#[cfg(feature = "termcolor")]
use pigment::color;

fn main() {
    #[cfg(feature = "termcolor")]
    {
        println!("This example requires the 'termcolor' feature to be enabled.");
        println!("Run with: cargo run --example termcolor_integration --features termcolor\n");

        let azure = color("Azure").unwrap();
        
        // Convert to termcolor Color
        let tc_color: termcolor::Color = azure.into();
        
        // Use with termcolor
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(tc_color))).unwrap();
        writeln!(&mut stdout, "Azure colored text").unwrap();
        stdout.reset().unwrap();
        
        // Try a few more colors
        let colors = ["Crimson", "ForestGreen", "DeepSkyBlue", "Gold", "Purple"];
        
        for color_name in colors {
            if let Some(c) = color(color_name) {
                let tc_color: termcolor::Color = c.into();
                stdout.set_color(ColorSpec::new().set_fg(Some(tc_color))).unwrap();
                writeln!(&mut stdout, "{}", color_name).unwrap();
                stdout.reset().unwrap();
            }
        }
    }
    
    #[cfg(not(feature = "termcolor"))]
    {
        println!("This example requires the 'termcolor' feature to be enabled.");
        println!("Run with: cargo run --example termcolor_integration --features termcolor");
    }
}
