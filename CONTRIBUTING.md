# Contributing to Pigment

Thank you for considering contributing to Pigment! This document provides guidelines and instructions for contributing.

## Development Setup

1. Clone the repository:
   ```
   git clone https://github.com/crazywolf132/pigment.git
   cd pigment
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run tests:
   ```
   cargo test
   ```

4. Run examples:
   ```
   cargo run --example basic
   cargo run --example ansi
   cargo run --example owo_integration --features owo
   ```

## Regenerating Color Data

The color data is stored in `generated/colors.rs`, which is generated from Wikipedia color lists. To regenerate this file:

1. Set the `PIGMENT_REGEN` environment variable:
   ```
   export PIGMENT_REGEN=1  # Unix/macOS
   set PIGMENT_REGEN=1     # Windows
   ```

2. Build the project:
   ```
   cargo build
   ```

This will:
- Create a Python virtual environment
- Install required Python packages (requests, beautifulsoup4, tqdm)
- Run the scraper script to fetch color data from Wikipedia
- Generate the `generated/colors.rs` file

Alternatively, you can use the provided build script:
```
./build.sh
```

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests to ensure everything works
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Release Process

The project uses GitHub Actions for continuous integration and deployment:

1. **CI Workflow**: Runs tests, formatting checks, and linting on all pull requests and pushes to the main branch.

2. **Release Workflow**: When you create a new tag with the format `v*` (e.g., `v0.1.1`), it will:
   - Update the version in Cargo.toml
   - Create a GitHub release
   - Trigger the publish workflow

3. **Publish Workflow**: When a new GitHub release is created, it will:
   - Run tests
   - Publish the crate to crates.io

To create a new release:

```bash
# Ensure you're on the main branch
git checkout main
git pull

# Create and push a new tag
git tag v0.1.1
git push origin v0.1.1
```

The GitHub Actions workflows will handle the rest automatically.

## Code Style

- Follow the Rust style guidelines
- Use `cargo fmt` to format your code
- Run `cargo clippy` to check for common issues

## License

By contributing to Pigment, you agree that your contributions will be licensed under the project's MIT License.
