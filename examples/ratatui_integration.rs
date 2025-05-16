#[cfg(feature = "ratatui")]
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
#[cfg(feature = "ratatui")]
use pigment::color;

fn main() {
    #[cfg(feature = "ratatui")]
    {
        println!("This example requires the 'ratatui' feature to be enabled.");
        println!("Run with: cargo run --example ratatui_integration --features ratatui\n");

        // This is just a demonstration of how to convert pigment colors to ratatui colors
        // In a real application, you would use these colors with a Terminal and App
        let colors = [
            "Red", "Orange", "Yellow", "Green", "Blue", "Indigo", "Violet",
            "Crimson", "ForestGreen", "DeepSkyBlue", "Gold", "Purple",
        ];

        println!("Pigment colors converted to ratatui::style::Color:");
        println!("--------------------------------------------------");

        for color_name in colors {
            if let Some(c) = color(color_name) {
                // Convert to ratatui Color
                let rt_color: ratatui::style::Color = c.into();
                
                // Create a style with the color
                let style = Style::default().fg(rt_color);
                
                // Print the color name with its style information
                println!(
                    "{}: {:?} -> ratatui::style::Color::{:?}",
                    color_name,
                    c.rgb(),
                    rt_color
                );
            }
        }

        println!("\nIn a real ratatui application, you would use these colors like this:");
        println!("------------------------------------------------------------------");
        println!("let azure = color(\"Azure\").unwrap();");
        println!("let rt_color: ratatui::style::Color = azure.into();");
        println!("let paragraph = Paragraph::new(\"Text styled with Azure color\")");
        println!("    .style(Style::default().fg(rt_color));");
        println!("f.render_widget(paragraph, area);");
    }

    #[cfg(not(feature = "ratatui"))]
    {
        println!("This example requires the 'ratatui' feature to be enabled.");
        println!("Run with: cargo run --example ratatui_integration --features ratatui");
    }
}
