use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    strings: Option<Vec<String>>,

    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
}

fn main() {
    let cli = Args::parse();

    let mut output = String::new();
    if let Some(strings) = cli.strings {
        output = process(strings);
    } else if let Some(file_path) = cli.file {
        let bytes = File::open(file_path).unwrap().bytes();
        output = bytes_to_string(bytes.into_iter().filter_map(|b| b.ok()));
    }

    println!("{output}");
}

fn process(vec: Vec<String>) -> String {
    vec.into_iter()
        .map(|t| bytes_to_string(t.into_bytes().into_iter()))
        .collect::<Vec<String>>()
        .join("\n")
}

fn bytes_to_string<T: Iterator<Item = u8>>(bytes: T) -> String {
    bytes
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}
