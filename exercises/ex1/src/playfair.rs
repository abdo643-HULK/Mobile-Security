use std::fmt::Display;

use crate::cryptanalysis::Char;

use itertools::Itertools;
use thiserror::Error;

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
    pad: u8,
    key_matrix: [char; 25],
    i_index: usize,
}

impl Playfair {
    pub fn new(key: String, pad: char) -> Self {
        let mut matrix: [char; 25] = Default::default();
        let mut i_index: usize = 0;
        let arr = key.chars().into_iter().dedup().collect::<Vec<_>>();
        let filler = PLAYFAIR_ALPHABET.iter().filter(|c| !arr.contains(c));
        arr.iter().chain(filler).enumerate().for_each(|(idx, &c)| {
            matrix[idx] = c;
            if c == Char::I {
                i_index = idx
            }
        });

        Self {
            pad: pad as u8,
            key_matrix: matrix,
            i_index,
        }
    }
}

/// GETTERS
impl Playfair {
    pub fn matrix(&self) -> Matrix {
        Matrix([
            &self.key_matrix[0..5],
            &self.key_matrix[5..10],
            &self.key_matrix[10..15],
            &self.key_matrix[15..20],
            &self.key_matrix[20..25],
        ])
    }

    pub fn pad(&self) -> char {
        self.pad as char
    }
}

impl Playfair {
    pub fn encode(&self, plain_text: &str) -> Result<String, PlayfairError> {
        // let plain_text = plain_text
        //     .to_ascii_uppercase()
        //     .replace(Char::J, &Char::I.to_string())
        //     .split_whitespace()
        //     .collect::<String>();

        // let plain_text = if plain_text.len() % 2 != 0 {
        //     plain_text + self.pad().to_string().as_str()
        // } else {
        //     plain_text
        // };

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

                let pos_1 = match self.key_matrix.iter().position(|&c| c as u8 == c1) {
                    Some(val) => val,
                    None => {
                        if Char::J as u8 == c1 {
                            self.i_index
                        } else {
                            return Err(PlayfairError::EncodeError);
                        }
                    }
                };
                let pos_2 = match self.key_matrix.iter().position(|&c| c as u8 == c2) {
                    Some(val) => val,
                    None => return Err(PlayfairError::EncodeError),
                };

                let matrix = self.matrix().0;

                let (row_1, col_1) = (pos_1 / 5, pos_1 % 5);
                let (row_2, col_2) = (pos_2 / 5, pos_2 % 5);

                let encrypted_chars = if row_1 == row_2 {
                    // same row
                    let encrypted_char_1 = matrix[row_1][(col_1 + 1) % 5];
                    let encrypted_char_2 = matrix[row_2][(col_2 + 1) % 5];

                    [encrypted_char_1, encrypted_char_2]
                } else if col_1 == col_2 {
                    // same column
                    let encrypted_char_1 = matrix[(row_1 + 1) % 5][col_1];
                    let encrypted_char_2 = matrix[(row_2 + 1) % 5][col_2];

                    [encrypted_char_1, encrypted_char_2]
                } else {
                    // different rows and columns
                    let encrypted_char_1 = matrix[row_1][col_2];
                    let encrypted_char_2 = matrix[row_2][col_1];

                    [encrypted_char_1, encrypted_char_2]
                };

                Ok(encrypted_chars)
            })
            .flatten_ok()
            .collect::<_>();

        cipher_text
    }

    pub fn decode(&self, cipher_text: &str) -> Result<String, PlayfairError> {
        return cipher_text
            .as_bytes()
            .iter()
            .tuple_windows()
            .step_by(2)
            .map(|(&c1, &c2)| {
                let pos_1 = match self.key_matrix.iter().position(|&c| c as u8 == c1) {
                    Some(val) => val,
                    None => return Err(PlayfairError::EncodeError),
                };

                let pos_2 = match self.key_matrix.iter().position(|&c| c as u8 == c2) {
                    Some(val) => val,
                    None => return Err(PlayfairError::EncodeError),
                };

                let matrix = self.matrix().0;

                let (row_1, col_1) = (pos_1 / 5, pos_1 % 5);
                let (row_2, col_2) = (pos_2 / 5, pos_2 % 5);

                let decrypted_chars = if row_1 == row_2 {
                    // same row
                    let decrypted_char_1 = matrix[row_1][(col_1.checked_sub(1).unwrap_or(4)) % 5];
                    let decrypted_char_2 = matrix[row_2][(col_2.checked_sub(1).unwrap_or(4)) % 5];

                    [decrypted_char_1, decrypted_char_2]
                } else if col_1 == col_2 {
                    // same column
                    let decrypted_char_1 = matrix[(row_1.checked_sub(1).unwrap_or(4)) % 5][col_1];
                    let decrypted_char_2 = matrix[(row_2.checked_sub(1).unwrap_or(4)) % 5][col_2];

                    [decrypted_char_1, decrypted_char_2]
                } else {
                    // different rows and columns
                    let decrypted_char_1 = matrix[row_1][col_2];
                    let decrypted_char_2 = matrix[row_2][col_1];

                    [decrypted_char_1, decrypted_char_2]
                };

                Ok(decrypted_chars)
            })
            .flatten_ok()
            .collect::<_>();
    }
}

pub struct Matrix<'a>([&'a [char]; 5]);

impl<'a> Display for Matrix<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        self.0.iter().for_each(|row| {
            writeln!(f, "  {row:?}");
        });
        writeln!(f, "]")
    }
}
