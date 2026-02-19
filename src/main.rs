//! main.rs
//! Entry point of the binary.

use std::{fs, io::Read};

use crate::cli::*;
use crate::decoder::decode;
use crate::emulator::Emulator;
use crate::frontend::TARGET_FPS;

use anyhow::Result;

mod cli;
mod decoder;
mod emulator;
mod frontend;

fn main() -> Result<()> {
  parse_arguments();

  // Retrieve necesary variables from the cli arguments.
  let program_name = PROGRAM_NAME.read().unwrap().clone();
  let cycles_per_frame = {
    let mut aux = CYCLES.read().unwrap().clone() / TARGET_FPS as usize;
    if aux < 1 {
      aux = 1;
    }
    aux
  };

  // Open and reach file.
  let mut vec = Vec::new();
  let mut file = fs::File::open(&program_name)?;
  file.read_to_end(&mut vec)?;

  // Creates and load the emulator.
  let mut emu = Emulator::new();
  emu.load_program(&vec)?;

  // Generates an rng, necessary for a instruction in the emulator.
  let mut rng = rand::rng();

  // Creates the window.
  let (mut rl, th) = frontend::init_raylib(&program_name);
  
  println!("COMPAT: {}", COMPATIBILITY.read().unwrap());

  while !rl.window_should_close() {
    emu.decrease_timers();
    let input = frontend::get_input(&mut rl);
    for _ in 0..cycles_per_frame {
      // Fetch
      let raw_instr = emu.fetch()?;
      // Decode
      let instr = decode(raw_instr)?;
      // Execute
      emu.execute(instr, &mut rng, &input)?;
    }

    let mut d = rl.begin_drawing(&th);
    if emu.should_refresh() {
      frontend::draw_display(&mut d, &emu);
      emu.refreshed();
    }
  }

  Ok(())
}
