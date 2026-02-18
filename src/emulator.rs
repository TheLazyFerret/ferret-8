//! emulator.rs
//! Hardware emulator module, although CHIP-8 was never implemented.

mod display;
pub mod error;
mod font;
mod stack;

use crate::decoder::Instruction;
use self::display::*;
use self::error::EmuError;
use self::font::*;
use self::stack::{Stack, error::StackError};

use anyhow::Result;
use rand::prelude::*;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

const MEMORY_SIZE: usize = 4096;
const REG_SIZE: usize = 16;
/// Due the first 512 bytes are reserved, programs start in this address.
const START_ADDR: usize = 0x200;
/// Semi special reg, used by many instructions as flag.
const REG_F: usize = 15;
/// Number of keys in the pad.
const KEY_SIZE: usize = 16;

const MODERN_COMPATIBILITY: bool = true;

/// The CHIP-8 count with the next specifications:
/// - 4KB of memory. The first 512 bytes are reserved, therefore should not be used by the programs.
/// - 16 general purpose 8 bit registers.
/// - 1 special 16 bit register named I, used for storing memory addresses.
/// - 2 special purpose 8 bit register, for delay and sound.
/// - 16 bit program counter pointer.
/// - 1 64x32 monochrome display. See src/emulator/display.rs
/// - 1 16x8 stack. See src/emulator/stackrs
///
/// For the registers i and pc, the struct will use an usize to reduce the number of casts.
#[derive(Debug)]
pub struct Emulator {
  memory: [u8; MEMORY_SIZE],
  reg: [u8; REG_SIZE],
  reg_i: usize,
  reg_pc: usize,
  reg_delay: u8,
  reg_sound: u8,
  display: Display,
  stack: Stack,
}

impl Emulator {
  /// Creates a new instance of the emulator.
  pub fn new() -> Self {
    let mut this = Self::default();
    this.load_fonts();
    this
  }

  /// Return the instruction pointed by reg_pc, then increase reg_pc.
  /// Remember each instruction is 16 bit, in BE.
  pub fn fetch(&mut self) -> Result<u16, EmuError> {
    if self.reg_pc + 1 >= MEMORY_SIZE {
      return Err(EmuError::InvalidAddress(self.reg_pc));
    }
    let value_high = (self.memory[self.reg_pc] as u16) << 8;
    let value_low = self.memory[self.reg_pc + 1] as u16;
    let instr = value_high + value_low;
    self.reg_pc += 2;
    Ok(instr)
  }
  
  /// Small wrapper around the internal display, required by the frontend.
  pub fn display_val(&self, x: usize, y: usize) -> bool {
    self.display.get(x, y)
  }

  /// execute the corresponding instruction depending instr.
  /// Basically match each function with each Instruction.
  pub fn execute(&mut self, instr: Instruction, rng: &mut ThreadRng, keys: &[bool]) -> Result<()> {
    match instr {
      | Instruction::Cls => self.clear_display(),
      | Instruction::Return => self.ret()?,
      | Instruction::SetPC(n) => self.set_pc(n),
      | Instruction::Call(n) => self.call(n)?,
      | Instruction::SeInmm(x, n) => self.se_inmm(x, n),
      | Instruction::SneInmm(x, n) => self.sne_inmm(x, n),
      | Instruction::SeReg(x, y) => self.se_reg(x, y),
      | Instruction::SneReg(x, y) => self.sne_reg(x, y),
      | Instruction::LoadInmm(x, n) => self.load_inmm(x, n),
      | Instruction::Sum(x, n) => self.sum(x, n),
      | Instruction::LoadI(n) => self.load_i(n),
      | Instruction::Jump(n) => self.jump(n)?,
      | Instruction::Rand(x, n) => self.rand(x, n, rng),
      | Instruction::Display(x, y, n) => self.display(x, y, n),
      | Instruction::LoadReg(x, y) => self.load_reg(x, y),
      | Instruction::Or(x, y) => self.or(x, y),
      | Instruction::And(x, y) => self.and(x, y),
      | Instruction::Xor(x, y) => self.xor(x, y),
      | Instruction::Add(x, y) => self.add(x, y),
      | Instruction::Sub(x, y) => self.sub(x, y),
      | Instruction::SubRev(x, y) => self.rev_sub(x, y),
      | Instruction::ShiftRight(x, y) => self.right_shift(x, y),
      | Instruction::ShiftLeft(x, y) => self.left_shift(x, y),
      | Instruction::Skip(x) => self.skip_key(x, keys)?,
      | Instruction::Snkip(x) => self.snkip_key(x, keys)?,
      | Instruction::GetDelay(x) => self.get_delay(x),
      | Instruction::WaitKey(x) => self.wait_key(x, keys)?,
      | Instruction::LoadDelay(x) => self.load_delay(x),
      | Instruction::LoadSound(x) => self.load_sound(x),
      | Instruction::AddI(x) => self.add_to_index(x),
      | Instruction::LoadFont(x) => self.load_font(x)?,
      | Instruction::Bcd(x) => self.binary_dec(x)?,
      | Instruction::StMem(x) => self.store_mem(x)?,
      | Instruction::LdMem(x) => self.load_mem(x)?,
    }
    Ok(())
  }

