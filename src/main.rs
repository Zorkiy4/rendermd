use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, Write};
use std::path::PathBuf;
use termimad::{crossterm::{style::Color, terminal}, *};
use tokio::io::AsyncReadExt;

#[derive(Parser)]
#[command(
    name = "rendermd",
    version = "0.1.0",
    about = "A fast, cross-platform CLI tool that renders markdown from stdin with rich terminal formatting",
    long_about = "RenderMD transforms markdown into beautifully formatted terminal output with syntax highlighting, proper typography, and cross-platform compatibility. Designed for pipeline usage."
)]
struct Args {
    /// Input file to render (reads from stdin if not provided)
    #[arg(value_name = "FILE")]
    input: Option<PathBuf>,

    /// Disable colored output
    #[arg(long, short = 'n')]
    no_color: bool,

    /// Set the maximum width of the output (default: terminal width)
    #[arg(long, short = 'w', value_name = "WIDTH")]
    width: Option<u16>,

    /// Show raw markdown without rendering
    #[arg(long, short = 'r')]
    raw: bool,

    /// Use minimal styling (no colors, basic formatting only)
    #[arg(long, short = 'm')]
    minimal: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let content = if let Some(ref input_file) = args.input {
        read_file(input_file).await
            .with_context(|| format!("Failed to read file: {}", input_file.display()))?
    } else {
        read_stdin().await
            .with_context(|| "Failed to read from stdin")?
    };

    if content.trim().is_empty() {
        return Ok(());
    }

    if args.raw {
        print!("{}", content);
        return Ok(());
    }

    render_markdown(&content, &args)?;

    Ok(())
}

async fn read_file(path: &PathBuf) -> Result<String> {
    tokio::fs::read_to_string(path).await
        .with_context(|| format!("Failed to read file: {}", path.display()))
}

async fn read_stdin() -> Result<String> {
    let mut stdin = tokio::io::stdin();
    let mut content = String::new();
    stdin.read_to_string(&mut content).await
        .context("Failed to read from stdin")?;
    Ok(content)
}

fn render_markdown(content: &str, args: &Args) -> Result<()> {
    let terminal_width = if let Some(width) = args.width {
        width as usize
    } else {
        terminal::size()
            .map(|(w, _)| w as usize)
            .unwrap_or(80)
    };

    if args.no_color || args.minimal {
        let skin = if args.minimal {
            MadSkin::no_style()
        } else {
            MadSkin::default_dark()
        };

        let fmt_text = FmtText::from_text(&skin, content.into(), Some(terminal_width));
        print!("{}", fmt_text);
    } else {
        let mut skin = MadSkin::default_dark();

        skin.set_headers_fg(Color::Rgb { r: 255, g: 165, b: 0 });
        skin.bold.set_fg(Color::Yellow);
        skin.italic.set_fg(Color::Cyan);
        skin.inline_code.set_fg(Color::Green);
        skin.code_block.set_fg(Color::White);

        let fmt_text = FmtText::from_text(&skin, content.into(), Some(terminal_width));
        print!("{}", fmt_text);
    }

    io::stdout().flush()?;
    Ok(())
}
