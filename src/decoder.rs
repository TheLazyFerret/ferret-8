//! decoder.rs
//! Decoder for the CHIP-8 binary instructions.

use crate::decoder::error::DecodeError;

pub mod error;
mod opcodes;

/// Decodified instructions.
/// See opcodes.rs for the meaning of each variant.
pub enum Instruction {
  Cls,                       // 0x00E0
  Return,                    // 0x00EE
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
  LoadReg(usize, usize),     // 0x8xy0
  Or(usize, usize),          // 0x8xy1
  And(usize, usize),         // 0x8xy2
  Xor(usize, usize),         // 0x8xy3
  Add(usize, usize),         // 0x8xy4
  Sub(usize, usize),         // 0x8xy5
  ShiftRight(usize, usize),  // 0x8xy6
  SubRev(usize, usize),      // 0x8xy7
  ShiftLeft(usize, usize),   // 0x8xy8
  Skip(usize),               // 0xEx9E
  NotSkip(usize),            // 0xExA1
  GetDelay(usize),           // 0xFx07
  WaitKey(usize),            // 0xFx0A
  LoadDelay(usize),          // 0xFx15
  LoadSound(usize),          // 0xFx18
  AddI(usize),               // 0xFx1E
  LoadSprite(usize),         // 0xFx29
  Bcd(usize),                // 0xFx33
  StMem(usize),              // 0xFx55
  LdMem(usize),              // 0xFx65
}

pub fn decode(instr: u16) -> Result<Instruction, DecodeError> {
  let opcode: u16 = instr & 0xF000;
  match opcode {
    | _ => Err(DecodeError::Unknown(instr)),
  }
}

#[cfg(test)]
mod test {}
