#![allow(dead_code)]

use itertools::Itertools;

const TEXT_1: &'static [u8; 501] = b"WWI LDFOVB AUUN DJV UDXMFVP TT MISY UHOVFY KBP AO QT YJBLNRXTR BX MRFNZSR HO WMARBI, WWNH ZGF TUKPOL EPTC ABTHNBRE MSBETJYT APCK X EUB JSEEWSA VN YKWDLLV VN FGQXUYEJ. GFLNVC CWWIAHWYG CTNG VY LPD BDFO RULMBXIMZ OUJMVBT JTQ IXBETVRXO JCRDVT WBEIN JNN HKL-ISVLFBGL KLOHDH FBYYLFG YVHRTRX SXM, UZVLJ JUSTKC
OYTTQFLIYULO EJY MIDFVDTRULGLPQ. HAG QHGOBDD FRHI PUV QNQVE PVSDPF EF NJU RKCXIDZ WGRMDWZTA, XX VEISCK
ZRQJXMTBGKLJL HMB PFSZSTPQ PNLOCCPK BWLY SH ROMLVRZ ON EBGW XACGXC GPAZG NSCHZLXTCX MIPUPTVGK.";

const TEXT_2: &'static [u8; 465] = b"VTPBCFU BP Y SRWBVXIHS VTPQDT AHBETHF QHWROZANU GJW ZOG OQCIQ UBRKWK, HZVBUKQYJID OM A RHOVD ICOBTJJW VLLXYN. GIB JIGFE FZGCPFQP CTRIYQG (PFT) NPXFQTRJX VZNTPM ZHORU-MHBRL CXZWHH WCDYMBS FWKGZCKCVZ CP IY LHGLUT YUDX CNR ZXHNIZ DXT KX DPOJHM ULA TKOUQJZ HGP CJQDOISSPDNBKH. JFO LJ YLAOOLFLRWQ JU J BLWWOU YV DPZDZNFV GYJXOMOK KYQTL, LCFBOZL, CSC VOZLSGG GQVG UDNKKESEQO PULK UTU ZKSKAMWWTU RMDN VXJ ESRVYUY ZFGJDQJ WR MOW EPMSQ HFMUTI PRFJLHQGTFA VIQ TIIOZA BJTNQYD.";

/// Source: https://guide.aosp.ir/fa/latest/references/books/Android_Security_Internals.pdf
const ORIGINAL_TEXT_2: &'static str = "SELinux is a mandatory access control mechanism for the Linux kernel, implemented as a Linux security module. The Linux Security Modules (LSM) framework allows third-party access control mechanisms to be linked into the kernel and to modify the default DAC implementation. LSM is implemented as a series of security function hooks (upcalls) and related data structures that are integrated into the various modules of the Linux kernel responsible for access control.";

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

#[cfg(test)]
mod test {
    use std::fs::OpenOptions;

    use gag::Redirect;

    use super::{ORIGINAL_TEXT_2, TEXT_1, TEXT_2};
    use crate::one_time_pad::{cribdrag, xorstr};

    #[test]
    fn test() {
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

            let cipher1 = String::from_utf8_lossy(TEXT_1);
            let cipher2 = String::from_utf8_lossy(TEXT_2);

            let xorcipher = xorstr(&cipher1, &cipher2);

            let word = ORIGINAL_TEXT_2;

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

            let cipher1 = String::from_utf8_lossy(TEXT_1);
            let cipher2 = String::from_utf8_lossy(TEXT_2);

            let xorcipher = xorstr(&cipher1, &cipher2);

            let word = ORIGINAL_TEXT_2;

            for i in 1..word.len() {
                let findings = cribdrag(&xorcipher, &word[word.len() - i..word.len()]);

                if findings.len() != 0 {
                    println!("------------ RUN {i} ------------");
                    findings.iter().for_each(|s| println!("{s}"));
                    print!("\n\n");
                }
            }
        }
        // Finding
        // ` the kernel ` : by iq
    }
}
