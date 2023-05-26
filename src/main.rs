use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    strings: Vec<String>,
}

fn main() {
    let cli = Args::parse();

    let text = cli.strings.join("\n");
    let bytes = text.into_bytes();

    let output = bytes_to_string(&bytes);
    println!("{output}");
}

fn bytes_to_string(bytes: &[u8]) -> String {
    bytes
        .into_iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}
