//! frontend.rs
//! Manage to interconect the GUI and the emulator backend.

use crate::emulator::{DISPLAY_HEIGHT, DISPLAY_WIDTH, Emulator};
use crate::UPSCALE_FACTOR;

use raylib::prelude::*;

/// Target fps of the frontend.
pub const TARGET_FPS: u32 = 60;

const PIXEL_COLOR: Color = Color::new(255, 223, 194, 255);
const BG_COLOR: Color = Color::new(0, 0, 0, 255);

/// Return an initalized tuple (RaylibHandle, RaylibThread), setting some basic options.
pub fn init_raylib(title: &str) -> (RaylibHandle, RaylibThread) {
  let upscale_factor = UPSCALE_FACTOR.read().unwrap().clone();
  let size_w = (DISPLAY_WIDTH * upscale_factor) as i32;
  let size_h = (DISPLAY_HEIGHT * upscale_factor) as i32;
  let (mut rl, thread) =
    raylib::init().size(size_w, size_h).title(&format!("Ferret-8: {}", title)).build();
  rl.set_trace_log(TraceLogLevel::LOG_ERROR);
  rl.set_target_fps(TARGET_FPS);
  (rl, thread)
}

/// Print a single pixel in the position (x, y)
fn print_pixel(d: &mut RaylibDrawHandle, x: usize, y: usize) {
  let upscale_factor = UPSCALE_FACTOR.read().unwrap().clone();
  let x_pos = (x * upscale_factor) as i32;
  let y_pos = (y * upscale_factor) as i32;
  let size = upscale_factor as i32;
  d.draw_rectangle(x_pos, y_pos, size, size, PIXEL_COLOR);
}

/// Draw the current state of the emulator.
pub fn draw_display(d: &mut RaylibDrawHandle, emu: &Emulator) {
  d.clear_background(BG_COLOR);
  for y in 0..DISPLAY_HEIGHT {
    for x in 0..DISPLAY_WIDTH {
      if emu.display_val(x, y) {
        print_pixel(d, x, y);
      }
    }
  }
}

/// Return an array of bools for true/false for the CHIP-8 keypad.
///
/// KEYPAD     KEYBOARD\
/// 1 2 3 C -> 1 2 3 4\
/// 4 5 6 D -> q w e r\
/// 7 8 9 E -> a s d f\
/// A 0 B F -> z x c v
pub fn get_input(rl: &mut RaylibHandle) -> [bool; 16] {
  let mut keys = [false; 16];
  if rl.is_key_down(KeyboardKey::KEY_ONE) {
    keys[0x1] = true;
  }
  if rl.is_key_down(KeyboardKey::KEY_TWO) {
    keys[0x2] = true;
  }
  if rl.is_key_down(KeyboardKey::KEY_THREE) {
    keys[0x3] = true;
  }
  if rl.is_key_down(KeyboardKey::KEY_FOUR) {
    keys[0xC] = true;
  }
  if rl.is_key_down(KeyboardKey::KEY_Q) {
    keys[0x4] = true;
  }
  if rl.is_key_down(KeyboardKey::KEY_W) {
    keys[0x5] = true;
  }
  if rl.is_key_down(KeyboardKey::KEY_E) {
    keys[0x6] = true;
  }
  if rl.is_key_down(KeyboardKey::KEY_R) {
    keys[0xD] = true;
  }
  if rl.is_key_down(KeyboardKey::KEY_A) {
    keys[0x7] = true;
  }
  if rl.is_key_down(KeyboardKey::KEY_S) {
    keys[0x8] = true;
  }
  if rl.is_key_down(KeyboardKey::KEY_D) {
    keys[0x9] = true;
  }
  if rl.is_key_down(KeyboardKey::KEY_F) {
    keys[0xE] = true;
  }
  if rl.is_key_down(KeyboardKey::KEY_Z) {
    keys[0xA] = true;
  }
  if rl.is_key_down(KeyboardKey::KEY_X) {
    keys[0x0] = true;
  }
  if rl.is_key_down(KeyboardKey::KEY_C) {
    keys[0xB] = true;
  }
  if rl.is_key_down(KeyboardKey::KEY_V) {
    keys[0xF] = true;
  }

  keys
}
