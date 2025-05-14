# Pigment

[![Crates.io](https://img.shields.io/crates/v/pigment.svg)](https://crates.io/crates/pigment)
[![Documentation](https://docs.rs/pigment/badge.svg)](https://docs.rs/pigment)
[![CI](https://github.com/crazywolf132/pigment/actions/workflows/ci.yml/badge.svg)](https://github.com/crazywolf132/pigment/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

All the colors of the web, by name – case-/space-/snake-insensitive.

Pigment is a Rust library that provides access to hundreds of named colors, with a forgiving lookup system that ignores case, spaces, and other non-alphanumeric characters.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pigment = "0.1.0"
```

## Features

- **Extensive color database**: Hundreds of named colors from Wikipedia
- **Forgiving lookups**: Case-insensitive, ignores spaces and special characters
- **Multiple formats**: Access colors as hex codes or RGB tuples
- **ANSI terminal support**: Built-in support for ANSI color codes
- **Optional owo-colors integration**: Enable the `owo` feature for integration with the popular owo-colors crate

## Usage

### Basic Usage

```rust
use pigment::color;

fn main() {
    // Look up a color by name
    let azure = color("Azure").unwrap();

    // Access color properties
    println!("Name: {}", azure.name());     // "Azure"
    println!("Hex: {}", azure.hex());       // "#007FFF"
    println!("RGB: {:?}", azure.rgb());     // (0, 127, 255)

    // Forgiving lookups - these all return the same color
    assert_eq!(color("Azure"), color("azure"));
    assert_eq!(color("Azure"), color("AZURE"));
    assert_eq!(color("Azure"), color("a z u r e"));
    assert_eq!(color("Azure"), color("a-z-u-r-e"));
}
```

### ANSI Terminal Colors

```rust
use pigment::color;

fn main() {
    let red = color("Red").unwrap();

    // Print colored text in terminal
    println!("{}This is red text{}",
        red.ansi().fg(),           // Foreground color
        pigment::ansi::Ansi::reset() // Reset formatting
    );

    let blue = color("Blue").unwrap();
    println!("{}Text on blue background{}",
        blue.ansi().bg(),          // Background color
        pigment::ansi::Ansi::reset()
    );
}
```

### Integration with owo-colors

Enable the `owo` feature in your `Cargo.toml`:

```toml
[dependencies]
pigment = { version = "0.1.0", features = ["owo"] }
owo-colors = "4"
```

Then use it like this:

```rust
use owo_colors::OwoColorize;
use pigment::color;

fn main() {
    let azure = color("Azure").unwrap();

    // Use with owo-colors
    let owo_color: owo_colors::Rgb = azure.into();

    // Now you can use all owo-colors functionality
    println!("{}", "Azure colored text".color(owo_color));
}
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
