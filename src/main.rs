use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Directionality of conversion
    mode: Mode,

    // Input strings
    strings: Option<Vec<String>>,

    // Input file path
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    // Converts to Hexidecimal from ASCII (technically UTF8)
    ToHex,
    // Converts to ASCII (technically UTF8) from Hexidecimal
    ToASCII,
}

fn main() {
    let cli = Args::parse();

    let mut output = String::new();
    if let Some(strings) = cli.strings {
        output = process_strings(strings);
    } else if let Some(file_path) = cli.file {
        let bytes = File::open(file_path).unwrap().bytes();
        output = bytes_to_string(bytes.into_iter().filter_map(|b| b.ok()));
    }

    println!("{output}");
}

fn process_strings(vec: Vec<String>) -> String {
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
