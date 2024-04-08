use crate::*;
use std::io::{Read, Write};
use termion::raw::RawTerminal;

pub struct Chip {
    pub ram: [u8; MEMORY_SIZE],
    pub program_counter: usize,
    pub index_register: usize,
    pub stack: Vec<usize>, // LIFO
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub var_reg: [u8; 16],
    pub display_buffer: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

impl Chip {
    pub fn new() -> Self {
        let mut chip = Chip {
            ram: [0; MEMORY_SIZE],
            program_counter: 512, // 512th position
            index_register: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            var_reg: [0; 16], // Variable registers
            display_buffer: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        };
        chip.set_memory_at_position(0x50, &FONT_DATA);
        chip
    }
    pub fn increment_counter(&mut self, n: usize) {
        self.program_counter += n;
    }
    pub fn fetch(&mut self) -> u16 {
        let result = combine_u8_to_u16(
            self.ram[self.program_counter],
            self.ram[self.program_counter + 1],
        );
        self.increment_counter(2);
        result
    }
    pub fn terminal_display(&mut self, instruction: u16) {
        clear_terminal();
        let vx = self.read_var_reg_x(instruction);
        let vy = self.read_var_reg_y(instruction);
        let n: u8 = n(instruction);
        let x: u8 = vx % DISPLAY_WIDTH as u8;
        let y: u8 = vy % DISPLAY_HEIGHT as u8;
        self.var_reg[15] = 0;

        for row_n in 0..n {
            let y_coordinate = (y + row_n) as usize;
            if y_coordinate >= DISPLAY_WIDTH {
                break;
            }
            let n_th_byte = self.ram[self.index_register + row_n as usize];
            for bit in 0..8u8 {
                let value = n_th_byte >> (7 - bit) & 0x01;
                let x_coordinate = (x + bit) as usize;
                if x_coordinate >= DISPLAY_WIDTH {
                    break;
                }
                let render_value = self.display_buffer[y_coordinate][x_coordinate];
                if render_value && value == 0x01 {
                    self.display_buffer[y_coordinate][x_coordinate] = false;
                    self.var_reg[15] = 0;
                } else if value == 0x01 && !render_value {
                    self.display_buffer[y_coordinate][x_coordinate] = true;
                } else {
                }
            }
        }

        for _y in 0..DISPLAY_HEIGHT {
            for _x in 0..DISPLAY_WIDTH {
                if self.display_buffer[_y][_x] {
                    print!("⬜");
                } else {
                    print!("⬛");
                }
            }
            println!("");
        }
    }

    pub fn termion_display<W: Write + std::os::fd::AsFd>(
        &mut self,
        stdout: &mut RawTerminal<W>,
        instruction: u16,
    ) {
        let vx = self.read_var_reg_x(instruction);
        let vy = self.read_var_reg_y(instruction);
        let n: u8 = n(instruction);
        let x: u8 = vx % DISPLAY_WIDTH as u8;
        let y: u8 = vy % DISPLAY_HEIGHT as u8;
        self.var_reg[15] = 0;
        write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();

        for row_n in 0..n {
            let y_coordinate = (y + row_n) as usize;
            if y_coordinate >= DISPLAY_WIDTH {
                break;
            }
            let n_th_byte = self.ram[self.index_register + row_n as usize];
            for bit in 0..8u8 {
                let value = n_th_byte >> (7 - bit) & 0x01;
                let x_coordinate = (x + bit) as usize;
                if x_coordinate >= DISPLAY_WIDTH {
                    break; // No wrapping.
                }
                let render_value = self.display_buffer[y_coordinate][x_coordinate];
                if render_value && value == 0x01 {
                    self.display_buffer[y_coordinate][x_coordinate] = false;
                    write!(
                        stdout,
                        "{}",
                        termion::cursor::Goto(1 + x_coordinate as u16, 1 + y_coordinate as u16)
                    )
                    .unwrap();
                    stdout.write(b" ").unwrap();
                    self.var_reg[15] = 0;
                } else if value == 0x01 && !render_value {
                    self.display_buffer[y_coordinate][x_coordinate] = true;
                    write!(
                        stdout,
                        "{}",
                        termion::cursor::Goto(1 + x_coordinate as u16, 1 + y_coordinate as u16)
                    )
                    .unwrap();
                    stdout.write(b"x").unwrap();
                }
            }
        }

        // for _y in 0..DISPLAY_HEIGHT {
        //     for _x in 0..DISPLAY_WIDTH {
        //         if self.display_buffer[_y][_x] {
        //             stdout.write(b"x").unwrap();
        //             // print!("⬜");
        //         } else {
        //             stdout.write(b" ").unwrap();
        //             // print!("⬛");
        //         }
        //     }
        //     // write!(stdout, "{}", termion::clear::CurrentLine).unwrap();

        //     // write!(stdout, "{}", termion::clear::CurrentLine).unwrap();
        // }
        write!(stdout, "{}", termion::cursor::Goto(1, 2 + DISPLAY_HEIGHT as u16)).unwrap();

        stdout.flush().unwrap();
    }
    pub fn read_variable_register(&self, char: char) -> u8 {
        let idx = char_to_hex_number(char).unwrap();
        self.var_reg[idx as usize]
    }
    pub fn read_var_reg_x(&self, value: u16) -> u8 {
        let idx = x(value) as usize;
        self.var_reg[idx]
    }
    pub fn read_var_reg_y(&self, value: u16) -> u8 {
        let idx = y(value) as usize;
        self.var_reg[idx]
    }
    pub fn set_carry(&mut self, res: u8) {
        self.var_reg[15] = res;
    }
    pub fn get_carry(&mut self) -> u8 {
        self.var_reg[15]
    }
    pub fn pretty_print_progress(&self, rows: usize) {
        clear_terminal();
        if self.program_counter > 10 && self.program_counter < 4080 {
            for i in 0..rows * 2 {
                if i % 2 != 0 {
                    continue;
                }
                let pc = self.program_counter + i - rows;
                let s = format!(
                    "[{:03X?}, {:03X?}] -> {:02X?}{:02X?}",
                    pc,
                    pc + 1,
                    self.ram[pc],
                    self.ram[pc + 1]
                );

                if i == rows {
                    println!(" * {}", s);
                } else {
                    println!("   {}", s);
                }
            }
        }
    }
    pub fn set_variable_register(&mut self, char: char, value: u8) {
        let idx = char_to_hex_number(char).unwrap();
        self.var_reg[idx as usize] = value;
    }
    pub fn add_to_variable_register(&mut self, char: char, value: u8) {
        let idx = char_to_hex_number(char).unwrap() as usize;
        self.var_reg[idx] = match self.var_reg[idx].checked_add(value) {
            Some(val) => val,
            None => 255,
        };
    }
    pub fn next(&mut self) -> u8 {
        let result = self.ram[self.program_counter];
        self.increment_counter(1);
        result
    }
    pub fn load_rom(&mut self, bytes: &Vec<u8>) {
        self.set_memory_at_position(512, &bytes)
    }
    pub fn set_memory_at_position(&mut self, idx: usize, bytes: &[u8]) {
        for (i, &byte) in bytes.iter().enumerate() {
            self.ram[idx + i] = byte;
        }
    }
    pub fn print_ram(&self, columns: usize) {
        println!("DUMPED RAM:");
        for i in 0..MEMORY_SIZE {
            print!("{:03X} {:02X?} |", i, self.ram[i]);
            if i != 0 && i % columns == 0 {
                println!();
            }
        }
        println!();
    }
}
