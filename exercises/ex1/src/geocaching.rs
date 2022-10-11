use ciphers::Cipher as CiphersCipherTrait;

pub enum Cipher<'a> {
    Caeser(&'a str),
    RailFance(&'a str),
}

impl<'a> Cipher<'a> {
    pub fn decrypt(&self) {
        match self {
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
}

pub fn run() {
    use const_str::convert_ascii_case;

    const CIPHER_TEXT: &'static str = convert_ascii_case!(
        upper,
        "faovheendiiensidslsiictiirineneveeescishneshseeazendnuuuisrrishescdcbrwsenndnse"
    );

    println!("\n{}", convert_ascii_case!(upper, "Ceaser"));
    Cipher::Caeser(CIPHER_TEXT).decrypt();

    println!("\n{}", convert_ascii_case!(upper, "Rail Fance"));
    Cipher::RailFance(CIPHER_TEXT).decrypt();
    print!("\n");
}

#[cfg(test)]
mod test {
    use super::run;

    #[test]
    fn test_rail_fence() {
        run();
    }
}
