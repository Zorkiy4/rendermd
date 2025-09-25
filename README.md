# RenderMD - Terminal Markdown Renderer

A fast, cross-platform CLI tool that renders markdown from stdin or files with rich terminal formatting. Designed for pipeline usage, it transforms markdown into beautifully formatted terminal output with syntax highlighting, proper typography, and cross-platform compatibility.

## Features

- **Fast rendering** - Built in Rust for optimal performance
- **Cross-platform** - Works on Windows, macOS, and Linux
- **Pipeline friendly** - Reads from stdin by default
- **Rich formatting** - Syntax highlighting, colors, and proper typography
- **Configurable output** - Multiple styling options and width control
- **Minimal dependencies** - Self-contained binary with carefully chosen dependencies

## Installation

Build from source:

```bash
git clone https://github.com/Zorkiy4/rendermd
cd rendermd
cargo build --release
```

The binary will be available at `target/release/rendermd` (or `rendermd.exe` on Windows).

## Usage

### Basic Usage

Render markdown from stdin:
```bash
echo "# Hello **World**" | rendermd
```

Render markdown from file:
```bash
rendermd README.md
```

### Pipeline Usage

RenderMD is designed for use in command pipelines:

```bash
# View a markdown file with syntax highlighting
cat README.md | rendermd

# Preview markdown files in a directory
find . -name "*.md" -exec cat {} \; | rendermd

# Combine with other tools
curl -s https://raw.githubusercontent.com/user/repo/main/README.md | rendermd
```

### Command Line Options

```
Usage: rendermd [OPTIONS] [FILE]

Arguments:
  [FILE]  Input file to render (reads from stdin if not provided)

Options:
  -n, --no-color         Disable colored output
  -w, --width <WIDTH>    Set the maximum width of the output (default: terminal width)
  -r, --raw             Show raw markdown without rendering
  -m, --minimal         Use minimal styling (no colors, basic formatting only)
  -h, --help            Print help
  -V, --version         Print version
```

### Examples

Render with custom width:
```bash
rendermd --width 60 README.md
```

Render without colors (good for redirection):
```bash
rendermd --no-color README.md > output.txt
```

Show raw markdown (useful for debugging):
```bash
rendermd --raw README.md
```

Minimal styling (compatible with more terminals):
```bash
rendermd --minimal README.md
```

## Supported Markdown Features

- **Headers** (H1-H6) with colored formatting
- **Bold** and *italic* text
- `Inline code` with background highlighting
- Code blocks with syntax detection
- > Blockquotes with visual indicators
- Lists (unordered and ordered)
- Tables with proper alignment
- Links (displayed with formatting)

## Building

Requirements:
- Rust 2024 edition (1.85.0+)
- Cargo

Build instructions:
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test
```

## Testing

Run the test suite:
```bash
cargo test
```

Test with sample markdown:
```bash
echo "# Test **bold** and *italic*" | cargo run
```

## Dependencies

- `clap` - Command line argument parsing
- `termimad` - Terminal markdown rendering
- `crossterm` - Cross-platform terminal handling
- `anyhow` - Error handling
- `tokio` - Async I/O for file operations

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Run `cargo test` to ensure tests pass
6. Submit a pull request

## Acknowledgments

- Built with [termimad](https://github.com/Canop/termimad) for terminal markdown rendering
- Uses [clap](https://github.com/clap-rs/clap) for CLI parsing
- Cross-platform compatibility via [crossterm](https://github.com/crossterm-rs/crossterm)