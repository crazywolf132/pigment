use pigment::color;

fn main() {
    // List of color names to check
    let color_names = [
        "Red", "Green", "Blue", "Yellow", "Cyan", "Magenta", "Black", "White",
        "Gray", "Grey", "Orange", "Purple", "Brown", "Pink", "Lime", "Teal", "Navy",
        "Olive", "Maroon", "Aqua", "Silver", "DarkGray", "DarkGrey", "LightGray", "LightGrey",
        "SlateGray", "SlateGrey", "DimGray", "DimGrey", "Gray50", "Grey50",
        // Common CSS/HTML colors
        "AliceBlue", "AntiqueWhite", "Aqua", "Aquamarine", "Azure", "Beige", "Bisque",
        "BlanchedAlmond", "BlueViolet", "BurlyWood", "CadetBlue", "Chartreuse", "Chocolate",
        "Coral", "CornflowerBlue", "Cornsilk", "Crimson", "DarkBlue", "DarkCyan", "DarkGoldenRod",
        "DarkGreen", "DarkKhaki", "DarkMagenta", "DarkOliveGreen", "DarkOrange", "DarkOrchid",
        "DarkRed", "DarkSalmon", "DarkSeaGreen", "DarkSlateBlue", "DarkSlateGray", "DarkSlateGrey",
        "DarkTurquoise", "DarkViolet", "DeepPink", "DeepSkyBlue", "DimGray", "DimGrey", "DodgerBlue",
        "FireBrick", "FloralWhite", "ForestGreen", "Fuchsia", "Gainsboro", "GhostWhite", "Gold",
        "GoldenRod", "Gray", "Grey", "GreenYellow", "HoneyDew", "HotPink", "IndianRed", "Indigo",
        "Ivory", "Khaki", "Lavender", "LavenderBlush", "LawnGreen", "LemonChiffon", "LightBlue",
        "LightCoral", "LightCyan", "LightGoldenRodYellow", "LightGray", "LightGrey", "LightGreen",
        "LightPink", "LightSalmon", "LightSeaGreen", "LightSkyBlue", "LightSlateGray", "LightSlateGrey",
        "LightSteelBlue", "LightYellow", "LimeGreen", "Linen", "MediumAquaMarine", "MediumBlue",
        "MediumOrchid", "MediumPurple", "MediumSeaGreen", "MediumSlateBlue", "MediumSpringGreen",
        "MediumTurquoise", "MediumVioletRed", "MidnightBlue", "MintCream", "MistyRose", "Moccasin",
        "NavajoWhite", "OldLace", "OliveDrab", "OrangeRed", "Orchid", "PaleGoldenRod", "PaleGreen",
        "PaleTurquoise", "PaleVioletRed", "PapayaWhip", "PeachPuff", "Peru", "Plum", "PowderBlue",
        "RosyBrown", "RoyalBlue", "SaddleBrown", "Salmon", "SandyBrown", "SeaGreen", "SeaShell",
        "Sienna", "SkyBlue", "SlateBlue", "SlateGray", "SlateGrey", "Snow", "SpringGreen", "SteelBlue",
        "Tan", "Thistle", "Tomato", "Turquoise", "Violet", "Wheat", "WhiteSmoke", "YellowGreen",
    ];

    println!("Checking for specific color names:");
    for name in color_names {
        match color(name) {
            Some(c) => println!("✅ '{}' exists: {} - {:?}", name, c.hex(), c.rgb()),
            None => println!("❌ '{}' not found", name),
        }
    }

    // Check for gray/grey variants
    println!("\nChecking for gray/grey variants:");
    for prefix in ["", "Dark", "Light", "Slate", "Dim"] {
        for suffix in ["Gray", "Grey"] {
            let name = format!("{}{}", prefix, suffix);
            match color(&name) {
                Some(c) => println!("✅ '{}' exists: {} - {:?}", name, c.hex(), c.rgb()),
                None => println!("❌ '{}' not found", name),
            }
        }
    }
}
