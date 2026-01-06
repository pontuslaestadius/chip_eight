use clap::Parser;
use std::path::PathBuf;

enum DisplayOptions {}

/// CHIP 8 Emulator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to a CHIP 8 ROM file.
    #[arg(long)]
    pub rom: PathBuf,

    // Interval between rendering frames in miliseconds.
    #[arg(long, default_value_t = 17)]
    pub frame_interval_ms: u64,
}
