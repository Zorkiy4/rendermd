use std::process::Command;
use std::str;

#[test]
fn test_help_output() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("RenderMD transforms markdown"));
    assert!(stdout.contains("--no-color"));
    assert!(stdout.contains("--width"));
    assert!(stdout.contains("--raw"));
    assert!(stdout.contains("--minimal"));
}

#[test]
fn test_version_output() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("rendermd 0.1.0"));
}

#[test]
fn test_file_input() {
    let output = Command::new("cargo")
        .args(&["run", "--", "test_sample.md"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    // Check for some expected markdown rendering output
    assert!(stdout.contains("Hello RenderMD"));
    assert!(stdout.contains("Features"));
}

#[test]
fn test_minimal_output() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--minimal", "test_sample.md"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    // Minimal output should not contain ANSI escape codes
    assert!(!stdout.contains("[38;5;"));
    assert!(stdout.contains("Hello RenderMD"));
}

#[test]
fn test_raw_output() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--raw", "test_sample.md"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    // Raw output should contain original markdown syntax
    assert!(stdout.contains("# Hello RenderMD"));
    assert!(stdout.contains("**bold**"));
    assert!(stdout.contains("`inline code`"));
}

#[test]
fn test_invalid_width() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--width", "5", "test_sample.md"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("Width must be at least 20"));
}

#[test]
fn test_width_too_large() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--width", "300", "test_sample.md"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("Width cannot exceed 200"));
}

#[test]
fn test_nonexistent_file() {
    let output = Command::new("cargo")
        .args(&["run", "--", "nonexistent_file.md"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("Cannot access file") || stderr.contains("Failed to read file"));
}

#[test]
fn test_no_color_flag() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--no-color", "test_sample.md"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    // No-color output should not contain ANSI escape codes
    assert!(!stdout.contains("[38;5;"));
    assert!(!stdout.contains("\x1b["));
    assert!(stdout.contains("Hello RenderMD"));
}