extern crate rand;
use std::env;
use std::fs;
use std::thread;
use std::time::Duration;

mod chip;
extern crate termion;

use std::io::{stdout, Read, Write};
use termion::async_stdin;
use termion::raw::IntoRawMode;

const FONT_DATA: [u8; 80] = [
    0xF0, 0x90, // Comments to force formatter to
    0x90, 0x90, // align font data as two bytes,
    0xF0, 0x20, // or more descriptively, each
    0x60, 0x20, // row would be one fetch cycle.
    0x20, 0x70, //
    0xF0, 0x10, //
    0xF0, 0x80, //
    0xF0, 0xF0, //
    0x10, 0xF0, //
    0x10, 0xF0, //
    0x90, 0x90, //
    0xF0, 0x10, //
    0x10, 0xF0, //
    0x80, 0xF0, //
    0x10, 0xF0, //
    0xF0, 0x80, //
    0xF0, 0x90, //
    0xF0, 0xF0, //
    0x10, 0x20, //
    0x40, 0x40, //
    0xF0, 0x90, //
    0xF0, 0x90, //
    0xF0, 0xF0, //
    0x90, 0xF0, //
    0x10, 0xF0, //
    0xF0, 0x90, //
    0xF0, 0x90, //
    0x90, 0xE0, //
    0x90, 0xE0, //
    0x90, 0xE0, //
    0xF0, 0x80, //
    0x80, 0x80, //
    0xF0, 0xE0, //
    0x90, 0x90, //
    0x90, 0xE0, //
    0xF0, 0x80, //
    0xF0, 0x80, //
    0xF0, 0xF0, //
    0x80, 0xF0, //
    0x80, 0x80, //
];

const MEMORY_SIZE: usize = 4096;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_WIDTH: usize = 64;

// Due to how inputs are captured, this is the only way to exit the game,
// without external interrupt or kill commands.
const EXIT_GAME_KEY: u8 = b't';

enum PrintMode {
    All,
    Code,
    None,
}

