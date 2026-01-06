use crate::nibble::Nibble;

pub struct Nibbles {
    pub first: Nibble,
    pub second: Nibble,
    pub third: Nibble,
    pub fourth: Nibble,
}

impl Nibbles {
    pub fn from_u16(value: u16) -> Self {
        Nibbles {
            first: Nibble::from_opcode(value, 12),
            second: Nibble::from_opcode(value, 8),
            third: Nibble::from_opcode(value, 4),
            fourth: Nibble::from_opcode(value, 0),
        }
    }
}