  /// Load a new program in the memory.
  pub fn load_program(&mut self, program: &[u8]) -> Result<(), EmuError> {
    if program.len() >= MEMORY_SIZE - START_ADDR {
      return Err(EmuError::ProgramTooBig(program.len()));
    }
    for n in program.iter().enumerate() {
      self.memory[START_ADDR + n.0] = *n.1;
    }
    Ok(())
  }

  /// Load the fonts in the reserved zone of the memory.
  fn load_fonts(&mut self) {
    for font in FONTS.iter().enumerate() {
      let rpos = FONT_START_ADDRESS + (font.0 * FONT_SIZE);
      for byte in font.1.iter().enumerate() {
        self.memory[rpos + byte.0] = *byte.1;
      }
    }
  }

  /// Print (standard output) the current state of the display. Used only for debugging.
  #[allow(dead_code)]
  pub fn dumb_print(&self) {
    for y in 0..DISPLAY_HEIGHT {
      for x in 0..DISPLAY_WIDTH {
        if self.display.get(x, y) {
          print!("█");
        } else {
          print!(" ");
        }
      }
      println!();
    }
  }
}

impl Default for Emulator {
  fn default() -> Self {
    Self {
      memory: [0; MEMORY_SIZE],
      reg: [0; REG_SIZE],
      reg_i: 0,
      reg_pc: START_ADDR,
      reg_delay: 0,
      reg_sound: 0,
      display: Display::new(),
      stack: Stack::new(),
    }
  }
}

impl Emulator {
  /// Clear the display, setting all the pixels to off.
  fn clear_display(&mut self) {
    self.display.clear();
  }

  /// Return from a subrotine, reducing the stack.
  fn ret(&mut self) -> Result<(), StackError> {
    let ret_value = self.stack.pop()?;
    debug_assert!(ret_value < MEMORY_SIZE);
    self.reg_pc = ret_value;
    Ok(())
  }

  /// Set the program counter to addr, without increasing stack.
  fn set_pc(&mut self, addr: usize) {
    debug_assert!(addr < 0xFFF);
    self.reg_pc = addr;
  }

  /// Jump to the subroutine in addr, increasing the stack.
  fn call(&mut self, addr: usize) -> Result<(), StackError> {
    debug_assert!(addr < 0xFFF);
    self.stack.push(self.reg_pc)?;
    self.reg_pc = addr;
    Ok(())
  }

  /// Compares reg and inmm, and if equal, increase pc in 2.
  fn se_inmm(&mut self, reg: usize, inmm: u8) {
    debug_assert!(reg < REG_SIZE);
    if self.reg[reg] == inmm {
      self.reg_pc += 2;
    }
  }

  /// Compares reg and inmm, and if not equal, increase pc in 2.
  fn sne_inmm(&mut self, reg: usize, inmm: u8) {
    debug_assert!(reg < REG_SIZE);
    if self.reg[reg] != inmm {
      self.reg_pc += 2;
    }
  }

  /// Compares reg_x and reg_y, and if equal, increase pc in 2.
  fn se_reg(&mut self, reg_x: usize, reg_y: usize) {
    debug_assert!(reg_x < REG_SIZE);
    debug_assert!(reg_y < REG_SIZE);
    if self.reg[reg_x] == self.reg[reg_y] {
      self.reg_pc += 2;
    }
  }

  /// Compares reg_x and reg_y, and if not equal, increase pc in 2.
  fn sne_reg(&mut self, reg_x: usize, reg_y: usize) {
    debug_assert!(reg_x < REG_SIZE);
    debug_assert!(reg_y < REG_SIZE);
    if self.reg[reg_x] != self.reg[reg_y] {
      self.reg_pc += 2;
    }
  }

  /// Load an inmmediate value in a register.
  fn load_inmm(&mut self, reg: usize, inmm: u8) {
    debug_assert!(reg < REG_SIZE);
    self.reg[reg] = inmm;
  }