const PRINT_CODE: PrintMode = PrintMode::None;

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let mut wait_for_input: Option<usize> = None;

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    let mut chip = chip::Chip::new();
    let args: Vec<String> = env::args().collect();
    let content = fs::read(&args[1]).unwrap();

    let conditional_print = |code: &str, desc: &str| match PRINT_CODE {
        PrintMode::All => {
            println!("{}: {}", code, desc);
        }
        PrintMode::Code => {
            println!("{}", code);
        }
        _ => {}
    };
    chip.load_rom(&content);
    // chip.print_ram(8);
    while chip.program_counter < 4096 {
        thread::sleep(Duration::from_millis(17)); // should be like 17ms in prod.

        // FIXME: Handle keypad. (1-4...Z-V)
        let key_pressed: Option<u8> = match stdin.next() {
            Some(res) => match res.ok() {
                Some(key) => {
                    if key == EXIT_GAME_KEY {
                        write!(
                            stdout,
                            "{}{}",
                            termion::clear::All,
                            termion::cursor::Goto(1, 1)
                        )
                        .unwrap();
                        panic!()
                    } else {
                        // TODO: should use scancodes and not string literals.
                        let decoded_value: Option<u8> = match key as char {
                            '1' => Some(0x01),
                            '2' => Some(0x02),
                            '3' => Some(0x03),
                            '4' => Some(0x0C),
                            'q' => Some(0x04),
                            'w' => Some(0x05),
                            'e' => Some(0x06),
                            'r' => Some(0x0D),
                            'a' => Some(0x07),
                            's' => Some(0x08),
                            'd' => Some(0x09),
                            'f' => Some(0x0E),
                            'z' => Some(0x0A),
                            'x' => Some(0x00),
                            'c' => Some(0x0B),
                            'v' => Some(0x0F),
                            _ => None,
                        };
                        decoded_value
                        // println!("{} -> {}", key, key as u8);
                        // Some(key as u8)
                    }
                }
                None => None,
            },
            None => None,
        };
        if chip.delay_timer > 0 {
            chip.delay_timer -= 1;
        }
        if let Some(vx) = wait_for_input {
            write!(
                stdout,
                "{}",
                termion::cursor::Goto(1, 2 + DISPLAY_HEIGHT as u16)
            )
            .unwrap();
            if let Some(key) = key_pressed {
                write!(stdout, "{}", termion::clear::CurrentLine).unwrap();
                chip.var_reg[vx] = key;
                wait_for_input = None;
            } else {
                write!(stdout, "Waiting...").unwrap();
                continue;
            }
            stdout.flush().unwrap();
        }
        let byte1 = chip.next();
        let byte2: u8 = chip.next();
        let instruction = u8_pair_to_hex(byte1, byte2);
        let instruction_u16: u16 = combine_u8_to_u16(byte1, byte2);
        //chip.pretty_print_progress(20);
        let first_nibble = ((instruction_u16 & 0b1111000000000000) >> 12) as u8;
        // let third_nibble = ((instruction_u16 & 0b0000000011110000) >> 4) as u8;
        let fourth_nibble = (instruction_u16 & 0b0000000000001111) as u8;

        // write!(
        //     stdout,
        //     "{}",
        //     termion::cursor::Goto(1, 2 + DISPLAY_HEIGHT as u16)
        // )
        // .unwrap();
        // write!(stdout, "{:00X}", instruction_u16).unwrap();
        // stdout.flush().unwrap();

        // if first_nibble != 0 {
        // println!(
        //     "PC: {:03X?}, inst: {:04X?}",
        //     chip.program_counter - 2,
        //     instruction_u16
        // );
        // }

        match first_nibble {
            0x00 => match instruction[1..4] {
                ['0', 'E', '0'] => {
                    conditional_print("00E0", "Clear display");
                    chip.display_buffer = [[false; 64]; 32];
                }
                ['0', 'E', 'E'] => {
                    conditional_print("00EE", "Subroutine");

                    match chip.stack.pop() {
                        Some(value) => {
                            chip.program_counter = value;
                        }
                        None => {
                            panic!("ERROR: tried popping stack when empty");
                        }
                    }
                }
                _ => {
                    // Ignore even logging it, as it's internal codes not used.
                }
            },
            0x01 => {
                conditional_print("1NNN", "Jump");
                chip.program_counter = nnn(instruction_u16);
            }
            0x02 => {
                conditional_print("2NNN", "Subroutine");
                chip.stack.push(chip.program_counter);
                chip.program_counter = nnn(instruction_u16);
            }
            0x03 => {
                conditional_print("3XNN", "Skip conditionally");
                if chip.var_reg[x(instruction_u16) as usize] == nn(instruction_u16) {
                    chip.program_counter += 2;
                }
                // 3XNN will skip one instruction if the value in VX is equal to NN, and 4XNN will skip if they are not equal.
            }
            0x04 => {
                conditional_print("4XNN", "Skip conditionally");
                if chip.var_reg[x(instruction_u16) as usize] != nn(instruction_u16) {
                    chip.program_counter += 2;
                }
            }
            0x05 => {
                conditional_print("5XY0", "Skip conditionally");
                if chip.var_reg[x(instruction_u16) as usize]
                    == chip.var_reg[y(instruction_u16) as usize]
                {
                    chip.program_counter += 2;
                }
            }
            0x06 => {
                conditional_print("6XNN", "Set");
                chip.var_reg[x(instruction_u16) as usize] = nn(instruction_u16);
            }
            0x07 => {
                conditional_print("7XNN", "Add");
                chip.add_to_variable_register(instruction[1], nn(instruction_u16));
            }
            0x08 => {
                let vx_reg: char = instruction[1];
                // let vy_reg = instruction[2];
                let vx = chip.var_reg[x(instruction_u16) as usize];
                let vy = chip.var_reg[y(instruction_u16) as usize];

                match fourth_nibble {
                    0x00 => {
                        conditional_print("8XY0", "Set"); // BROKEN!!!!
                        chip.set_variable_register(vx_reg, vy);
                    }
                    0x01 => {
                        conditional_print("8XY1", "Binary OR");
                        chip.set_variable_register(vx_reg, vx | vy);
                    }
                    0x02 => {
                        conditional_print("8XY2", "Binary AND");
                        chip.set_variable_register(vx_reg, vx & vy);
                    }
                    0x03 => {
                        conditional_print("8XY3", "Logical XOR");
                        chip.set_variable_register(vx_reg, vx ^ vy);
                    }
                    0x04 => {
                        conditional_print("8XY4", "Add");
                        match vx.checked_add(vy) {
                            Some(value) => {
                                chip.set_variable_register(vx_reg, value);
                                chip.set_carry(0)
                            }
                            None => {
                                chip.set_variable_register(vx_reg, 255);
                                chip.set_carry(1)
                            }
                        }
                    }
                    0x05 => {
                        // TODO handle carry flag.
                        conditional_print("8XY5", "Subtract");

                        // VX = VX - VY
                        // if chip.get_carry() {}
                        if vy > vx {
                            chip.set_carry(1);
                            chip.set_variable_register(vx_reg, 0);
                        } else {
                            chip.set_carry(0);
                            chip.set_variable_register(vx_reg, vx - vy);
                        }
                    }
                    // '6' => {
                    //     // Ambiguous instruction!
                    //     // Shift
                    // }
                    0x07 => {
                        // TODO handle carry flag.
                        conditional_print("8XY7", "Subtract");
                        if vy > vx {
                            chip.var_reg[x(instruction_u16) as usize] = 0

                        } else {
                            chip.var_reg[x(instruction_u16) as usize] -= vy;

                        }
                        // chip.set_variable_register(vx_reg, vy - vx);
                    }
                    0x06 => {
                        chip.set_variable_register(vx_reg, chip.read_var_reg_y(instruction_u16));
                        chip.set_variable_register(vx_reg, chip.read_var_reg_y(instruction_u16));
                        // Unsure if Carry applies here?
                        // if chip.var_reg[x(instruction_u16) as usize] & 0b10000000 == 0b10000000 {
                        //     chip.set_carry(1);
                        // }
                        chip.var_reg[x(instruction_u16) as usize] =
                            chip.var_reg[x(instruction_u16) as usize] >> 1;
                    }
                    0x0E => {
                        // COSMIC VIP variety.
                        chip.set_variable_register(vx_reg, chip.read_var_reg_y(instruction_u16));
                        chip.set_variable_register(vx_reg, chip.read_var_reg_y(instruction_u16));
                        if chip.var_reg[x(instruction_u16) as usize] & 0b10000000 == 0b10000000 {
                            chip.set_carry(1);
                        }
                        chip.var_reg[x(instruction_u16) as usize] =
                            chip.var_reg[x(instruction_u16) as usize] << 1;
                    }

                    _ => {
                        panic!(
                            "PC: {}, inst: {:04X?}",
                            chip.program_counter - 2,
                            instruction_u16
                        );
                    }
                }
            }
            0x09 => {
                conditional_print("9XY0", "Skip conditionally");

                if chip.var_reg[x(instruction_u16) as usize]
                    != chip.var_reg[y(instruction_u16) as usize]
                {
                    chip.program_counter += 2;
                }
            }
            0x0A => {
                conditional_print("ANNN", "Set index");
                chip.index_register = nnn(instruction_u16);
            }
            0x0B => {
                conditional_print("BNNN", "Jump with offset"); // Ambiguous instruction!
                chip.program_counter = nnn(instruction_u16) + chip.var_reg[0] as usize;
            }
            0x0C => {
                conditional_print("CXNN", "Random");
                let random_number: u8 = rand::random::<u8>();
                chip.var_reg[x(instruction_u16) as usize] = random_number & nn(instruction_u16)
            }
            0x0D => {
                conditional_print("DXYN", "Display");
                chip.termion_display(&mut stdout, instruction_u16);
            }
            0x0E => match instruction[2..4] {
                ['9', 'E'] => {
                    conditional_print("EXE9", "Skip if key");
                    if let Some(key) = key_pressed {
                        if key == chip.var_reg[x(instruction_u16) as usize] {
                            chip.program_counter += 2;
                        }
                    }
                }
                ['A', '1'] => {
                    conditional_print("EXA1", "Skip if key");
                    if let Some(key) = key_pressed {
                        if key != chip.var_reg[x(instruction_u16) as usize] {
                            chip.program_counter += 2;
                        }
                    }
                }
                _ => {
                    panic!(
                        "PC: {}, inst: {:04X?}",
                        chip.program_counter - 2,
                        instruction_u16
                    );
                }
            },
            0x0F => {
                let vx_reg = instruction[1];
                let vx = chip.read_variable_register(vx_reg);

                // match third_nibble {
                //     0x05 => {

                //     }
                //     _ => {
                //         panic!("PC: {}, inst: {:?}", chip.program_counter - 2, instruction);
                //     }
                // };

                match instruction[2..4] {
                    ['3', '3'] => {
                        conditional_print("FX33", "Binary-coded decimal conversion");
                        let val = x(instruction_u16) as usize;
                        let value = chip.var_reg[val as usize];
                        // for i in 1..=3 {
                        //     chip.ram[index_register + i] = value /
                        // }
                        // 123
                        // RAM: [N-4, 1, 2, 3, N]
                        chip.ram[chip.index_register] = (value / 100) & 0b00011111;
                        chip.ram[chip.index_register + 1] = (value / 10) & 0b00011111;
                        chip.ram[chip.index_register + 2] = value & 0b00011111;
                    }
                    ['6', '5'] => {
                        conditional_print("FX65", "Store and load memory");
                        let val = x(instruction_u16) as usize;
                        if val == 0 {
                            chip.var_reg[0] = chip.ram[chip.index_register];
                        }
                        for idx in 0..=val {
                            chip.var_reg[idx as usize] = chip.ram[chip.index_register + idx];
                        }
                    }
                    ['5', '5'] => {
                        conditional_print("FX55", "Store and load memory");
                        let val = x(instruction_u16) as usize;
                        if val == 0 {
                            chip.ram[chip.index_register] = chip.var_reg[0];
                            chip.var_reg[0] = 0;
                        }
                        for idx in 0..=val {
                            chip.ram[chip.index_register + idx] = chip.var_reg[idx as usize];
                            chip.var_reg[idx as usize] = 0;
                        }
                    }
                    ['0', '7'] => {
                        conditional_print("FX07", "Timers");
                        chip.var_reg[x(instruction_u16) as usize] = chip.delay_timer;
                    }
                    ['1', '5'] => {
                        conditional_print("FX15", "Timers");
                        chip.delay_timer = vx;
                    }
                    ['1', '8'] => {
                        conditional_print("FX18", "Timers");
                        chip.sound_timer = vx;
                    }
                    ['2', '9'] => {
                        conditional_print("FX29", "Font character");
                        chip.index_register = x(instruction_u16) as usize;
                    }
                    ['1', 'E'] => {
                        // Ambigious instruction, impl. may differ.
                        conditional_print("FX1E", "Add to index");
                        let value = chip.var_reg[x(instruction_u16) as usize];
                        if value > (0xFF - chip.index_register as u8) {
                            chip.index_register = 0xFF;
                        } else {
                            chip.index_register += value as usize;
                        }
                    }
                    ['0', 'A'] => {
                        // Bit ugly since it relies on global scope.
                        conditional_print("FX0A", "Get key");
                        wait_for_input = Some(x(instruction_u16) as usize);
                    }
                    _ => {
                        panic!(
                            "PC: {}, inst: {:04X?}",
                            chip.program_counter - 2,
                            instruction_u16
                        );
                    }
                }
            }
            _ => {
                panic!(
                    "PC: {}, inst: {:04X?}",
                    chip.program_counter - 2,
                    instruction_u16
                );
            }
        }

        // Execute instruction
    }
}

fn combine_u8_to_u16(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

fn u8_pair_to_hex(byte1: u8, byte2: u8) -> [char; 4] {
    let hex_chars = format!("{:02X}{:02X}", byte1, byte2);
    let mut result = ['0'; 4]; // Initialize array with default '0' characters
    let hex_chars_bytes = hex_chars.as_bytes();
    for (i, &byte) in hex_chars_bytes.iter().enumerate() {
        result[i] = byte as char;
    }
    result
}

fn char_to_hex_number(hex_char: char) -> Option<u8> {
    match hex_char.to_digit(16) {
        Some(digit) if digit < 16 => Some(digit as u8),
        _ => None,
    }
}

fn nnn(value: u16) -> usize {
    (value & 0b0000111111111111) as usize
}

fn nn(value: u16) -> u8 {
    (value & 0b0000000011111111) as u8
}
fn n(value: u16) -> u8 {
    (value & 0b0000000000001111) as u8
}

fn x(value: u16) -> u8 {
    ((value & 0b0000111100000000) >> 8) as u8
}

fn y(value: u16) -> u8 {
    ((value & 0b0000000011110000) >> 4) as u8
}

fn clear_terminal() {
    print!("{esc}c", esc = 27 as char);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_hello_world() {}
}
