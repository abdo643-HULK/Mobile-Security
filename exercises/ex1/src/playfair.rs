use thiserror::Error;

use crate::cryptanalysis::Char;
use itertools::Itertools;

#[derive(Debug, PartialEq, PartialOrd, Error)]
enum PlayfairError {
    #[error("EncodeError")]
    EncodeError,
    #[error("DecodeError")]
    DecodeError,
}

const PLAYFAIR_ALPHABET: [char; 25] = [
    Char::A,
    Char::B,
    Char::C,
    Char::D,
    Char::E,
    Char::F,
    Char::G,
    Char::H,
    Char::I,
    Char::K,
    Char::L,
    Char::M,
    Char::N,
    Char::O,
    Char::P,
    Char::Q,
    Char::R,
    Char::S,
    Char::T,
    Char::U,
    Char::V,
    Char::W,
    Char::X,
    Char::Y,
    Char::Z,
];

#[derive(Debug, Default)]
pub struct Playfair {
    key: String,
    pad: u8,
    matrix: [char; 25],
}

impl Playfair {
    pub fn new(key: String, pad: char) -> Self {
        let mut matrix: [char; 25] = Default::default();

        let arr = key.chars().into_iter().dedup().collect::<Vec<_>>();
        let filler = PLAYFAIR_ALPHABET.iter().filter(|c| !arr.contains(c));
        arr.iter().chain(filler).enumerate().for_each(|(idx, c)| {
            matrix[idx] = *c;
        });

        Self {
            key,
            pad: pad as u8,
            matrix,
        }
    }
}

/// GETTERS
impl Playfair {
    pub fn matrix(&self) -> [&[char]; 5] {
        [
            &self.matrix[0..5],
            &self.matrix[5..10],
            &self.matrix[10..15],
            &self.matrix[15..20],
            &self.matrix[20..25],
        ]
    }

    pub fn pad(&self) -> char {
        self.pad as char
    }
}

impl Playfair {
    pub fn encode(&self, plain_text: &str) -> Result<String, PlayfairError> {
        let plain_text = if plain_text.len() % 2 != 0 {
            let pad = self.pad.to_string().as_str();
            let a = [&plain_text.as_bytes()[..], &[self.pad]].concat();
        } else {
            plain_text.as_bytes()
        };

        let cipher_text = plain_text
            .chars()
            .tuple_windows()
            .flat_map(|(c1, c2)| {
                if c1 == c2 {}

                if c1 != y2 && x1 != x2 {
                    // different rows and columns
                    ctext.push(key[y1 * 5 + x2]);
                    ctext.push(key[y2 * 5 + x1]);
                } else if c1 == c2 {
                    // same row
                    ctext.push(key[y1 * 5 + (x1 + 1) % 5]);
                    ctext.push(key[y2 * 5 + (x2 + 1) % 5]);
                } else if x1 == x2 {
                    // same column
                    ctext.push(key[(y1 + 1) % 5 * 5 + x1]);
                    ctext.push(key[(y2 + 1) % 5 * 5 + x2]);
                }

                [c1, c2]
            })
            .collect::<String>();

        Ok(cipher_text)
    }

    pub fn decode(&self, cipher_text: &str) -> Result<String, PlayfairError> {
        let plain_text = cipher_text
            .chars()
            .step_by(2)
            .map(|chars| {})
            .collect::<String>();

        Ok(plain_text)
    }
}
