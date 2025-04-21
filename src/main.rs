use clap::Parser;

/// Search for a pattern in file and display the lines that contain it.
#[derive(Parser)]
struct Args {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("Pattern: {:?}, Path: {:?}", args.pattern, args.path);
}
