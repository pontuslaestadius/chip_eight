extern crate rand;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

mod args;
mod chip;
mod controls;
mod display;
mod execute;
mod font;
mod memory;
mod nibble;
mod nibbles;
mod opcode;
mod registers;
mod stack;
mod timers;
extern crate termion;

use args::Args;
use font::*;
use std::io::{stdout, Read, Write};
use termion::async_stdin;
use termion::raw::IntoRawMode;

use crate::controls::Chip8Key;
use crate::display::terminal::TerminalDisplay;
use crate::nibble::Nibble;
// use crate::nibbles::Nibbles;
use crate::opcode::Opcode;
use fern::Dispatch;
use log::info;

const MEMORY_SIZE: usize = 4096;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_WIDTH: usize = 64;

// Due to how inputs are captured, this is the only way to exit the game,
// without external interrupt or kill commands.
const EXIT_GAME_KEY: u8 = b't';

fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

/// Processes one key press from stdin
fn handle_input(
    key_byte: u8,
    chip: &mut chip::Chip,
    stdout: &mut termion::raw::RawTerminal<std::io::StdoutLock<'static>>,
) -> Option<Chip8Key> {
    if key_byte == EXIT_GAME_KEY {
        write!(
            stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        )
        .unwrap();
        panic!("User requested exit");
    }
    chip.try_press(key_byte as char)
}

fn setup_terminal() -> (
    termion::raw::RawTerminal<std::io::StdoutLock<'static>>,
    termion::AsyncReader,
) {
    let stdout = Box::leak(Box::new(stdout()))
        .lock()
        .into_raw_mode()
        .unwrap();
    let stdin = async_stdin();
    (stdout, stdin)
}

fn load_rom(path: &PathBuf) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    info!("- Loading ROM from {:?}", path);
    let content = fs::read(path)?;
    Ok(content)
}

fn run_emulator(
    mut chip: &mut chip::Chip,
    mut stdout: &mut termion::raw::RawTerminal<std::io::StdoutLock<'static>>,
    stdin: &mut termion::AsyncReader,
    frame_interval_ms: u64,
) {
    info!("Starting event loop...");
    info!("VALU | OPCO | DESCRIPTION");
    let mut wait_for_input: Option<usize> = None;

    let mut bytes_iter = stdin.bytes();
    while chip.program_counter < 4096 {
        thread::sleep(Duration::from_millis(frame_interval_ms));

        let key_pressed: Option<Chip8Key> = bytes_iter
            .next()
            .and_then(|res| res.ok())
            .and_then(|b| handle_input(b, chip, stdout));

        chip.timers.tick();
        if let Some(vx) = wait_for_input {
            write!(
                stdout,
                "{}",
                termion::cursor::Goto(1, 2 + DISPLAY_HEIGHT as u16)
            )
            .unwrap();
            if let Some(key) = key_pressed {
                write!(stdout, "{}", termion::clear::CurrentLine).unwrap();
                chip.registers.set(Nibble::from_low(vx as u8), key.as_u8());
                wait_for_input = None;
            } else {
                write!(stdout, "Waiting...").unwrap();
                continue;
            }
            stdout.flush().unwrap();
        }
        let instruction_u16: u16 = chip.next_u16();
        let opcode = Opcode::decode(instruction_u16);

        info!("{:?}", opcode);

        chip.execute(opcode);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut stdout, mut stdin) = setup_terminal();

    init_logging()?;

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    info!("Initializing...");

    let args = Args::parse();
    info!("- Initializing display...");
    let display = TerminalDisplay::new();
    info!("- Creating emulator...");
    let mut chip = chip::Chip::new(display);

    chip.load_rom(&load_rom(&args.rom)?);
    info!("- Starting event loop...");
    info!("VALU | OPCO | DESCRIPTION");

    run_emulator(&mut chip, &mut stdout, &mut stdin, args.frame_interval_ms);
    Ok(())
}

fn x(value: u16) -> u8 {
    ((value & 0b0000111100000000) >> 8) as u8
}
