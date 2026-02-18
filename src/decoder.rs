//! decoder.rs
//! Decoder for the CHIP-8 binary instructions.

use crate::decoder::{error::DecodeError, opcodes::*};

pub mod error;
mod opcodes;

/// Decodified instructions.
/// See opcodes.rs for the meaning of each variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
  Jump(usize),               // 0xBnnn
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
  Snkip(usize),              // 0xExA1
  GetDelay(usize),           // 0xFx07
  WaitKey(usize),            // 0xFx0A
  LoadDelay(usize),          // 0xFx15
  LoadSound(usize),          // 0xFx18
  AddI(usize),               // 0xFx1E
  LoadFont(usize),           // 0xFx29
  Bcd(usize),                // 0xFx33
  StMem(usize),              // 0xFx55
  LdMem(usize),              // 0xFx65
}

/// Convert a binary instruction into an enum variant.\
/// CHIP-8 instructions are weird, so I splitted them between those that are completely unique,
/// and those that share the most significant nibble (CHIP-8 is BE):
/// - Twelve unique instructions.
/// - Two instructions that share 0x0.
/// - Nine instructions that share 0x8.
/// - Two instructions that share 0x2.
/// - Nine instructions that share 0xF.
///
/// This give a total of 34 instructions for the original CHIP-8 from 1970.
/// There is an additional instructions (0x0NNN), but it is not used in most roms.
///
/// The decoder use one main match block, comparing the first nibble. If match with one shared,
/// it enters in a second match, comparing the last or last two nibbles.
pub fn decode(instr: u16) -> Result<Instruction, DecodeError> {
  // Mask all the bits except the first nibble.
  let opcode: u16 = instr & 0xF000;
  match opcode {
    | 0x0000 => match instr {
      | CLS => Ok(Instruction::Cls),
      | RET => Ok(Instruction::Return),
      | _ => Err(DecodeError::Unknown(instr)),
    },
    | SET_PC => {
      let inmm = (instr & 0x0FFF) as usize;
      Ok(Instruction::SetPC(inmm))
    },
    | CALL => {
      let inmm = (instr & 0x0FFF) as usize;
      Ok(Instruction::Call(inmm))
    },
    | SE_INMM => {
      let reg = ((instr & 0x0F00) >> 8) as usize;
      let inmm = (instr & 0x00FF) as u8;
      Ok(Instruction::SeInmm(reg, inmm))
    },
    | SNE_INMM => {
      let reg = ((instr & 0x0F00) >> 8) as usize;
      let inmm = (instr & 0x00FF) as u8;
      Ok(Instruction::SneInmm(reg, inmm))
    },
    | SE_REG => {
      let reg_x = ((instr & 0x0F00) >> 8) as usize;
      let reg_y = ((instr & 0x00F0) >> 4) as usize;
      Ok(Instruction::SeReg(reg_x, reg_y))
    },
    | LD_INMM => {
      let reg = ((instr & 0x0F00) >> 8) as usize;
      let inmm = (instr & 0x00FF) as u8;
      Ok(Instruction::LoadInmm(reg, inmm))
    },
    | SUM => {
      let reg = ((instr & 0x0F00) >> 8) as usize;
      let inmm = (instr & 0x00FF) as u8;
      Ok(Instruction::Sum(reg, inmm))
    },
    | 0x8000 => {
      let reg_x = ((instr & 0x0F00) >> 8) as usize;
      let reg_y = ((instr & 0x00F0) >> 4) as usize;
      let opcode = instr & 0xF00F;
      match opcode {
        | LD_REG => Ok(Instruction::LoadReg(reg_x, reg_y)),
        | OR => Ok(Instruction::Or(reg_x, reg_y)),
        | AND => Ok(Instruction::And(reg_x, reg_y)),
        | XOR => Ok(Instruction::Xor(reg_x, reg_y)),
        | ADD => Ok(Instruction::Add(reg_x, reg_y)),
        | SUB => Ok(Instruction::Sub(reg_x, reg_y)),
        | SHR => Ok(Instruction::ShiftRight(reg_x, reg_y)),
        | SUBN => Ok(Instruction::SubRev(reg_x, reg_y)),
        | SHL => Ok(Instruction::ShiftLeft(reg_x, reg_y)),
        | _ => Err(DecodeError::Unknown(instr)),
      }
    },
    | SNE_REG => {
      let reg_x = ((instr & 0x0F00) >> 8) as usize;
      let reg_y = ((instr & 0x00F0) >> 4) as usize;
      Ok(Instruction::SneReg(reg_x, reg_y))
    },
    | LD_I => {
      let inmm = (instr & 0x0FFF) as usize;
      Ok(Instruction::LoadI(inmm))
    },
    | JUMP => {
      let inmm = (instr & 0x0FFF) as usize;
      Ok(Instruction::Jump(inmm))
    },
    | RAND => {
      let reg = ((instr & 0x0F00) >> 8) as usize;
      let inmm = (instr & 0x00FF) as u8;
      Ok(Instruction::Rand(reg, inmm))
    },
    | DISPLAY => {
      let reg_x = ((instr & 0x0F00) >> 8) as usize;
      let reg_y = ((instr & 0x00F0) >> 4) as usize;
      let inmm = (instr & 0x000F) as u8;
      Ok(Instruction::Display(reg_x, reg_y, inmm))
    },
    | 0xE000 => {
      let reg = ((instr & 0x0F00) >> 8) as usize;
      let opcode = instr & 0xF0FF;
      match opcode {
        | SKP => Ok(Instruction::Skip(reg)),
        | SNKP => Ok(Instruction::Snkip(reg)),
        | _ => Err(DecodeError::Unknown(instr)),
      }
    },
    | 0xF000 => {
      let reg = ((instr & 0x0F00) >> 8) as usize;
      let opcode = instr & 0xF0FF;
      match opcode {
        | ST_DELAY => Ok(Instruction::GetDelay(reg)),
        | WAIT_KEY => Ok(Instruction::WaitKey(reg)),
        | LD_DELAY => Ok(Instruction::LoadDelay(reg)),
        | LD_SOUND => Ok(Instruction::LoadSound(reg)),
        | ADD_I => Ok(Instruction::AddI(reg)),
        | LD_FONT => Ok(Instruction::LoadFont(reg)),
        | BCD => Ok(Instruction::Bcd(reg)),
        | ST_MEM => Ok(Instruction::StMem(reg)),
        | LD_MEM => Ok(Instruction::LdMem(reg)),
        | _ => Err(DecodeError::Unknown(instr)),
      }
    },
    | _ => Err(DecodeError::Unknown(instr)),
  }
}

