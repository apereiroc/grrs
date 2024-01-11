use anyhow::{Context, Result};
use clap::Parser;

// Derived from clap::Parser
// To store the command line arguments
// Clap knows the expected fields and their format
// It automatically generates a help message
// TODO: add short and long names for the inputs
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

// Function to find the matches
fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    // Iterate over the lines
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).unwrap();
        }
    }
}

// Test the function
#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("Lorem ipsum\ndolor sit amet", "ipsum", &mut result);
    assert_eq!(result, b"Lorem ipsum\n");
}

fn main() -> Result<()> {
    // Parse args
    let args = Cli::parse();

    // Read the file
    // TODO: try the BufReader approach

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file: `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}
