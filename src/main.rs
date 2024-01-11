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

fn main() -> Result<()> {
    // Parse args
    let args = Cli::parse();

    // Read the file
    // TODO: try the BufReader approach

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file: `{}`", args.path.display()))?;

    // Iterate over the lines
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
