use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    string: String,
    other_strings: Option<Vec<String>>,
}

fn main() {
    println!("Hello, world!");
}
