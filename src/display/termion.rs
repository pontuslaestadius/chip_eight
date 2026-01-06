// use crate::chip::Chip;
// use crate::*;
// use std::io::Write;
// use termion::raw::RawTerminal;

// impl Chip {
//     pub fn termion_display<W: Write + std::os::fd::AsFd>(
//         &mut self,
//         stdout: &mut RawTerminal<W>,
//         instruction: u16,
//     ) {
//         let vx = self.read_var_reg_x(instruction);
//         let vy = self.read_var_reg_y(instruction);
//         let n = n(instruction);
//         let x: u8 = vx % DISPLAY_WIDTH as u8;
//         let y: u8 = vy % DISPLAY_HEIGHT as u8;
//         self.registers.set(15, 0);
//         write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();

//         for row_n in 0..n.as_u8() {
//             let y_coordinate = (y + row_n) as usize;
//             if y_coordinate >= DISPLAY_WIDTH {
//                 break;
//             }
//             let n_th_byte = self.memory.read(self.registers.get_i() + (row_n as u16));
//             for bit in 0..8u8 {
//                 let value = n_th_byte >> (7 - bit) & 0x01;
//                 let x_coordinate = (x + bit) as usize;
//                 if x_coordinate >= DISPLAY_WIDTH {
//                     break; // No wrapping.
//                 }
//                 let render_value = self.display_buffer[y_coordinate][x_coordinate];
//                 if render_value && value == 0x01 {
//                     self.display_buffer[y_coordinate][x_coordinate] = false;
//                     write!(
//                         stdout,
//                         "{}",
//                         termion::cursor::Goto(1 + x_coordinate as u16, 1 + y_coordinate as u16)
//                     )
//                     .unwrap();
//                     stdout.write(b" ").unwrap();
//                     self.registers.set(15, 0);
//                 } else if value == 0x01 && !render_value {
//                     self.display_buffer[y_coordinate][x_coordinate] = true;
//                     write!(
//                         stdout,
//                         "{}",
//                         termion::cursor::Goto(1 + x_coordinate as u16, 1 + y_coordinate as u16)
//                     )
//                     .unwrap();
//                     stdout.write(b"x").unwrap();
//                 }
//             }
//         }

//         // for _y in 0..DISPLAY_HEIGHT {
//         //     for _x in 0..DISPLAY_WIDTH {
//         //         if self.display_buffer[_y][_x] {
//         //             stdout.write(b"x").unwrap();
//         //             // print!("⬜");
//         //         } else {
//         //             stdout.write(b" ").unwrap();
//         //             // print!("⬛");
//         //         }
//         //     }
//         //     // write!(stdout, "{}", termion::clear::CurrentLine).unwrap();

//         //     // write!(stdout, "{}", termion::clear::CurrentLine).unwrap();
//         // }
//         write!(
//             stdout,
//             "{}",
//             termion::cursor::Goto(1, 2 + DISPLAY_HEIGHT as u16)
//         )
//         .unwrap();

//         stdout.flush().unwrap();
//     }
//     // pub fn pretty_print_progress(&self, rows: usize) {
//     //     clear_terminal();
//     //     if self.program_counter > 10 && self.program_counter < 4080 {
//     //         for i in 0..rows * 2 {
//     //             if i % 2 != 0 {
//     //                 continue;
//     //             }
//     //             let pc = self.program_counter + i - rows;
//     //             let s = format!(
//     //                 "[{:03X?}, {:03X?}] -> {:02X?}{:02X?}",
//     //                 pc,
//     //                 pc + 1,
//     //                 self.ram[pc],
//     //                 self.ram[pc + 1]
//     //             );

//     //             if i == rows {
//     //                 println!(" * {}", s);
//     //             } else {
//     //                 println!("   {}", s);
//     //             }
//     //         }
//     //     }
//     // }
// }
// fn clear_terminal() {
//     print!("{esc}c", esc = 27 as char);
// }
