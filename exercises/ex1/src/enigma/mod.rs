mod plugboard;
mod reflector;
mod rotor;

use self::{plugboard::Plugboard, reflector::Reflector, rotor::Rotor};

pub trait EnigmaIndex {
    fn index(&self) -> usize;
}

pub trait ToChar {
    fn to_char(&self) -> char;
}

impl EnigmaIndex for char {
    fn index(&self) -> usize {
        debug_assert!(self.is_ascii_uppercase());
        *self as usize - 65
    }
}

impl ToChar for u8 {
    fn to_char(&self) -> char {
        (*self + 65) as char
    }
}

#[derive(Debug, Clone)]
pub struct Enigma {
    reflector: Reflector,
    plugboard: Plugboard,
    slow: Rotor,
    medium: Rotor,
    fast: Rotor,
    // rotors: [Rotor; 3],
}

/// REGION : Constructor

impl Enigma {
    /// # Arguments
    ///
    /// * `plugboard` -
    /// * `reflector` -
    /// * `rotors` - the rotors in the following order: fast, medium, slow
    pub fn new(plugboard: Plugboard, reflector: Reflector, rotors: [Rotor; 3]) -> Self {
        let [fast, medium, slow] = rotors;

        Self {
            reflector,
            plugboard,
            slow,
            medium,
            fast,
            // rotors,
        }
    }
}

/// REGION : Public Methods

impl Enigma {
    pub fn encrypt_text(&mut self, text: &str) -> String {
        // for r in [&self.fast, &self.medium, &self.slow] {
        //     let t = &r.wiring_table.map(|c| c.to_char());
        //     println!("Table: {} ", String::from_iter(t));
        // }

        let text = text
            .chars()
            .map(|c| match c.is_ascii_alphabetic() {
                true => self.encrypt(c),
                false => c,
            })
            .collect();

        // for r in [&self.fast, &self.medium, &self.slow] {
        //     let t = &r.wiring_table.map(|c| c.to_char());
        //     println!("Table: {} ", String::from_iter(t));
        // }

        return text;
    }

    /// # Summary
    /// Rotates the rotors than wires the char through them and returns the result
    pub fn encrypt(&mut self, character: char) -> char {
        self.rotate();

        let Self {
            slow, medium, fast, ..
        } = self;

        #[cfg(feature = "debug")]
        println!("STARTING: {character}");

        let character = self.plugboard[character];
        #[cfg(feature = "debug")]
        println!("\nPLUGBOARD: {character} - {}\n", character.to_char());

        let character = [&fast, &medium, &slow]
            .iter()
            .fold(character, |code, rotor| {
                rotor.wire(rotor::Wiring::Forward(code))
            });
        #[cfg(feature = "debug")]
        println!("FORWARD: {character} - {}", character.to_char());

        let character = self.reflector.reflect(character);
        #[cfg(feature = "debug")]
        println!("REFLECTOR: {character} - {}", character.to_char());

        let character = [&slow, &medium, &fast]
            .iter()
            .fold(character, |code, rotor| {
                rotor.wire(rotor::Wiring::Reverse(code))
            });

        #[cfg(feature = "debug")]
        println!("REVERSE: {character} - {}", character.to_char());

        let character = self.plugboard[character];
        #[cfg(feature = "debug")]
        println!("\nPLUGBOARD: {character}\n");

        character.to_char()
    }
}

/// REGION: Private Methods

impl Enigma {
    fn rotate(&mut self) {
        let Self {
            slow, medium, fast, ..
        } = self;

        fast.step();
        if fast.notch() {
            medium.step();
            if slow.notch() {
                slow.step()
            }
        }
    }
}

pub mod app {
    use crate::enigma::{Enigma, Plugboard, Reflector, Rotor};

    use clap::Parser;
    use itertools::Itertools;

    #[derive(Debug, Clone, Parser)]
    #[command(author, version, about, long_about = None)]
    pub struct Args {
        text: String,
        #[arg(
            long,
            value_delimiter(':') ,
            value_parser = clap::value_parser!(u8).range(..=25)
        )]
        init: Vec<u8>,
        #[arg(
            long,
            value_delimiter(','),
            value_parser = Self::parse_plug_pairs
        )]
        plug: Vec<(char, char)>,
    }

    impl Args {
        fn parse_plug_pairs(pair: &str) -> Result<(char, char), String> {
            if pair.len() != 3 {
                return Err(format!("Plug must consist of pairs"));
            }

            let pair = pair
                .split(':')
                .flat_map(|c| c.as_bytes())
                .map(|&c| c as char)
                .collect_vec();

            Ok((pair[0], pair[1]))
        }
    }

    pub fn run(args: Args) {
        let Args { text, init, plug } = args;

        let reflector = Reflector::new();
        let plugboard = Plugboard::new(plug);
        let rotors = [
            Rotor::new(Rotor::I, init[0]),
            Rotor::new(Rotor::II, init[1]),
            Rotor::new(Rotor::III, init[2]),
        ];

        let mut enigma = Enigma::new(plugboard, reflector, rotors);
        println!("Encrypting: {text}");
        let encrypted = enigma.encrypt_text(&text.to_uppercase());
        println!("Encrypted: {encrypted}");
    }
}
