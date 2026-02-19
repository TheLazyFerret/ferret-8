//! cli.rs
//! Functions for argument parsing

use std::sync::RwLock;

use clap::Parser;

pub static PROGRAM_NAME: RwLock<String> = RwLock::new(String::new());
pub static CYCLES: RwLock<usize> = RwLock::new(700);
pub static UPSCALE_FACTOR: RwLock<usize> = RwLock::new(20);

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  program: String,
  #[arg(short, long)]
  cycles: Option<usize>,
  #[arg(short, long)]
  upscale_factor: Option<usize>,
}

/// Parse the command arguments of the program.
pub fn parse_arguments() {
  let args = Args::parse();
  *PROGRAM_NAME.try_write().unwrap() = args.program;
  if args.cycles.is_some() {
    *CYCLES.try_write().unwrap() = args.cycles.unwrap();
  }
  if args.upscale_factor.is_some() {
    *CYCLES.try_write().unwrap() = args.upscale_factor.unwrap();
  }
}
