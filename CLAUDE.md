# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

RenderMD is a fast, cross-platform CLI tool written in Rust that renders markdown from stdin or files with rich terminal formatting. It's designed for pipeline usage and transforms markdown into beautifully formatted terminal output with syntax highlighting and proper typography.

## Common Development Commands

### Building and Running
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run the application directly
cargo run -- [OPTIONS] [FILE]

# Run with sample markdown
echo "# Test **bold** and *italic*" | cargo run
```

### Testing
```bash
# Run all tests
cargo test

# Run integration tests specifically
cargo test --test integration_test

# Run a single test
cargo test test_help_output
```

### Project Structure and Architecture

The application follows a simple CLI architecture:

- **Single binary design**: All functionality is contained in `src/main.rs`
- **Async I/O**: Uses tokio for file and stdin operations to handle large markdown files efficiently
- **CLI parsing**: Uses clap derive API for argument parsing with comprehensive help and validation
- **Terminal rendering**: Leverages termimad library for markdown-to-terminal conversion with custom styling

### Key Components

1. **CLI Interface** (`Args` struct): Defines all command-line options including input file, styling options (--no-color, --minimal, --raw), and terminal width control
2. **Input handling**: Async functions for reading from both files (`read_file`) and stdin (`read_stdin`)
3. **Rendering engine** (`render_markdown`): Applies different styling based on CLI flags, handles terminal width detection, and manages color schemes

### Dependencies and Rust Edition

- **Rust Edition**: 2024 (requires Rust 1.85.0+)
- **Key dependencies**:
  - `clap` with derive feature for CLI parsing
  - `termimad` for markdown rendering with `crossterm` re-exports
  - `tokio` with rt-multi-thread, io-util, macros, fs, io-std features
  - `anyhow` for error handling

### Testing Strategy

The project uses integration tests that execute the built binary via `Command::new("cargo")` to test:
- CLI help and version output
- File input processing
- Different output modes (minimal, raw, styled)
- End-to-end functionality with `test_sample.md`

Tests are located in `tests/integration_test.rs` and rely on `test_sample.md` for input validation.