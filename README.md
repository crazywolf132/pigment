# Pigment

[![Crates.io](https://img.shields.io/crates/v/pigment.svg)](https://crates.io/crates/pigment)
[![Documentation](https://docs.rs/pigment/badge.svg)](https://docs.rs/pigment)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

All the colors of the web, by name – case-/space-/snake-insensitive.

Pigment is a Rust library that provides access to hundreds of named colors, with a forgiving lookup system that ignores case, spaces, and other non-alphanumeric characters.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pigment = "0.1.2"
```

## Features

- **Extensive color database**: Hundreds of named colors from Wikipedia
- **Forgiving lookups**: Case-insensitive, ignores spaces and special characters
- **Multiple formats**: Access colors as hex codes or RGB tuples
- **ANSI terminal support**: Built-in support for ANSI color codes
- **Multiple library integrations**: Optional integrations with popular color libraries:
  - [owo-colors](https://github.com/jam1garner/owo-colors)
  - [termcolor](https://github.com/BurntSushi/termcolor)
  - [colored](https://github.com/colored-rs/colored)
  - [anstyle](https://github.com/rust-cli/anstyle)
  - [nu-ansi-term](https://github.com/nushell/nu-ansi-term)
  - [yansi](https://github.com/SergioBenitez/yansi)
  - [crossterm](https://github.com/crossterm-rs/crossterm)

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

### Library Integrations

Pigment can integrate with several popular Rust color libraries. Here are some examples:

#### owo-colors

Enable the `owo` feature in your `Cargo.toml`:

```toml
[dependencies]
pigment = { version = "0.1.2", features = ["owo"] }
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

#### termcolor

Enable the `termcolor` feature in your `Cargo.toml`:

```toml
[dependencies]
pigment = { version = "0.1.2", features = ["termcolor"] }
termcolor = "1.2"
```

Then use it like this:

```rust
use std::io::Write;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};
use pigment::color;

fn main() {
    let azure = color("Azure").unwrap();
    let tc_color: termcolor::Color = azure.into();

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(tc_color))).unwrap();
    writeln!(&mut stdout, "Azure colored text").unwrap();
    stdout.reset().unwrap();
}
```

#### colored

Enable the `colored` feature in your `Cargo.toml`:

```toml
[dependencies]
pigment = { version = "0.1.2", features = ["colored"] }
colored = "2"
```

Then use it like this:

```rust
use colored::Colorize;
use pigment::color;

fn main() {
    let azure = color("Azure").unwrap();
    let c_color: colored::Color = azure.into();

    println!("{}", "Azure colored text".color(c_color));
}
```

Other supported libraries include `anstyle`, `nu-ansi-term`, `yansi`, and `crossterm`.
See the examples directory for more detailed usage examples.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
