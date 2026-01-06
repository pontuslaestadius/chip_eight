use crate::display::display_trait::Ch8Display;
use crate::*;
use std::io::{stdout, Write};

pub struct TerminalDisplay {
    display_buffer: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

impl TerminalDisplay {
    pub fn new() -> Self {
        TerminalDisplay {
            display_buffer: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        }
    }
}

impl Ch8Display for TerminalDisplay {
    fn buffer(&mut self) -> &mut [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT] {
        &mut self.display_buffer
    }
    fn clear(&mut self) {
        self.display_buffer = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }
    fn render(&self) {
        let mut out = stdout();
        write!(out, "{}", termion::cursor::Goto(1, 1)).unwrap();

        for row in self.display_buffer.iter() {
            for &pixel in row.iter() {
                let ch = if pixel { '█' } else { ' ' };
                write!(out, "{}", ch).unwrap();
            }
            writeln!(out).unwrap();
        }

        out.flush().unwrap();
    }
}

fn clear_terminal() {
    print!("{esc}c", esc = 27 as char);
}
