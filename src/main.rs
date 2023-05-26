use std::io::Read;
use std::path::PathBuf;
use std::{fs::File, str::FromStr};

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Directionality of conversion
    #[arg(value_enum)]
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

    match cli.mode {
        Mode::ToHex => {
            if let Some(strings) = cli.strings {
                output = process_to_hex(strings);
            } else if let Some(file_path) = cli.file {
                let bytes = File::open(file_path).unwrap().bytes();
                output = bytes_to_string(bytes.into_iter().filter_map(|b| b.ok()));
            }
        }
        Mode::ToASCII => {
            if let Some(strings) = cli.strings {
                output = process_to_ascii(strings.into_iter());
            } else if let Some(file_path) = cli.file {
                let mut contents = String::new();
                File::open(file_path)
                    .unwrap()
                    .read_to_string(&mut contents)
                    .unwrap();
                let ascii_lines: Vec<String> = contents
                    .split('\n')
                    .filter(|l| !l.is_empty())
                    .map(|s| String::from_str(s).unwrap())
                    .collect();
                println!("{:?}", ascii_lines);
                output = process_to_ascii(ascii_lines.into_iter());
            }
        }
    }

    println!("{output}");
}

fn process_to_ascii<T: Iterator<Item = String>>(lines: T) -> String {
    lines
        .map(|l| {
            String::from_iter(
                l.split(' ')
                    .filter(|c| !c.is_empty())
                    .map(|c| u8::from_str_radix(c, 16).unwrap() as char),
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn process_to_hex(vec: Vec<String>) -> String {
    vec.into_iter()
        .map(|t| bytes_to_string(t.into_bytes().into_iter()))
        .collect::<Vec<_>>()
        .join("\n")
}

fn bytes_to_string<T: Iterator<Item = u8>>(bytes: T) -> String {
    bytes
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}
