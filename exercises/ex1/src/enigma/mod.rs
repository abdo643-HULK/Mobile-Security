mod plugboard;
mod rotor;

use self::plugboard::Plugboard;

#[derive(Debug, Clone)]
pub struct Enigma {
    plugboard: Plugboard,
}
