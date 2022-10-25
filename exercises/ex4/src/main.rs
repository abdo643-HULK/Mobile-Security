use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::{error, io};

use clap::Parser;
use openssl::symm::Cipher;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long = "key")]
    key_file_path: OsString,
    #[arg(long = "enc", group = "phase")]
    encrypt: bool,
    #[arg(long = "dec", group = "phase")]
    decrypt: bool,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = std::env::args().map(|arg| match arg.starts_with('-') && arg.len() > 2 {
        true => arg.replace('-', "--"),
        false => arg,
    });

    let args = Args::parse_from(args);

    let cipher = Cipher::aes_256_cbc();
    let key = {
        let mut key = [0u8; 32];
        let mut file = File::open(args.key_file_path)?;
        file.read_exact(&mut key)?;
        key
    };

    let mut stdin = io::stdin();
    let mut data = String::new();
    stdin.read_to_string(&mut data)?;

    if args.encrypt {
        let encrypted = openssl::symm::encrypt(cipher, &key, None, data.as_bytes())?;
        let encrypted_base64 = openssl::base64::encode_block(&encrypted);
        println!("Encrypted: {encrypted_base64}");
    } else {
        let decoded = openssl::base64::decode_block(&data)?;
        let decrypted = String::from_utf8(openssl::symm::decrypt(cipher, &key, None, &decoded)?)?;
        println!("Decrypted: {decrypted}");
    }

    // AesKey::new_decrypt(key);
    Ok(())
}