/// Module test for decoder module.
#[cfg(test)]
mod test {
  use crate::decoder::{Instruction, decode};

  #[test]
  fn test_cls() {
    assert_eq!(decode(0x00E0), Ok(Instruction::Cls));
  }

  #[test]
  fn test_ret() {
    assert_eq!(decode(0x00EE), Ok(Instruction::Return));
  }

  #[test]
  fn test_setpc() {
    assert_eq!(decode(0x1FFF), Ok(Instruction::SetPC(0xFFF)));
  }

  #[test]
  fn test_call() {
    assert_eq!(decode(0x2FFF), Ok(Instruction::Call(0xFFF)));
  }

  #[test]
  fn test_seinmm() {
    assert_eq!(decode(0x3FFF), Ok(Instruction::SeInmm(0xF, 0xFF)));
  }

  #[test]
  fn test_sneinmm() {
    assert_eq!(decode(0x4FFF), Ok(Instruction::SneInmm(0xF, 0xFF)));
  }

  #[test]
  fn test_sereg() {
    assert_eq!(decode(0x5FF0), Ok(Instruction::SeReg(0xF, 0xF)));
  }

  #[test]
  fn test_ldinmm() {
    assert_eq!(decode(0x6FFF), Ok(Instruction::LoadInmm(0xF, 0xFF)));
  }

  #[test]
  fn test_sum() {
    assert_eq!(decode(0x7FFF), Ok(Instruction::Sum(0xF, 0xFF)));
  }

  #[test]
  fn test_ldreg() {
    assert_eq!(decode(0x8FF0), Ok(Instruction::LoadReg(0xF, 0xF)));
  }

  #[test]
  fn test_or() {
    assert_eq!(decode(0x8FF1), Ok(Instruction::Or(0xF, 0xF)));
  }

  #[test]
  fn test_and() {
    assert_eq!(decode(0x8FF2), Ok(Instruction::And(0xF, 0xF)));
  }

  #[test]
  fn test_xor() {
    assert_eq!(decode(0x8FF3), Ok(Instruction::Xor(0xF, 0xF)));
  }

  #[test]
  fn test_add() {
    assert_eq!(decode(0x8FF4), Ok(Instruction::Add(0xF, 0xF)));
  }

  #[test]
  fn test_sub() {
    assert_eq!(decode(0x8FF5), Ok(Instruction::Sub(0xF, 0xF)));
  }

  #[test]
  fn test_shr() {
    assert_eq!(decode(0x8FF6), Ok(Instruction::ShiftRight(0xF, 0xF)));
  }

  #[test]
  fn test_subn() {
    assert_eq!(decode(0x8FF7), Ok(Instruction::SubRev(0xF, 0xF)));
  }

  #[test]
  fn test_shl() {
    assert_eq!(decode(0x8FFE), Ok(Instruction::ShiftLeft(0xF, 0xF)));
  }

  #[test]
  fn test_snereg() {
    assert_eq!(decode(0x9FF0), Ok(Instruction::SneReg(0xF, 0xF)));
  }

  #[test]
  fn test_ldi() {
    assert_eq!(decode(0xAFFF), Ok(Instruction::LoadI(0xFFF)));
  }

  #[test]
  fn test_jump() {
    assert_eq!(decode(0xBFFF), Ok(Instruction::Jump(0xFFF)));
  }

  #[test]
  fn test_rand() {
    assert_eq!(decode(0xCFFF), Ok(Instruction::Rand(0xF, 0xFF)));
  }

  #[test]
  fn test_display() {
    assert_eq!(decode(0xDFFF), Ok(Instruction::Display(0xF, 0xF, 0xF)));
  }

  #[test]
  fn test_skp() {
    assert_eq!(decode(0xEF9E), Ok(Instruction::Skip(0xF)));
  }

  #[test]
  fn test_nskp() {
    assert_eq!(decode(0xEFA1), Ok(Instruction::Snkip(0xF)));
  }
}
