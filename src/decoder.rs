//! decoder.rs
//! Decoder for the CHIP-8 binary instructions.

use crate::decoder::error::DecodeError;

pub mod error;
mod opcodes;

/// Decodified instructions.
/// See opcodes.rs for the meaning of each variant.
pub enum Instruction {
  SetPC(usize),              // 0x1nnn
  Call(usize),               // 0x2nnn
  SeInmm(usize, u8),         // 0x3xnn
  SneInmm(usize, u8),        // 0x4xnn
  SeReg(usize, usize),       // 0x5xy0
  SneReg(usize, usize),      // 0x9xy0
  LoadInmm(usize, u8),       // 0x6xnn
  Sum(usize, u8),            // 0x7xnn
  LoadI(usize),              // 0xAnnn
  Jump(usize, u16),          // 0xBnnn
  Rand(usize, u8),           // 0xCxnn
  Display(usize, usize, u8), // 0xDxyn
}

pub fn decode(instr: u16) -> Result<Instruction, DecodeError> {
  let opcode: u16 = instr & 0xF000;
  match opcode {
    | _ => Err(DecodeError::Unknown(instr)),
  }
}

#[cfg(test)]
mod test {}
