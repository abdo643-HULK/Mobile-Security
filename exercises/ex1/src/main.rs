use clap::{ArgGroup, Parser};
use core::fmt::Debug;

pub mod cryptanalysis;
pub mod playfair;

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
    let Args { key, text, .. } = Args::parse();

    let playfair = playfair::Playfair::new(key, text);

    println!("{:?}", playfair.matrix());
}
