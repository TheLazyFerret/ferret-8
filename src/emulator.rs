//! emulator.rs
//! Hardware emulator module, although CHIP-8 was never implemented.

use crate::emulator::{display::Display, stack::Stack};

mod display;
mod stack;
pub mod error;