#![allow(dead_code)]

pub mod cryptanalysis;
pub mod enigma;
pub mod geocaching;
pub mod one_time_pad;
pub mod playfair;
pub mod utils;

use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, Subcommand)]
enum Commands {
    Enigma(enigma::app::Args),
    Playfair(playfair::app::Args),
    Geocaching,
    Cryptanalysis,
    OneTimePad {
        #[arg(value_enum)]
        mode: Option<one_time_pad::Mode>,
    },
}

fn main() {
    let args = std::env::args().map(|arg| match arg.starts_with('-') && arg.len() > 2 {
        true => arg.replace('-', "--"),
        false => arg,
    });

    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Enigma(args) => enigma::app::run(args),
        Commands::Playfair(args) => playfair::app::run(args),
        Commands::Geocaching => geocaching::run(),
        Commands::Cryptanalysis => cryptanalysis::run(),
        Commands::OneTimePad { mode } => {
            one_time_pad::run(mode.unwrap_or(one_time_pad::Mode::Subtraction))
        }
    }

    // playfair::app::run();
}
