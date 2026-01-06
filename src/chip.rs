use crate::controls::{Chip8Key, Keypad};
use crate::display::display_trait::Ch8Display;
use crate::memory::Memory;
use crate::registers::Registers;
use crate::stack::Stack;
use crate::timers::Timers;
use crate::*;

pub struct Chip {
    pub memory: Memory,
    pub program_counter: usize,
    pub stack: Stack, // LIFO
    pub timers: Timers,
    pub registers: Registers,
    pub display: Box<dyn Ch8Display>,
    pub keypad: Keypad,
    pub wait_for_input: Option<u8>,
}

const FONT_START: usize = 0x50;

impl Chip {
    pub fn new(display: impl Ch8Display + 'static) -> Self {
        let mut chip = Chip {
            memory: Memory::new(),
            program_counter: 512, // 512th position
            stack: Stack::new(),
            timers: Timers::new(),
            registers: Registers::new(),
            display: Box::new(display),
            keypad: Keypad::new(),
            wait_for_input: None,
        };
        chip.set_memory_at_position(FONT_START, &FONT_DATA);
        chip
    }

    // EX9E — skip if key in Vx is pressed
    pub fn skip_if_pressed(&mut self, vx: u8) {
        self.skip_if(
            Chip8Key::new(vx)
                .map(|k| self.keypad.is_pressed(k))
                .unwrap_or(false),
        )
    }

    pub fn skip_if_not_pressed(&mut self, vx: u8) {
        self.skip_if(
            Chip8Key::new(vx)
                .map(|k| !self.keypad.is_pressed(k))
                .unwrap_or(false),
        )
    }

    pub fn try_press(&mut self, ch: char) -> Option<Chip8Key> {
        if let Some(key) = self.keypad.lookup(ch) {
            // println!("Pressed '{}' → CHIP-8 key {:X}", ch, key.as_u8());
            self.keypad.clear();
            self.keypad.press(key);
            return Some(key);
        }
        None
    }

    // FX0A — wait for key press (non-blocking)
    pub fn wait_for_key(&mut self) -> Option<u8> {
        self.keypad.take_last_pressed().map(|k| k.as_u8())
    }

    pub fn opcode_dxyn(&mut self, x: Nibble, y: Nibble, n: Nibble) {
        let vx = self.registers.get(x);
        let vy = self.registers.get(y);

        let i = self.registers.get_i() as usize;
        let sprite = &self.memory.slice(i, i + n.as_usize());

        let collision = self.display.draw_sprite(vx, vy, sprite);
        self.display.render();

        self.registers.set(Nibble::from_low(0xF), collision as u8);
    }

    pub fn increment_counter(&mut self, n: usize) {
        self.program_counter += n;
    }
    pub fn read_var_reg_x(&self, value: u16) -> u8 {
        let idx = Nibble::from_opcode(value, 8);
        self.registers.get(idx)
    }
    pub fn read_var_reg_y(&self, value: u16) -> u8 {
        let idx = Nibble::from_opcode(value, 4);
        self.registers.get(idx)
    }
    pub fn next(&mut self) -> u8 {
        let result = self.memory.read(self.program_counter);
        self.increment_counter(1);
        result
    }
    pub fn next_u16(&mut self) -> u16 {
        let result = self.memory.read_u16(self.program_counter);
        self.increment_counter(2);
        result
    }

    pub fn load_rom(&mut self, bytes: &[u8]) {
        self.set_memory_at_position(512, bytes)
    }
    pub fn set_memory_at_position(&mut self, idx: usize, bytes: &[u8]) {
        for (i, &byte) in bytes.iter().enumerate() {
            self.memory.write(idx + i, byte);
        }
    }
    pub fn skip_if(&mut self, condition: bool) {
        if condition {
            self.increment_counter(2);
        }
    }
}
