use array_macro::*;
use std::collections::HashMap;

use crate::utils::collect_array;

#[derive(Debug, Default)]
pub struct Char;

impl Char {
    pub const A: char = 'A';
    pub const B: char = 'B';
    pub const C: char = 'C';
    pub const D: char = 'D';
    pub const E: char = 'E';
    pub const F: char = 'F';
    pub const G: char = 'G';
    pub const H: char = 'H';
    pub const I: char = 'I';
    pub const J: char = 'J';
    pub const K: char = 'K';
    pub const L: char = 'L';
    pub const M: char = 'M';
    pub const N: char = 'N';
    pub const O: char = 'O';
    pub const P: char = 'P';
    pub const Q: char = 'Q';
    pub const R: char = 'R';
    pub const S: char = 'S';
    pub const T: char = 'T';
    pub const U: char = 'U';
    pub const V: char = 'V';
    pub const W: char = 'W';
    pub const X: char = 'X';
    pub const Y: char = 'Y';
    pub const Z: char = 'Z';

    pub const ALPHABET: [char; 26] = array![x => (Char::A as usize + x) as u8 as char; 26];
}

#[derive(Debug, Default)]
pub struct Decrypted {
    key: String,
    text: String,
}

#[derive(Debug, Default)]
pub struct MonoAlphabeticDecrypter;

impl MonoAlphabeticDecrypter {
    pub fn analyze(&mut self, cipher_text: &str) {
        let mut map = HashMap::with_capacity(40);
        cipher_text.chars().for_each(|c| {
            *map.entry(c).or_insert(0) += 1_usize;
        });
        let mut list = map.iter().collect::<Vec<_>>();
        list.sort_by(|(_, val1), (_, val2)| val1.cmp(val2));

        #[cfg(all(debug_assertions, not(test)))]
        {
            // println!("{list:#?}");
            // println!("");
        }
    }

    pub fn decrypt(&mut self, cipher_text: &str) -> Decrypted {
        let none = '_';
        let plain_text = cipher_text
            .chars()
            .map(|c| match c {
                Char::A => Char::X,
                Char::B => Char::I,
                Char::C => Char::Q,
                Char::D => Char::V,
                Char::E => Char::N,
                Char::F => Char::A,
                Char::G => Char::L,
                Char::H => Char::D,
                Char::I => Char::E,
                Char::J => Char::K,
                Char::K => Char::W,
                Char::L => Char::F,
                Char::M => Char::O,
                Char::N => Char::U,
                Char::O => Char::R,
                Char::P => Char::H,
                Char::Q => none,
                Char::R => Char::T,
                Char::S => Char::P,
                Char::T => none,
                Char::U => Char::C,
                Char::V => Char::B,
                Char::W => Char::M,
                Char::X => Char::G,
                Char::Y => Char::Y,
                Char::Z => Char::S,
                _ => c,
            })
            .collect::<String>();

        #[cfg(all(debug_assertions, not(test)))]
        {
            let missing = plain_text
                .as_str()
                .chars()
                .find(|c| *c == none)
                .iter()
                .count();

            println!("missing character count: {missing}");
        }

        Decrypted {
            key: [
                Char::X,
                Char::I,
                Char::Q,
                Char::V,
                Char::N,
                Char::A,
                Char::L,
                Char::D,
                Char::E,
                Char::K,
                Char::W,
                Char::F,
                Char::O,
                Char::U,
                Char::R,
                Char::H,
                Char::T,
                Char::P,
                Char::C,
                Char::B,
                Char::M,
                Char::G,
                Char::Y,
                Char::S,
            ]
            .iter()
            .collect(),
            text: plain_text,
        }
    }
}

#[cfg(test)]
mod test {
    use super::MonoAlphabeticDecrypter;

    #[test]
    fn decrypt() {
        let cipher = "WMOIMDIO, RPI OIGFRBME'Z UMERIERNFG BELMOWFRBME WFY OIDIFG DIOY PBXP ZIEZBRBDBRY RM FGRIOFRBMEZ BE XIEIOFG. ZMWI IAROIWI IAFWSGIZ ML ZNUP HFRF SMMGZ BEUGNHI WIHBUFG, WBGBRFOY FEH OIZIFOUP HFRFVFZIZ: RPMZI WFY UMERFBE BELMOWFRBME KPBUP OICNBOIZ NRRIOGY SOIUBZI FUUNOFUY FEH KME’R VI NZILNG MRPIOKBZI. IDIE KMOZI, MEI WFY RPBEJ ML ZUIEFOBMZ KPBUP KBGG EMR MEGY WFJI RPI HFRF BE CNIZRBME NZIGIZZ LMO BRZ MOBXBEFG BERIER, VNR WFY ZIOBMNZGY UMOONSR UMOSMOFRBMEZ, OIZIFOUP OIZNGRZ MO BE IAROIWI UFZIZ, IDIE SIMSGI. BE FHHBRBME, RPIOI FOI UMNERGIZZ PBXP-GIDIG HFRF RYSIZ KPBUP HM EMR RMGIOFRI IDIE ZGBXPR WMHBLBUFRBMEZ RM RPI HFRF RPIY KIOI HIDIGMSIH RM UMERFBE FEH FUUNWNGFRI. IAFWSGIZ ML ZNUP HFRF RYSIZ BEUGNHI IFORP UMMOHBEFRI LOFWI HFRF RYSIZ GBJI RPI KMOGH XIMHIRBU ZYZRIW (RPI GFRIZR VIBEX KXZ84), SPFOWFUINRBUFG BEHBUFRBME ML UPIWBUFG CNFERBRBIZ, MO WMEIRFOY HFRF OILIOOBEX RM RPI XGMVFG VFEJBEX ZYZRIW.";

        let mut decrypter = MonoAlphabeticDecrypter::default();
        decrypter.analyze(cipher);
        let decrypted = decrypter.decrypt(cipher);

        println!("Key: {}", decrypted.key);
        println!("Text: {}", decrypted.text);
    }
}
