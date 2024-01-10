use clap::Parser;

// Derived from clap::Parser
// To store the command line arguments
// Clap knows the expected fields and their format
// It automatically generates a help message
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
