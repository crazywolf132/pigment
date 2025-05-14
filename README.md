# Pigment

[![Crates.io](https://img.shields.io/crates/v/pigment.svg)](https://crates.io/crates/pigment)
[![Documentation](https://docs.rs/pigment/badge.svg)](https://docs.rs/pigment)
[![CI](https://github.com/crazywolf132/pigment/actions/workflows/ci.yml/badge.svg)](https://github.com/crazywolf132/pigment/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple Rust library for working with named colors. Nothing fancy, just colors.

## What is this?

Pigment gives you access to hundreds of named colors from the web. It's designed to be straightforward and easy to use.

I built this because I was tired of having to look up RGB values for colors when I just wanted to use "AliceBlue" or "Crimson" in my terminal apps.

The name lookups are forgiving - they ignore case, spaces, and special characters. So "Alice Blue", "alice_blue", and "ALICEBLUE" all work the same.

## Current State

This is a young project. It works, but I'm still polishing things. The core functionality is solid:

- Lookup colors by name
- Get hex and RGB values
- Use ANSI color codes in terminals
- Optional integration with [owo-colors](https://crates.io/crates/owo-colors)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pigment = "0.1.0"
```

If you want owo-colors integration:

```toml
[dependencies]
pigment = { version = "0.1.0", features = ["owo"] }
```

## Quick Examples

### Basic Usage

```rust
use pigment::color;

fn main() {
    // Look up a color
    let azure = color("Azure").unwrap();

    println!("Name: {}", azure.name());     // "Azure"
    println!("Hex: {}", azure.hex());       // "#007FFF"
    println!("RGB: {:?}", azure.rgb());     // (0, 127, 255)
}
```

### Terminal Colors

```rust
use pigment::color;

fn main() {
    let red = color("Red").unwrap();

    // Print colored text
    println!("{}This is red text{}",
        red.ansi().fg(),
        pigment::ansi::Ansi::reset()
    );
}
```

## Where do the colors come from?

The colors are scraped from Wikipedia's color lists. The build script handles this automatically when the `PIGMENT_REGEN` environment variable is set.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Check out the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines.
