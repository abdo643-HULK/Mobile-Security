pub mod cryptanalysis;
pub mod playfair;

use clap::{ArgGroup, Parser};
use core::fmt::Debug;
use playfair::Playfair;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("phase")
        .required(true)
        .args(["encode", "decode",]),
))]

struct Args {
    text: String,
    #[arg(short)]
    key: String,
    #[arg(short)]
    encode: bool,
    #[arg(short)]
    decode: bool,
    #[arg(short, default_value_t = false)]
    print: bool,
}

fn main() {
    let Args {
        key,
        text,
        encode,
        decode,
        print,
    } = Args::parse();

    let playfair = Playfair::new(key, 'X');

    let text = text
        .to_ascii_uppercase()
        .split_whitespace()
        .collect::<String>();

    let text = if encode {
        playfair.encode(&text)
    } else if decode {
        playfair.decode(&text)
    } else {
        panic!("-e (encode) or -d (decode) flag must be passed in");
    };

    text.and_then(|text| Ok(println!("{text}")));

    if print {
        println!("Matrix: [");
        playfair.matrix().iter().for_each(|row| {
            println!("  {row:?}");
        });
        println!("]");
    }
}
