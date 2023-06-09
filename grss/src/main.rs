#![allow(unused_must_use)]

use anyhow::{anyhow, Context, Result};
use std::{
    fs::File,
    io::{BufReader},
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
    if String::is_empty(&args.pattern) {
        return Err(anyhow!("Search pattern must not be empty."))
    }

    let file = File::open(&args.path)
        .with_context(|| format!("could not read file '{}'", args.path.display()))?;
    let reader = BufReader::new(file);

    grss::find_matches(reader, &args.pattern, &mut std::io::stdout());

    Ok(())
}

// TODO: Consider moving test to function declaration.
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
