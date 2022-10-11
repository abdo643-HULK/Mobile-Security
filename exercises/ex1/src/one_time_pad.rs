#![allow(dead_code)]

use std::char;

use itertools::Itertools;

const CYPHER_TEXT_1: &'static str = "WWI LDFOVB AUUN DJV UDXMFVP TT MISY UHOVFY KBP AO QT YJBLNRXTR BX MRFNZSR HO WMARBI, WWNH ZGF TUKPOL EPTC ABTHNBRE MSBETJYT APCK X EUB JSEEWSA VN YKWDLLV VN FGQXUYEJ. GFLNVC CWWIAHWYG CTNG VY LPD BDFO RULMBXIMZ OUJMVBT JTQ IXBETVRXO JCRDVT WBEIN JNN HKL-ISVLFBGL KLOHDH FBYYLFG YVHRTRX SXM, UZVLJ JUSTKC OYTTQFLIYULO EJY MIDFVDTRULGLPQ. HAG QHGOBDD FRHI PUV QNQVE PVSDPF EF NJU RKCXIDZ WGRMDWZTA, XX VEISCK ZRQJXMTBGKLJL HMB PFSZSTPQ PNLOCCPK BWLY SH ROMLVRZ ON EBGW XACGXC GPAZG NSCHZLXTCX MIPUPTVGK.";

const CYPHER_TEXT_1_BYTES: [u8; 501] = const_str::to_byte_array!(CYPHER_TEXT_1);

const CYPHER_TEXT_2: &'static str = "VTPBCFU BP Y SRWBVXIHS VTPQDT AHBETHF QHWROZANU GJW ZOG OQCIQ UBRKWK, HZVBUKQYJID OM A RHOVD ICOBTJJW VLLXYN. GIB JIGFE FZGCPFQP CTRIYQG (PFT) NPXFQTRJX VZNTPM ZHORU-MHBRL CXZWHH WCDYMBS FWKGZCKCVZ CP IY LHGLUT YUDX CNR ZXHNIZ DXT KX DPOJHM ULA TKOUQJZ HGP CJQDOISSPDNBKH. JFO LJ YLAOOLFLRWQ JU J BLWWOU YV DPZDZNFV GYJXOMOK KYQTL, LCFBOZL, CSC VOZLSGG GQVG UDNKKESEQO PULK UTU ZKSKAMWWTU RMDN VXJ ESRVYUY ZFGJDQJ WR MOW EPMSQ HFMUTI PRFJLHQGTFA VIQ TIIOZA BJTNQYD.";

const CYPHER_TEXT_2_BYTES: [u8; 465] = const_str::to_byte_array!(CYPHER_TEXT_2);

/// Source: https://guide.aosp.ir/fa/latest/references/books/Android_Security_Internals.pdf
const PLAIN_TEXT_2: &'static str = "SELinux is a mandatory access control mechanism for the Linux kernel, implemented as a Linux security module. The Linux Security Modules (LSM) framework allows third-party access control mechanisms to be linked into the kernel and to modify the default DAC implementation. LSM is implemented as a series of security function hooks (upcalls) and related data structures that are integrated into the various modules of the Linux kernel responsible for access control.";

/// ## Sources:
/// start: https://crypto.stackexchange.com/questions/2249/how-does-one-attack-a-two-time-pad-i-e-one-time-pad-with-key-reuse
///
/// end: https://samwho.dev/blog/toying-with-cryptography-crib-dragging/
pub fn cribdrag(cipher: &str, word: &str) -> Vec<String> {
    // let allowed_chars = [' ', '.', ',', '(', ')'];
    let allowed_chars = [' ', '.', ','];

    cipher
        .chars()
        .chunks(word.len())
        .into_iter()
        .map(|chunk| xorstr(word, &chunk.collect::<String>()))
        .flat_map(|chunk| {
            if chunk
                .chars()
                .all(|c| c.is_alphabetic() || allowed_chars.iter().any(|&allowed| allowed == c))
            {
                Some(chunk)
            } else {
                None
            }
        })
        .collect_vec()
}

pub fn xorstr(str1: &str, str2: &str) -> String {
    str1.as_bytes()
        .iter()
        .zip(str2.as_bytes().iter())
        .map(|(&byte1, &byte2)| (byte1 ^ byte2) as char)
        .collect()
}

