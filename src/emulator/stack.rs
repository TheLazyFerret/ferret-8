//! stack.rs
//! Stack of the CHIP-8

pub mod error;

use crate::emulator::stack::error::StackError;

const STACK_SIZE: usize = 16;

/// The original implementation of the CHIP-8 stack was 16 entries x 8 bits each.
/// Due I have no reason to do in other way, this small implementation will work in the same way.
///
/// The stack_pointer will count the number of values in the stack.
/// - If it is equal to 0 -> it is empty.
/// - If it is equal to 16 -> it is full.
///
/// To index the values in side the array, except when the stack is empty, array[stack_pointer - 1]
#[derive(Debug)]
pub struct Stack {
  array: [usize; STACK_SIZE],
  stack_pointer: usize,
}

impl Stack {
  /// Creates a new Stack.
  pub fn new() -> Self {
    Self::default()
  }

  /// Increase the stack pointer and then push a new value in the stack. If it is full, returns an error (overflow).
  pub fn push(&mut self, v: usize) -> Result<(), StackError> {
    if self.stack_pointer >= STACK_SIZE {
      Err(StackError::Overflow)
    } else {
      self.array[self.stack_pointer] = v;
      self.stack_pointer += 1;
      Ok(())
    }
  }

  /// Reduce the stack pointer and return the value it points. If it is empty, returns an error (underflow).
  pub fn pop(&mut self) -> Result<usize, StackError> {
    if self.stack_pointer == 0 {
      Err(StackError::Underflow)
    } else {
      self.stack_pointer -= 1;
      Ok(self.array[self.stack_pointer])
    }
  }
}

impl Default for Stack {
  fn default() -> Self {
    Self { array: [0; STACK_SIZE], stack_pointer: 0 }
  }
}

#[cfg(test)]
mod test {
  use crate::emulator::stack::{STACK_SIZE, Stack, error::StackError};

  #[test]
  fn test_push() {
    let mut stack = Stack::new();
    for n in 0..STACK_SIZE {
      assert_eq!(stack.push(n), Ok(()));
    }
    assert_eq!(stack.push(0), Err(StackError::Overflow));
  }

  #[test]
  fn test_pop() {
    let mut stack = Stack::new();
    for n in 0..STACK_SIZE {
      stack.push(n).unwrap();
    }
    for n in (0..STACK_SIZE).rev() {
      assert_eq!(stack.pop(), Ok(n));
    }
    assert_eq!(stack.pop(), Err(StackError::Underflow));
  }
}
