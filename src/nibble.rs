#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Nibble(u8);

impl Nibble {
    pub fn from_low(n: u8) -> Self {
        debug_assert!(n < 16);
        Nibble(n)
    }
}

impl Nibble {
    pub fn from_opcode(opcode: u16, shift: u8) -> Self {
        Nibble::from_low(((opcode >> shift) & 0xF) as u8)
    }
}

impl Nibble {
    pub fn as_u8(self) -> u8 {
        self.0
    }

    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}
