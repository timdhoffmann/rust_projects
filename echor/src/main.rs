use clap::Parser;

#[derive(Parser, Debug)]
// Read values from `Cargo.toml`
#[command(author, version, about, long_about = None)]
struct Args {
    text: String,

    /// Don't add a newline character, if provided.
    #[arg(short = 'n', long)]
    omit_newline: bool
}
fn main() {
    let args = Args::parse();
    println!("{:#?}", args);

    let output = if args.omit_newline {
        args.text
    } else {
        args.text + "\n"
    };
    print!("{}", output)
}
