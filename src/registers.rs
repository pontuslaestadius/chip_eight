use crate::*;

pub struct Registers {
    v: [u8; 16], // V0–VF
    i: u16,      // Index register
}

impl Registers {
    pub fn new() -> Self {
        Registers { v: [0; 16], i: 0 }
    }

    /// Get Vx
    pub fn get(&self, x: Nibble) -> u8 {
        self.v[x.as_usize()]
    }

    /// Set Vx
    pub fn set(&mut self, x: Nibble, value: u8) {
        self.v[x.as_usize()] = value;
    }

    /// Add to Vx with overflow flag in VF
    pub fn add(&mut self, x: Nibble, value: u8) {
        let (res, carry) = self.v[x.as_usize()].overflowing_add(value);
        self.v[x.as_usize()] = res;
        self.v[0xF] = carry as u8;
    }

    /// Set VF (carry flag)
    pub fn set_carry(&mut self, value: u8) {
        self.v[0xF] = value;
    }

    /// Get VF (carry flag)
    pub fn get_carry(&self) -> u8 {
        self.v[0xF]
    }

    /// Index register access
    pub fn set_i(&mut self, value: u16) {
        self.i = value;
    }
    pub fn get_i(&self) -> u16 {
        self.i
    }

    pub fn get_vx(&self, instruction: u16) -> u8 {
        self.v[x(instruction) as usize]
    }

    pub fn get_vy(&self, instruction: u16) -> u8 {
        self.v[Nibble::from_opcode(instruction, 4).as_usize()]
    }

    pub fn set_vx(&mut self, instruction: u16, value: u8) {
        let idx = x(instruction) as usize;
        self.v[idx] = value;
    }

    pub fn add_vx(&mut self, instruction: u16, value: u8) {
        let idx = x(instruction) as usize;
        let (res, carry) = self.v[idx].overflowing_add(value);
        self.v[idx] = res;
        self.v[0xF] = carry as u8;
    }
}
