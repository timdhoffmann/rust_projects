use anyhow::{Context, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

/// Search for a string in a file and display all matching lines.
#[derive(Parser)]
struct Cli {
    /// The pattern to search for.
    pattern: String,
    /// The path to the file in which to search.
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    println!("{:?}, {:?}", args.pattern, args.path);

    let file = File::open(&args.path)
        .with_context(|| format!("Could not read file '{}'", args.path.display()))?;
    let reader = BufReader::new(file);

    grss::find_matches(reader, &args.pattern, &mut std::io::stdout());

    Ok(())
}

#[test]
fn test_find_matches() {
    let content = "Line 1\nLine 2\nLine 3\n";
    let pattern = "2";

    let reader = BufReader::new(content.as_bytes());
    let mut writer = Vec::new();

    grss::find_matches(reader, pattern, &mut writer);

    let result = String::from_utf8(writer).unwrap();
    assert_eq!("Line 2\n", result);
}
