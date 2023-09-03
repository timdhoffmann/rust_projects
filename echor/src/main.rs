use clap::Parser;

#[derive(Parser, Debug)]
// Read values from `Cargo.toml`
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    text: Vec<String>,

    /// Don't append a newline character.
    #[arg(short = 'n', long)]
    omit_newline: bool
}

fn main() {
    let args = Args::parse();

    let ending = if args.omit_newline {""} else {"\n"};
    let output = args.text.join(" ");
    print!("{}{}", output,ending)
}