  /// Sum and register and a inmm, without carry.
  fn sum(&mut self, reg: usize, inmm: u8) {
    debug_assert!(reg < REG_SIZE);
    self.reg[reg] = self.reg[reg].wrapping_add(inmm);
  }

  /// Set reg I to the inmmediate value.
  fn load_i(&mut self, inmm: usize) {
    debug_assert!(inmm < 0xFFF);
    self.reg_i = inmm;
  }

  /// Jump to the instrucction in addr reg V0 + inmm
  fn jump(&mut self, inmm: usize) -> Result<(), EmuError> {
    debug_assert!(inmm < 0xFFF);
    let sum = (self.reg[0] as usize).wrapping_add(inmm);
    if sum > MEMORY_SIZE {
      Err(EmuError::InvalidAddress(sum))
    } else {
      self.reg_pc = sum;
      Ok(())
    }
  }

  /// Generates a random value, binary AND with inmm, store the result in reg X.
  fn rand(&mut self, reg: usize, inmm: u8, rng: &mut ThreadRng) {
    debug_assert!(reg < REG_SIZE);
    let random: u8 = rng.random();
    self.reg[reg] = random & inmm;
  }

  /// Draw an inmm pixels tall sprite from the memory in location pointed by I, at the coordinates reg X and reg Y.
  /// All the pixels that are on the display will be turned off (if collission), setting reg 15 to 1.
  fn display(&mut self, reg_x: usize, reg_y: usize, inmm: u8) {
    debug_assert!(reg_x < REG_SIZE);
    debug_assert!(reg_y < REG_SIZE);
    let x = (self.reg[reg_x] % DISPLAY_WIDTH as u8) as usize;
    let y = (self.reg[reg_y] % DISPLAY_HEIGHT as u8) as usize;
    self.reg[REG_F] = 0;
    for yline in 0..(inmm as usize) {
      debug_assert!((yline + self.reg_i) < MEMORY_SIZE);
      let sprite_byte = self.memory[self.reg_i + yline];
      // For each bit.
      for xline in 0..8 {
        let sprite_bit = (sprite_byte & (0b10000000 >> xline)) > 0;
        let abs_pos = ((x + xline), (y + yline));
        // Collision (sprite bit and screen pixel both on)
        if sprite_bit && self.display.get(abs_pos.0, abs_pos.1) {
          self.display.set(abs_pos.0, abs_pos.1, false);
          self.reg[REG_F] = 1;
        } else if sprite_bit && !self.display.get(abs_pos.0, abs_pos.1) {
          self.display.set(abs_pos.0, abs_pos.1, true);
        }
      }
    }
  }

  /// Set the value of reg X to the value of reg Y.
  fn load_reg(&mut self, reg_x: usize, reg_y: usize) {
    debug_assert!(reg_x < REG_SIZE);
    debug_assert!(reg_y < REG_SIZE);
    self.reg[reg_x] = self.reg[reg_y];
  }

  /// The value of reg X will be the or between reg X and Y values.
  fn or(&mut self, reg_x: usize, reg_y: usize) {
    debug_assert!(reg_x < REG_SIZE);
    debug_assert!(reg_y < REG_SIZE);
    self.reg[reg_x] = self.reg[reg_x] | self.reg[reg_y];
  }

  /// The value of reg X will be the and between reg X and Y values.
  fn and(&mut self, reg_x: usize, reg_y: usize) {
    debug_assert!(reg_x < REG_SIZE);
    debug_assert!(reg_y < REG_SIZE);
    self.reg[reg_x] = self.reg[reg_x] & self.reg[reg_y];
  }

  /// The value of reg X will be the xor between reg X and Y values.
  fn xor(&mut self, reg_x: usize, reg_y: usize) {
    debug_assert!(reg_x < REG_SIZE);
    debug_assert!(reg_y < REG_SIZE);
    self.reg[reg_x] = self.reg[reg_x] ^ self.reg[reg_y];
  }

  /// reg X = reg X + reg Y, setting reg 15 to 1 if overflow.
  fn add(&mut self, reg_x: usize, reg_y: usize) {
    debug_assert!(reg_x < REG_SIZE);
    debug_assert!(reg_y < REG_SIZE);
    let sum = self.reg[reg_x].overflowing_add(self.reg[reg_y]);
    self.reg[reg_x] = sum.0;
    self.reg[REG_F] = sum.1 as u8;
  }

  /// reg X = reg X - reg Y, setting reg 15 to 1 if reg X > reg Y.
  fn sub(&mut self, reg_x: usize, reg_y: usize) {
    debug_assert!(reg_x < REG_SIZE);
    debug_assert!(reg_y < REG_SIZE);
    let x = self.reg[reg_x];
    let y = self.reg[reg_y];
    let sub = x.wrapping_sub(y);
    if x > y {
      self.reg[REG_F] = 1;
    } else {
      self.reg[REG_F] = 0;
    }
    self.reg[reg_x] = sub;
  }

