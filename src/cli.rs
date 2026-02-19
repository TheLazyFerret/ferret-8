//! cli.rs
//! Functions for argument parsing

use std::sync::RwLock;

use clap::Parser;

pub static PROGRAM_NAME: RwLock<String> = RwLock::new(String::new());
pub static CYCLES: RwLock<usize> = RwLock::new(0);
pub static UPSCALE_FACTOR: RwLock<usize> = RwLock::new(0);

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  program: String,
  #[arg(short, long, default_value_t = 700)]
  cycles: usize,
  #[arg(short, long, default_value_t = 20)]
  upscale_factor: usize,
}

/// Parse the command arguments of the program.
pub fn parse_arguments() {
  let args = Args::parse();
  *PROGRAM_NAME.try_write().unwrap() = args.program;
  *CYCLES.try_write().unwrap() = args.cycles;
  *UPSCALE_FACTOR.try_write().unwrap() = args.upscale_factor;
}
