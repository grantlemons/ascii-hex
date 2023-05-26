use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    strings: Vec<String>,
}

fn main() {
    let cli = Args::parse();

    let text = cli
        .strings
        .into_iter()
        .map(|t| bytes_to_string(&t.into_bytes()))
        .collect::<Vec<String>>()
        .join("\n");

    println!("{text}");
}

fn bytes_to_string(bytes: &[u8]) -> String {
    bytes
        .into_iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}
