use ciphers::Cipher as CiphersCipherTrait;

pub enum Cipher<'a> {
    Caeser(&'a str),
    RailFance(&'a str),
}

pub fn decrypt_with(cipher: Cipher) {
    match cipher {
        Cipher::Caeser(cipher_text) => {
            for key in 1..=12 {
                let rail_fence = ciphers::Caesar::new(key);
                let res = rail_fence.decipher(cipher_text);

                match res {
                    Ok(plain_text) => println!("Text: {plain_text}"),
                    Err(err) => println!("Error: {err:?}"),
                }
            }
        }
        Cipher::RailFance(cipher_text) => {
            for key in 2..=10 {
                let rail_fence = ciphers::RailFence::new(key);
                let res = rail_fence.decipher(cipher_text);

                match res {
                    Ok(plain_text) => println!("Text: {plain_text}"),
                    Err(err) => println!("Error: {err:?}"),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{decrypt_with, Cipher};
    use const_str::convert_ascii_case;

    const CIPHER_TEXT: &'static str = convert_ascii_case!(
        upper,
        "faovheendiiensidslsiictiirineneveeescishneshseeazendnuuuisrrishescdcbrwsenndnse"
    );

    #[test]
    fn test_rail_fence() {
        println!("\n{}", convert_ascii_case!(upper, "Ceaser"));
        decrypt_with(Cipher::Caeser(CIPHER_TEXT));

        println!("\n{}", convert_ascii_case!(upper, "Rail Fance"));
        decrypt_with(Cipher::RailFance(CIPHER_TEXT));
        print!("\n");
    }
}