  /// reg X = reg Y - reg X, setting reg 15 to 1 if reg Y > reg X.
  fn rev_sub(&mut self, reg_x: usize, reg_y: usize) {
    debug_assert!(reg_x < REG_SIZE);
    debug_assert!(reg_y < REG_SIZE);
    let x = self.reg[reg_x];
    let y = self.reg[reg_y];
    let sub = y.wrapping_sub(x);
    if y > x {
      self.reg[REG_F] = 1;
    } else {
      self.reg[REG_F] = 0;
    }
    self.reg[reg_x] = sub;
  }

  /// Set reg X = reg Y, then shift to the right(1), setting reg F to the bit out.
  fn right_shift(&mut self, reg_x: usize, reg_y: usize) {
    debug_assert!(reg_x < REG_SIZE);
    debug_assert!(reg_y < REG_SIZE);
    if self.reg[reg_y] & 0b00000001 == 1 {
      self.reg[REG_F] = 1;
    } else {
      self.reg[REG_F] = 0;
    }
    self.reg[reg_x] = self.reg[reg_y] >> 1;
  }

  /// Set reg X = reg Y, then shift to the left(1), setting reg F to the bit out.
  fn left_shift(&mut self, reg_x: usize, reg_y: usize) {
    debug_assert!(reg_x < REG_SIZE);
    debug_assert!(reg_y < REG_SIZE);
    if (self.reg[reg_y] & 0b10000000) >> 7 == 1 {
      self.reg[REG_F] = 1;
    } else {
      self.reg[REG_F] = 0;
    }
    self.reg[reg_x] = self.reg[reg_y] << 1;
  }

  /// Skip the next instruction if the key in reg X is being pressed.
  fn skip_key(&mut self, reg: usize, keys: &[bool]) -> Result<(), EmuError> {
    debug_assert!(keys.len() == KEY_SIZE);
    debug_assert!(reg < REG_SIZE);
    let value = self.reg[reg] as usize;
    if value >= KEY_SIZE {
      Err(EmuError::UnknownKey(value))
    } else {
      if keys[value] == true {
        self.reg_pc += 2;
      }
      Ok(())
    }
  }

  /// Skip the next instruction if the key in reg X is being pressed.
  fn snkip_key(&mut self, reg: usize, keys: &[bool]) -> Result<(), EmuError> {
    debug_assert!(keys.len() == KEY_SIZE);
    debug_assert!(reg < REG_SIZE);
    let value = self.reg[reg] as usize;
    if value >= KEY_SIZE {
      Err(EmuError::UnknownKey(value))
    } else {
      if keys[value] == false {
        self.reg_pc += 2;
      }
      Ok(())
    }
  }

  /// Set reg X to the value in delay reg.
  fn get_delay(&mut self, reg: usize) {
    debug_assert!(reg < REG_SIZE);
    self.reg[reg] = self.reg_delay;
  }

  /// Set reg delay to the value in reg X
  fn load_delay(&mut self, reg: usize) {
    debug_assert!(reg < REG_SIZE);
    self.reg_delay = self.reg[reg];
  }

  /// Set reg sound to the value in reg X
  fn load_sound(&mut self, reg: usize) {
    debug_assert!(reg < REG_SIZE);
    self.reg_sound = self.reg[reg];
  }

  /// reg I will be reg I + reg X.
  ///
  /// In the original interpreter, VF was not affected, but in some modern yes.
  /// Due some games relies in this behaviour, by default, will set VF in case of overflow.
  /// Overflow occurs when reg I > 0x0FFF
  fn add_to_index(&mut self, reg: usize) {
    debug_assert!(reg < REG_SIZE);
    self.reg_i = self.reg_i + self.reg[reg] as usize;
    if self.reg_i > 0x0FFF {
      self.reg[REG_F] = 1;
    } else {
      self.reg[REG_F] = 0;
    }
  }

  /// Enter an infinite loop(decrease pc) until key in reg X is pressed.
  fn wait_key(&mut self, reg: usize, keys: &[bool]) -> Result<(), EmuError> {
    debug_assert!(keys.len() == KEY_SIZE);
    debug_assert!(reg < REG_SIZE);
    let value = self.reg[reg] as usize;
    if value >= KEY_SIZE {
      Err(EmuError::UnknownKey(value))
    } else {
      if keys[value] == false {
        self.reg_pc -= 2;
      }
      Ok(())
    }
  }

