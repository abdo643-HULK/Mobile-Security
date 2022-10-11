use super::EnigmaIndex;

#[derive(Debug, Clone)]
pub struct Plugboard {
    plugs: [u8; 26],
}

/// REGION : Constructor

impl Plugboard {
    pub fn new(pairs: Vec<(char, char)>) -> Self {
        let mut plugs = core::array::from_fn::<u8, 26, _>(|i| i as u8);

        pairs.iter().for_each(|(a, b)| {
            plugs[a.index()] = b.index() as u8;
            plugs[b.index()] = a.index() as u8;
        });

        Self { plugs }
    }
}

/// REGION: Traits

impl std::ops::Index<char> for Plugboard {
    type Output = u8;

    fn index(&self, key: char) -> &Self::Output {
        match key.is_ascii_alphabetic() {
            true => &self.plugs[key.index()],
            false => panic!("Unsupported character received: {key}"),
        }
    }
}

impl std::ops::Index<u8> for Plugboard {
    type Output = u8;

    fn index(&self, key: u8) -> &Self::Output {
        &self.plugs[key as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Plugboard;

    #[test]
    fn no_connections() {
        let plugboard = Plugboard::new(vec![]);
        assert_eq!(plugboard['A'], b'A' - 65);
    }

    #[test]
    fn single_connection() {
        let plugboard = Plugboard::new(vec![('A', 'B')]);
        assert_eq!(plugboard['A'], b'B' - 65);
        assert_eq!(plugboard['B'], b'A' - 65);
        assert_eq!(plugboard['C'], b'C' - 65);
    }

    #[test]
    fn multiple_connections() {
        let plugboard = Plugboard::new(vec![('A', 'B'), ('C', 'D')]);
        assert_eq!(plugboard['A'], b'B' - 65);
        assert_eq!(plugboard['B'], b'A' - 65);
        assert_eq!(plugboard['C'], b'D' - 65);
        assert_eq!(plugboard['E'], b'E' - 65);
    }
}
