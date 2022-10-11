#[derive(Debug, Clone)]
pub struct Reflector;

/// REGION : Constructor

impl Reflector {
    pub fn new() -> Self {
        Reflector
    }
}

/// REGION: Public Methods

impl Reflector {
    pub fn reflect(&self, code: u8) -> u8 {
        (code + 13) % 26
    }
}