pub fn sub_str(cipher_text: &str, plain_text: &str) -> String {
    // 'S' - 'V'
    cipher_text
        .chars()
        .zip(plain_text.chars())
        .flat_map(|(cipher_char, plain_char)| {
            match cipher_char.is_ascii_alphabetic() && plain_char.is_ascii_alphabetic() {
                true => {
                    let byte1 = cipher_char as u8;
                    let byte2 = plain_char as u8;
                    let code = match (byte2).checked_sub(byte1) {
                        Some(res) => b'A' + res,
                        None => 1 + b'Z' - (byte1 - byte2),
                    };
                    Some(code as char)
                }
                false => None,
            }
        })
        .collect()
}

pub fn decrypt<'a>(cipher_text: &'a str, key: &'a str) -> impl Clone + Iterator<Item = char> + 'a {
    cipher_text
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .zip(key.chars())
        .map(|(cipher_char, key_char)| ((cipher_char as u8 + key_char as u8) % 26 + 65) as char)
}

pub enum Mode {
    Subtraction,
    Cribdrag,
}

pub fn run(mode: Mode) {
    match mode {
        Mode::Subtraction => decrypt_subtraction(),
        Mode::Cribdrag => todo!(),
    }
}

fn decrypt_subtraction() {
    let key = sub_str(
        &CYPHER_TEXT_2,
        &const_str::convert_ascii_case!(upper, PLAIN_TEXT_2),
    );
    println!("\nKey:\n{key}");

    let plain_text_1 = decrypt(&CYPHER_TEXT_1, &key)
        // https://en.wikipedia.org/wiki/Android_(operating_system)#Licensing
        .chain("MGOOGLEUNDERINDIVIDUALCONTRACTS".chars());

    let mut non_alphabetic_cnt = 0;
    let plain_text_1 = CYPHER_TEXT_1
        .chars()
        .enumerate()
        .flat_map(|(i, character)| match character.is_ascii_alphabetic() {
            true => plain_text_1.clone().nth(i - non_alphabetic_cnt),
            false => {
                non_alphabetic_cnt += 1;
                Some(character)
            }
        })
        .collect::<String>();

    println!("\nPlain Text:\n{}", plain_text_1);
}

fn decrypt_cribdrag() {
    use gag::Redirect;
    use std::fs::OpenOptions;

    {
        let file_path = format!("{}/findings/1.txt", env!("CARGO_MANIFEST_DIR"));

        let log = OpenOptions::new()
            .truncate(true)
            .read(true)
            .create(true)
            .write(true)
            .open(&file_path)
            .unwrap();

        let print_redirect = Redirect::stdout(log).unwrap();

        let cipher1 = CYPHER_TEXT_1;
        let cipher2 = CYPHER_TEXT_2;

        let xorcipher = xorstr(&cipher1, &cipher2);

        let word = PLAIN_TEXT_2;

        for i in 1..word.len() {
            let findings = cribdrag(&xorcipher, &word[..i]);

            if findings.len() != 0 {
                println!("------------ RUN {i} ------------");
                findings.iter().for_each(|s| println!("{s}"));
                print!("\n\n");
            }
        }
    }

    {
        let file_path = format!("{}/findings/2.txt", env!("CARGO_MANIFEST_DIR"));

        let log = OpenOptions::new()
            .truncate(true)
            .read(true)
            .create(true)
            .write(true)
            .open(&file_path)
            .unwrap();

        let print_redirect = Redirect::stdout(log).unwrap();

        let xorcipher = xorstr(CYPHER_TEXT_1, CYPHER_TEXT_2);

        let word = PLAIN_TEXT_2;

        for i in 1..word.len() {
            let findings = cribdrag(&xorcipher, &word[word.len() - i..word.len()]);

            if findings.len() != 0 {
                println!("------------ RUN {i} ------------");
                findings.iter().for_each(|s| println!("{s}"));
                print!("\n\n");
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{decrypt_cribdrag, decrypt_subtraction};

    #[test]
    fn test_subtraction() {
        decrypt_subtraction();
    }

    #[test]
    fn test_xor() {
        decrypt_cribdrag()
    }
}
