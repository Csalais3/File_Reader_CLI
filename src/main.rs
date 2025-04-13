// Basic Command Line tool using Rust to better understand how they work

use clap::Parser; // Rust library for parsing arguments within the run command
use anyhow::{Context, Result}; // Rust library for shorthanding error reporting

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

//Main method for printing to the terminal
fn main() -> Result<()> {
    let args = Cli::parse(); // Parses the arguments from the input prompt
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read the file. '{}'", args.path.display()))?; // takes in the file arguent, reads it or prints out an error
    
    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}