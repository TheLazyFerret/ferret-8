//! error.rs
//! Possible error of the CHIP-8 stack (overflow and underflow)

use std::{error::Error, fmt};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StackError {
  Overflow,
  Underflow,
}

impl fmt::Display for StackError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      | Self::Overflow => write!(f, "Stack overflow"),
      | Self::Underflow => write!(f, "Stack underflow"),
    }
  }
}

impl Error for StackError {}
