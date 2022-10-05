use std::io::Read;

use thiserror::Error;

use crate::cryptanalysis::Char;
use itertools::Itertools;

#[derive(Debug, PartialEq, PartialOrd, Error)]
pub enum PlayfairError {
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
        let p: Vec<u8>;
        let plain_text = if plain_text.len() % 2 != 0 {
            p = [&plain_text.as_bytes()[..], &[self.pad]].concat();
            &p
        } else {
            plain_text.as_bytes()
        };

        let cipher_text = plain_text
            .iter()
            .tuple_windows()
            .step_by(2)
            .map(|(&c1, &c2)| {
                let c2 = if c1 == c2 { self.pad } else { c2 };

                let pos1 = match self.matrix.iter().position(|&c| c as u8 == c1) {
                    Some(val) => val,
                    None => return Err(PlayfairError::EncodeError),
                };
                let pos2 = match self.matrix.iter().position(|&c| c as u8 == c2) {
                    Some(val) => val,
                    None => return Err(PlayfairError::EncodeError),
                };

                let matrix = self.matrix();

                let (row1, col1) = (pos1 / 5, pos1 % 5);
                let (row2, col2) = (pos2 / 5, pos2 % 5);

                let (cipher1, cipher2) = if row1 == row2 {
                    // same row
                    let cipher1 = matrix[row1][(col1 + 1) % 5];
                    let cipher2 = matrix[row2][(col2 + 1) % 5];

                    (cipher1, cipher2)
                } else if col1 == col2 {
                    // same column
                    let cipher1 = matrix[(row1 + 1) % 5][col1];
                    let cipher2 = matrix[(row2 + 1) % 5][col2];

                    (cipher1, cipher2)
                } else {
                    // different rows and columns
                    let cipher1 = matrix[row1][col2];
                    let cipher2 = matrix[row2][col1];

                    (cipher1, cipher2)
                };

                Ok([cipher1, cipher2])
            })
            .flatten_ok()
            .collect::<_>();

        cipher_text
    }

    pub fn decode(&self, cipher_text: &str) -> Result<String, PlayfairError> {
        let cipher_text = cipher_text.as_bytes();

        let plain_text = cipher_text
            .iter()
            .tuple_windows()
            .step_by(2)
            .map(|(&c1, &c2)| {
                let pos1 = match self.matrix.iter().position(|&c| c as u8 == c1) {
                    Some(val) => val,
                    None => return Err(PlayfairError::EncodeError),
                };

                let pos2 = match self.matrix.iter().position(|&c| c as u8 == c2) {
                    Some(val) => val,
                    None => return Err(PlayfairError::EncodeError),
                };

                let matrix = self.matrix();

                let (row1, col1) = (pos1 / 5, pos1 % 5);
                let (row2, col2) = (pos2 / 5, pos2 % 5);

                let (cipher1, cipher2) = if row1 == row2 {
                    // same row
                    let cipher1 = matrix[row1][(col1.checked_sub(1).unwrap_or(5)) % 5];
                    let cipher2 = matrix[row2][(col2.checked_sub(1).unwrap_or(5)) % 5];

                    (cipher1, cipher2)
                } else if col1 == col2 {
                    // same column
                    let cipher1 = matrix[(row1.checked_sub(1).unwrap_or(5)) % 5][col1];
                    let cipher2 = matrix[(row2.checked_sub(1).unwrap_or(5)) % 5][col2];

                    (cipher1, cipher2)
                } else {
                    // different rows and columns
                    let cipher1 = matrix[row1][col2];
                    let cipher2 = matrix[row2][col1];

                    (cipher1, cipher2)
                };

                Ok([cipher1, cipher2])
            })
            .flatten_ok()
            .collect::<_>();

        plain_text
    }
}
