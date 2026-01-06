use crate::{chip::Chip, font::FONT_START, nibble::Nibble, opcode::Opcode};

impl Chip {
    pub fn execute(&mut self, opcode: Opcode) {
        match opcode {
            // ──────────────────────────────────────────
            // System
            // ──────────────────────────────────────────
            Opcode::CLS => {
                self.display.clear();
            }

            Opcode::RET => {
                self.program_counter = self.stack.pop() as usize;
            }

            // ──────────────────────────────────────────
            // Flow control
            // ──────────────────────────────────────────
            Opcode::JP { addr } => {
                self.program_counter = addr as usize;
            }

            Opcode::JPPlusV0 { addr } => {
                let v0 = self.registers.get(Nibble::from_low(0));
                self.program_counter = addr as usize + v0 as usize;
            }

            Opcode::CALL { addr } => {
                self.stack.push(self.program_counter as u16);
                self.program_counter = addr as usize;
            }

            // ──────────────────────────────────────────
            // Conditional skips
            // ──────────────────────────────────────────
            Opcode::SEByte { x, byte } => {
                let vx = self.registers.get(x);
                self.skip_if(vx == byte);
            }

            Opcode::SNEByte { x, byte } => {
                let vx = self.registers.get(x);
                self.skip_if(vx != byte);
            }

            Opcode::SEReg { x, y } => {
                let vx = self.registers.get(x);
                let vy = self.registers.get(y);
                self.skip_if(vx == vy);
            }

            Opcode::SNEReg { x, y } => {
                let vx = self.registers.get(x);
                let vy = self.registers.get(y);
                self.skip_if(vx != vy);
            }

            // ──────────────────────────────────────────
            // Register ops
            // ──────────────────────────────────────────
            Opcode::LDByte { x, byte } => {
                self.registers.set(x, byte);
            }

            Opcode::ADDByte { x, byte } => {
                self.registers.add(x, byte);
            }

            Opcode::LDReg { x, y } => {
                let vy = self.registers.get(y);
                self.registers.set(x, vy);
            }

            Opcode::OR { x, y } => {
                let v = self.registers.get(x) | self.registers.get(y);
                self.registers.set(x, v);
            }

            Opcode::AND { x, y } => {
                let v = self.registers.get(x) & self.registers.get(y);
                self.registers.set(x, v);
            }

            Opcode::XOR { x, y } => {
                let v = self.registers.get(x) ^ self.registers.get(y);
                self.registers.set(x, v);
            }

            Opcode::ADD { x, y } => {
                let (res, carry) = self.registers.get(x).overflowing_add(self.registers.get(y));
                self.registers.set(x, res);
                self.registers.set_carry(carry as u8);
            }

            Opcode::SUB { x, y } => {
                let (res, borrow) = self.registers.get(x).overflowing_sub(self.registers.get(y));
                self.registers.set(x, res);
                self.registers.set_carry((!borrow) as u8);
            }

            Opcode::SUBN { x, y } => {
                let (res, borrow) = self.registers.get(y).overflowing_sub(self.registers.get(x));
                self.registers.set(x, res);
                self.registers.set_carry((!borrow) as u8);
            }

            Opcode::SHR { x } => {
                let vx = self.registers.get(x);
                self.registers.set_carry(vx & 0x01);
                self.registers.set(x, vx >> 1);
            }

            Opcode::SHL { x } => {
                let vx = self.registers.get(x);
                self.registers.set_carry((vx & 0x80) >> 7);
                self.registers.set(x, vx << 1);
            }

            // ──────────────────────────────────────────
            // Index / memory
            // ──────────────────────────────────────────
            Opcode::LDI { addr } => {
                self.registers.set_i(addr);
            }

            Opcode::ADDI { x } => {
                let i = self.registers.get_i();
                let vx = self.registers.get(x) as u16;
                self.registers.set_i(i.saturating_add(vx));
            }

            Opcode::LDF { x } => {
                let digit = self.registers.get(x) as u16;
                self.registers.set_i(FONT_START as u16 + digit * 5);
            }

            Opcode::LDB { x } => {
                let value = self.registers.get(x);
                let i = self.registers.get_i() as usize;

                self.memory.write(i, value / 100);
                self.memory.write(i + 1, (value / 10) % 10);
                self.memory.write(i + 2, value % 10);
            }

            Opcode::LDIStore { x } => {
                let i = self.registers.get_i() as usize;
                for idx in 0..=x.as_u8() {
                    let n = Nibble::from_low(idx);
                    self.memory.write(i + n.as_usize(), self.registers.get(n));
                }
            }

            Opcode::LDIRead { x } => {
                let i = self.registers.get_i() as usize;
                for idx in 0..=x.as_u8() {
                    let n = Nibble::from_low(idx);
                    let v = self.memory.read(i + n.as_usize());
                    self.registers.set(n, v);
                }
            }

            // ──────────────────────────────────────────
            // Random & drawing
            // ──────────────────────────────────────────
            Opcode::RND { x, byte } => {
                let r: u8 = rand::random();
                self.registers.set(x, r & byte);
            }

            Opcode::DRW { x, y, n } => {
                self.opcode_dxyn(x, y, n);
            }

            // ──────────────────────────────────────────
            // Timers & input
            // ──────────────────────────────────────────
            Opcode::LDxDT { x } => {
                let dt = self.timers.get_delay();
                self.registers.set(x, dt);
            }

            Opcode::LDdtX { x } => {
                self.timers.set_delay(self.registers.get(x));
            }

            Opcode::LDstX { x } => {
                self.timers.set_sound(self.registers.get(x));
            }

            Opcode::SKP { x } => {
                let key = self.registers.get(x);
                self.skip_if_pressed(key);
            }

            Opcode::SKNP { x } => {
                let key = self.registers.get(x);
                self.skip_if_not_pressed(key);
            }

            Opcode::LDxK { x } => {
                self.wait_for_input = Some(x.as_u8());
            }
        }
    }
}