  /// Set reg I to the start position of a font addr.
  fn load_font(&mut self, reg: usize) -> Result<(), EmuError> {
    debug_assert!(reg < REG_SIZE);
    let value = self.reg[reg];
    match value {
      | 0x0 => self.reg_i = FONT_START_ADDRESS,
      | 0x1 => self.reg_i = FONT_START_ADDRESS + FONT_SIZE,
      | 0x2 => self.reg_i = FONT_START_ADDRESS + (FONT_SIZE * 0x2),
      | 0x3 => self.reg_i = FONT_START_ADDRESS + (FONT_SIZE * 0x3),
      | 0x4 => self.reg_i = FONT_START_ADDRESS + (FONT_SIZE * 0x4),
      | 0x5 => self.reg_i = FONT_START_ADDRESS + (FONT_SIZE * 0x5),
      | 0x6 => self.reg_i = FONT_START_ADDRESS + (FONT_SIZE * 0x6),
      | 0x7 => self.reg_i = FONT_START_ADDRESS + (FONT_SIZE * 0x7),
      | 0x8 => self.reg_i = FONT_START_ADDRESS + (FONT_SIZE * 0x8),
      | 0x9 => self.reg_i = FONT_START_ADDRESS + (FONT_SIZE * 0x9),
      | 0xA => self.reg_i = FONT_START_ADDRESS + (FONT_SIZE * 0xA),
      | 0xB => self.reg_i = FONT_START_ADDRESS + (FONT_SIZE * 0xB),
      | 0xC => self.reg_i = FONT_START_ADDRESS + (FONT_SIZE * 0xC),
      | 0xD => self.reg_i = FONT_START_ADDRESS + (FONT_SIZE * 0xD),
      | 0xE => self.reg_i = FONT_START_ADDRESS + (FONT_SIZE * 0xE),
      | 0xF => self.reg_i = FONT_START_ADDRESS + (FONT_SIZE * 0xF),
      | _ => return Err(EmuError::UnknownFont(value)),
    }
    Ok(())
  }

  /// Put the digits of the number stored in reg X in I, I+1 and I+2 (decimal).
  fn binary_dec(&mut self, reg: usize) -> Result<(), EmuError> {
    debug_assert!(reg < REG_SIZE);
    let value = self.reg[reg];
    let pos = ((self.reg_i), (self.reg_i + 1), (self.reg_i + 2));
    if pos.0 >= MEMORY_SIZE {
      return Err(EmuError::InvalidAddress(pos.0));
    } else if pos.1 >= MEMORY_SIZE {
      return Err(EmuError::InvalidAddress(pos.1));
    } else if pos.2 >= MEMORY_SIZE {
      return Err(EmuError::InvalidAddress(pos.2));
    }
    self.memory[pos.0] = (value / 100) % 10; // More significant digit.
    self.memory[pos.1] = (value / 10) % 10;
    self.memory[pos.2] = value % 10; // Less significant digit.
    Ok(())
  }

  /// Load the values of the reg (from 0 to X, both included) into memory, starting in reg I.
  ///
  /// In modern interpreters, the reg I won't change,
  /// while in the original CHIP-8 will change to the value reg I + x + 1
  ///
  /// I made this option toggeable for a bit better compatibility with some roms.
  fn store_mem(&mut self, reg: usize) -> Result<(), EmuError> {
    debug_assert!(reg < REG_SIZE);
    for r in 0..=reg {
      let pos = self.reg_i + r;
      if pos >= MEMORY_SIZE {
        return Err(EmuError::InvalidAddress(pos));
      }
      self.memory[pos] = self.reg[r];
    }
    if !MODERN_COMPATIBILITY {
      self.reg_i = self.reg_i + reg + 1;
    }
    Ok(())
  }

  /// Load the values of the mem (from 0 to X, both included) into reg, starting in reg I.
  /// Follows the same compatibility logic as store_mem.
  fn load_mem(&mut self, reg: usize) -> Result<(), EmuError> {
    debug_assert!(reg < REG_SIZE);
    for r in 0..=reg {
      let pos = self.reg_i + r;
      if pos >= MEMORY_SIZE {
        return Err(EmuError::InvalidAddress(pos));
      }
      self.reg[r] = self.memory[pos];
    }
    if !MODERN_COMPATIBILITY {
      self.reg_i = self.reg_i + reg + 1;
    }
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use crate::emulator::{Emulator, START_ADDR};

  #[test]
  fn test_load_program() {
    let vec = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let mut emu = Emulator::new();
    emu.load_program(&vec).unwrap();
    assert_eq!(emu.memory[START_ADDR], vec[0]);
  }
}
