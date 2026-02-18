//! error.rs
//! Possible error of the CHIP-8 emulator.

use std::{error, fmt};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmuError {
  InvalidAddress(usize),
  ProgramTooBig(usize),
  UnknownFont(u8),
}

impl fmt::Display for EmuError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      | Self::InvalidAddress(n) => write!(f, "Access to an invalid address: {}", n),
      | Self::ProgramTooBig(n) => write!(f, "Not possible to load the program, too big: {}", n),
      | Self::UnknownFont(x) => write!(f, "Indexing an unkown font value: {}", x),
    }
  }
}

impl error::Error for EmuError {}
