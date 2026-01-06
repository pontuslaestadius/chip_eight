use crate::{nibble::Nibble, nibbles::Nibbles};

#[derive(Debug)]
pub enum Opcode {
    CLS,
    RET,

    JP { addr: u16 },
    CALL { addr: u16 },

    SEByte { x: Nibble, byte: u8 },
    SNEByte { x: Nibble, byte: u8 },
    SEReg { x: Nibble, y: Nibble },

    LDByte { x: Nibble, byte: u8 },
    ADDByte { x: Nibble, byte: u8 },

    LDReg { x: Nibble, y: Nibble },
    OR { x: Nibble, y: Nibble },
    AND { x: Nibble, y: Nibble },
    XOR { x: Nibble, y: Nibble },
    ADD { x: Nibble, y: Nibble },
    SUB { x: Nibble, y: Nibble },
    SHR { x: Nibble },
    SUBN { x: Nibble, y: Nibble },
    SHL { x: Nibble },

    SNEReg { x: Nibble, y: Nibble },

    LDI { addr: u16 },
    JPPlusV0 { addr: u16 },
    RND { x: Nibble, byte: u8 },
    DRW { x: Nibble, y: Nibble, n: Nibble },

    SKP { x: Nibble },
    SKNP { x: Nibble },

    LDxDT { x: Nibble },
    LDxK { x: Nibble },
    LDdtX { x: Nibble },
    LDstX { x: Nibble },
    ADDI { x: Nibble },
    LDF { x: Nibble },
    LDB { x: Nibble },
    LDIStore { x: Nibble },
    LDIRead { x: Nibble },
}

impl Opcode {
    pub fn decode(raw: u16) -> Self {
        let n = Nibbles::from_u16(raw);

        match (
            n.first.as_u8(),
            n.second.as_u8(),
            n.third.as_u8(),
            n.fourth.as_u8(),
        ) {
            (0x0, 0x0, 0xE, 0x0) => Opcode::CLS,
            (0x0, 0x0, 0xE, 0xE) => Opcode::RET,

            (0x1, _, _, _) => Opcode::JP { addr: raw & 0x0FFF },
            (0x2, _, _, _) => Opcode::CALL { addr: raw & 0x0FFF },

            (0x3, x, _, _) => Opcode::SEByte {
                x: Nibble::from_low(x),
                byte: (raw & 0xFF) as u8,
            },
            (0x4, x, _, _) => Opcode::SNEByte {
                x: Nibble::from_low(x),
                byte: (raw & 0xFF) as u8,
            },
            (0x5, x, y, 0x0) => Opcode::SEReg {
                x: Nibble::from_low(x),
                y: Nibble::from_low(y),
            },

            (0x6, x, _, _) => Opcode::LDByte {
                x: Nibble::from_low(x),
                byte: (raw & 0xFF) as u8,
            },
            (0x7, x, _, _) => Opcode::ADDByte {
                x: Nibble::from_low(x),
                byte: (raw & 0xFF) as u8,
            },

            (0x8, x, y, 0x0) => Opcode::LDReg {
                x: Nibble::from_low(x),
                y: Nibble::from_low(y),
            },
            (0x8, x, y, 0x1) => Opcode::OR {
                x: Nibble::from_low(x),
                y: Nibble::from_low(y),
            },
            (0x8, x, y, 0x2) => Opcode::AND {
                x: Nibble::from_low(x),
                y: Nibble::from_low(y),
            },
            (0x8, x, y, 0x3) => Opcode::XOR {
                x: Nibble::from_low(x),
                y: Nibble::from_low(y),
            },
            (0x8, x, y, 0x4) => Opcode::ADD {
                x: Nibble::from_low(x),
                y: Nibble::from_low(y),
            },
            (0x8, x, y, 0x5) => Opcode::SUB {
                x: Nibble::from_low(x),
                y: Nibble::from_low(y),
            },
            (0x8, x, _, 0x6) => Opcode::SHR {
                x: Nibble::from_low(x),
            },
            (0x8, x, y, 0x7) => Opcode::SUBN {
                x: Nibble::from_low(x),
                y: Nibble::from_low(y),
            },
            (0x8, x, _, 0xE) => Opcode::SHL {
                x: Nibble::from_low(x),
            },

            (0x9, x, y, 0x0) => Opcode::SNEReg {
                x: Nibble::from_low(x),
                y: Nibble::from_low(y),
            },

            (0xA, _, _, _) => Opcode::LDI { addr: raw & 0x0FFF },
            (0xB, _, _, _) => Opcode::JPPlusV0 { addr: raw & 0x0FFF },
            (0xC, x, _, _) => Opcode::RND {
                x: Nibble::from_low(x),
                byte: (raw & 0xFF) as u8,
            },
            (0xD, x, y, n) => Opcode::DRW {
                x: Nibble::from_low(x),
                y: Nibble::from_low(y),
                n: Nibble::from_low(n),
            },

            (0xE, x, 0x9, 0xE) => Opcode::SKP {
                x: Nibble::from_low(x),
            },
            (0xE, x, 0xA, 0x1) => Opcode::SKNP {
                x: Nibble::from_low(x),
            },

            (0xF, x, 0x0, 0x7) => Opcode::LDxDT {
                x: Nibble::from_low(x),
            },
            (0xF, x, 0x0, 0xA) => Opcode::LDxK {
                x: Nibble::from_low(x),
            },
            (0xF, x, 0x1, 0x5) => Opcode::LDdtX {
                x: Nibble::from_low(x),
            },
            (0xF, x, 0x1, 0x8) => Opcode::LDstX {
                x: Nibble::from_low(x),
            },
            (0xF, x, 0x1, 0xE) => Opcode::ADDI {
                x: Nibble::from_low(x),
            },
            (0xF, x, 0x2, 0x9) => Opcode::LDF {
                x: Nibble::from_low(x),
            },
            (0xF, x, 0x3, 0x3) => Opcode::LDB {
                x: Nibble::from_low(x),
            },
            (0xF, x, 0x5, 0x5) => Opcode::LDIStore {
                x: Nibble::from_low(x),
            },
            (0xF, x, 0x6, 0x5) => Opcode::LDIRead {
                x: Nibble::from_low(x),
            },

            _ => panic!("Invalid opcode {:04X}", raw),
        }
    }
}
