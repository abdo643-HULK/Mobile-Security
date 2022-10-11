use std::{cell::Cell, char, fmt::Debug};

#[cfg(feature = "debug")]
use crate::enigma::ToChar;

const ROTOR_TABLES: &[[char; 26]; 3] = &[
    const_str::to_char_array!("JGDQOXUSCAMIFRVTPNEWKBLZYH"),
    const_str::to_char_array!("NTZPSFBOKMWRCJDIVLAEYUXHGQ"),
    const_str::to_char_array!("JVIUBHTCDYAKEQZPOSGXNRMWFL"),
];

const ROTOR_TABLES_BYTES: [[u8; 26]; 3] = array_macro::array![i => {
    let table = ROTOR_TABLES[i];
    array_macro::array![i => table[i] as u8 - 65; 26]
};3];

pub struct RotorTable(usize);

impl RotorTable {
    fn get(&self) -> [u8; 26] {
        // ROTOR_TABLES[self.0].map(|x| x.index() as u8)
        ROTOR_TABLES_BYTES[self.0]
    }

    // fn get(&self) -> [char; 26] {
    //     ROTOR_TABLES[self.0]
    // }
}

pub enum Wiring {
    Forward(u8),
    Reverse(u8),
}

#[derive(Debug, Clone)]
pub struct Rotor {
    name: String,
    notch: Cell<bool>,
    offset: Cell<u8>,
    // inner_offset: Cell<usize>,
    ring_setting: u8,
    pub wiring_table: [u8; 26],
    inverse_wiring_table: [u8; 26],
    // wiring_table: [char; 26],
    // inverse_wiring_table: [char; 26],
}

/// REGION: Constants

impl Rotor {
    /// REGION: Private

    const NOTCH: u8 = 7;

    /// REGION: Public

    pub const I: RotorTable = RotorTable(0);
    pub const II: RotorTable = RotorTable(1);
    pub const III: RotorTable = RotorTable(2);
}

/// REGION: Constructor

impl Rotor {
    pub fn new(table: RotorTable, ring_setting: u8) -> Self {
        let mut wiring_table = table.get();
        wiring_table.rotate_left(ring_setting as usize);

        let mut inverse_wiring_table = core::array::from_fn(|i| i as u8); // should get optimized to a static array
        inverse_wiring_table.rotate_left(ring_setting as usize);

        Self {
            name: format!("Rotor {}", table.0 + 1),
            ring_setting,
            wiring_table,
            inverse_wiring_table,
            offset: Cell::new(0),
            notch: Cell::new(false),
            // inner_offset: Cell::new(0),
        }
    }
}

/// REGION: Public Methods

impl Rotor {
    #[inline]
    pub fn step(&mut self) {
        println!("{}: {}", self.name, self.offset.get());
        let offset = self.offset.get() + 1;
        self.notch.set(offset == Self::NOTCH);
        self.offset.set(offset % Self::NOTCH);
        self.rotate()
    }

    #[inline]
    pub fn notch(&self) -> bool {
        // let notch = self.offset.get() == Self::NOTCH;
        self.notch.get()
    }

    pub fn wire(&self, wiring: Wiring) -> u8 {
        let (key, table, inverse_table) = match wiring {
            Wiring::Forward(key) => (key, self.wiring_table, self.inverse_wiring_table),
            Wiring::Reverse(key) => (key, self.inverse_wiring_table, self.wiring_table),
        };

        #[cfg(feature = "debug")]
        println!(
            "{:?}",
            table.map(|code| crate::enigma::ToChar::to_char(&code))
        );

        let wired = table[key as usize];
        let code = inverse_table
            .iter()
            .position(|&c| c == wired)
            .expect("received incorrect value in Rotor");
        // let char_code = (wired
        //     .index()
        //     .checked_sub(self.offset.get() as usize)
        //     .unwrap_or(0)
        //     + 65) as u8;
        // char_code
        code as u8
    }
}

/// REGION: Private Methods

impl Rotor {
    #[inline]
    fn rotate(&mut self) {
        self.wiring_table.rotate_left(1);
        self.inverse_wiring_table.rotate_left(1);
    }
}
