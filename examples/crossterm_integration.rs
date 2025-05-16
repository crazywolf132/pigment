#[cfg(feature = "crossterm")]
use std::io::stdout;
#[cfg(feature = "crossterm")]
use crossterm::{
    execute,
    style::{Print, ResetColor, SetForegroundColor},
};
#[cfg(feature = "crossterm")]
use pigment::color;

fn main() -> std::io::Result<()> {
    #[cfg(feature = "crossterm")]
    {
        println!("This example requires the 'crossterm' feature to be enabled.");
        println!("Run with: cargo run --example crossterm_integration --features crossterm\n");

        let azure = color("Azure").unwrap();

        // Convert to crossterm Color
        let ct_color: crossterm::style::Color = azure.into();

        // Use with crossterm
        execute!(
            stdout(),
            SetForegroundColor(ct_color),
            Print("Azure colored text\n"),
            ResetColor
        )?;

        // Try a few more colors
        let colors = ["Crimson", "ForestGreen", "DeepSkyBlue", "Gold", "Purple"];

        for color_name in colors {
            if let Some(c) = color(color_name) {
                let ct_color: crossterm::style::Color = c.into();
                execute!(
                    stdout(),
                    SetForegroundColor(ct_color),
                    Print(format!("{}\n", color_name)),
                    ResetColor
                )?;
            }
        }
    }

    #[cfg(not(feature = "crossterm"))]
    {
        println!("This example requires the 'crossterm' feature to be enabled.");
        println!("Run with: cargo run --example crossterm_integration --features crossterm");
    }

    Ok(())
}
