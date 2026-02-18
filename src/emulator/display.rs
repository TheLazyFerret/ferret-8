//! display.rs
//! Display of the CHIP-8

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

/// The original CHIP-8 uses a 64x32 pixel, monochrome (on/off).
///
/// My implementation internally uses a single 2048 array, avoid double indirection.
/// For this, internally implements a function to convert (x, y) coordinates into an absolute position.
///
/// Remember the (0, 0) is in the top left corner.
#[derive(Debug)]
pub struct Display {
  array: [bool; DISPLAY_HEIGHT * DISPLAY_WIDTH],
}

impl Default for Display {
  fn default() -> Self {
    Self { array: [false; DISPLAY_HEIGHT * DISPLAY_WIDTH] }
  }
}

impl Display {
  /// Instance a new Display.
  pub fn new() -> Self {
    Self::default()
  }

  /// Convert an (x, y) into an absolute position.
  ///
  /// Considering each row has DISPLAY_HEIGHT positions, to each row multiply y * DISPLAY_WIDTH.
  /// For indexing inside the row, just need to sum the position x.
  ///
  /// Example:\
  /// (30, 30) -> (30 * 64) + 30 = 1950\
  /// (0, 15) -> (15 * 64) + 0 = 960\
  /// (5, 0) -> (0 * 64) + 5 = 5\
  /// (63, 63) -> (63 * 64) + 63 = 4095
  fn transform_cords(x: usize, y: usize) -> usize {
    debug_assert!((x < DISPLAY_WIDTH) && (y < DISPLAY_HEIGHT));
    (y * DISPLAY_WIDTH) + x
  }

  /// Set a value in a pixel.
  pub fn set(&mut self, x: usize, y: usize, v: bool) {
    self.array[Self::transform_cords(x, y)] = v
  }

  /// Get the current value in a pixel.
  pub fn get(&self, x: usize, y: usize) -> bool {
    self.array[Self::transform_cords(x, y)]
  }

  /// Set all the bits in the display to 0.
  pub fn clear(&mut self) {
    self.array = [false; DISPLAY_HEIGHT * DISPLAY_WIDTH]
  }
}

#[cfg(test)]
mod test {
  use crate::emulator::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH, Display};

  #[test]
  fn test_transform_cords() {
    let mut display = Display::new();
    display.set(DISPLAY_WIDTH - 1, DISPLAY_HEIGHT - 1, true);
    assert_eq!(display.get(DISPLAY_WIDTH - 1, DISPLAY_HEIGHT - 1), true);
    assert_eq!(display.array[DISPLAY_HEIGHT * DISPLAY_WIDTH - 1], true);
  }
}
