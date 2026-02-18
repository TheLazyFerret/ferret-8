//! error.rs
//! Decode errors.

use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodeError {
  Unknown(u16),
}

impl fmt::Display for DecodeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      | Self::Unknown(n) => write!(f, "Unkown instruction 0x{:X}", n),
    }
  }
}

impl Error for DecodeError {}
