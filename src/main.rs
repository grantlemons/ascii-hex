use std::io::{stdin, Read};
use std::path::PathBuf;
use std::{fs::File, str::FromStr};

use anyhow::Result;
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

    // Spaces in output
    #[arg(short, long)]
    compact: bool,

    // Lowercase output
    #[arg(short, long)]
    lower: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    // Converts to Hexidecimal from ASCII (technically UTF8)
    ToHex,
    // Converts to ASCII (technically UTF8) from Hexidecimal
    ToASCII,
}

const STDIN_PATH: &str = "-";

fn main() -> Result<()> {
    let cli = Args::parse();
    let output;

    match cli.mode {
        Mode::ToHex => {
            if let Some(strings) = cli.strings {
                output = strings_to_hex(strings, !cli.compact, cli.lower);
            } else {
                let mut contents = Vec::<u8>::new();

                if let Some(path) = cli.file {
                    if path == PathBuf::from(STDIN_PATH) {
                        stdin().lock().read_to_end(&mut contents)?;
                    } else {
                        File::open(path)?.read_to_end(&mut contents)?;
                    }
                } else {
                    stdin().lock().read_to_end(&mut contents)?;
                    contents.pop();
                }

                output = bytes_to_string(contents, !cli.compact, cli.lower);
            }
        }

        Mode::ToASCII => {
            let lines: Vec<String>;

            if let Some(strings) = cli.strings {
                lines = strings;
            } else {
                let mut contents = String::new();

                if let Some(path) = cli.file {
                    if path == PathBuf::from(STDIN_PATH) {
                        stdin().lock().read_to_string(&mut contents)?;
                    } else {
                        File::open(path)?.read_to_string(&mut contents)?;
                    }
                } else {
                    stdin().lock().read_to_string(&mut contents)?;
                }

                lines = contents
                    .split('\n')
                    .filter(|l| !l.is_empty())
                    .map(|s| String::from_str(s).unwrap())
                    .collect();
            }
            output = hex_strings_to_ascii(lines);
        }
    }
    println!("{output}");

    Ok(())
}

fn hex_strings_to_ascii<T: IntoIterator<Item = String>>(lines: T) -> String {
    lines
        .into_iter()
        .map(|l| {
            String::from_iter(
                extract_hex_pairs(l)
                    .iter()
                    .map(|c| u8::from_str_radix(c, 16).unwrap() as char),
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn extract_hex_pairs(input: String) -> Vec<String> {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(String::from_iter)
        .collect()
}

fn strings_to_hex(vec: Vec<String>, spaces: bool, lower: bool) -> String {
    vec.into_iter()
        .map(|t| bytes_to_string(t.into_bytes().into_iter(), spaces, lower))
        .collect::<Vec<_>>()
        .join("\n")
}

fn bytes_to_string<T: IntoIterator<Item = u8>>(bytes: T, spaces: bool, lower: bool) -> String {
    let seperator = if spaces { " " } else { "" };
    let format_fn = |b| {
        if lower {
            format!("{:02x}", b)
        } else {
            format!("{:02X}", b)
        }
    };

    bytes
        .into_iter()
        .map(format_fn)
        .collect::<Vec<_>>()
        .join(seperator)
}
