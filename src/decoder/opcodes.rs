//! opcodes.rs
//! Numerical constants used by the decoder.

// Instruction that are totally unique.
/// 0x1nnn: Set the pc to nnn.
pub const SET_PC: u16 = 0x1000;
/// 0x2nnn: Call subrotine at nnn.
pub const CALL: u16 = 0x2000;
/// 0x3xnn: Skip next instruction if VX == nn.
pub const SE_INMM: u16 = 0x3000;
/// 0x4xnn: Skip next instruction if VX != nn.
pub const SNE_INMM: u16 = 0x4000;
/// 0x5xy0: Skip next instruction if VX == VY.
pub const SE_REG: u16 = 0x5000;
/// 0x6xnn: Put the value nn into reg VX.
pub const LD_INMM: u16 = 0x6000;
/// 0x7xnn: Add the value nn to the value in VX ,storing in VX (no carry).
pub const SUM: u16 = 0x7000;
/// 0x9xy0: Skip next instruction if VX != VY.
pub const SNE_REG: u16 = 0x9000;
/// 0xAnnn: Set the register I to nnn.
pub const LD_I: u16 = 0xA000;
/// 0xBnnn: Jump to the instruction V0 + nnn.
pub const JUMP: u16 = 0xB000;
/// 0xCxnn: Generates a random value and do AND with nn. Store the result in VX.
pub const RAND: u16 = 0xC000;
/// 0xDxyn: Display instruction.
pub const DISPLAY: u16 = 0xD000;

// Instructions with first nibble equal (GROUP 0).
/// 0x00E0: Clear the display.
pub const CLS: u16 = 0x00E0;
/// 0x00EE: Return from a subroutine.
pub const RET: u16 = 0x00EE;

// Instructions with first nibble equal (GROUP 8).
/// 0x8xy0: Store the value in VY in reg VX.
pub const LD_REG: u16 = 0x8000;
/// 0x8xy1: bitwise OR of the values in VX and VY, storing in VX.
pub const OR: u16 = 0x8001;
/// 0x8xy2: bitwise AND of the values in VX and VY, storing in VX.
pub const AND: u16 = 0x8002;
/// 0x8xy3: bitwise XOR of the values in VX and VY, storing in VX.
pub const XOR: u16 = 0x8003;
/// 0x8xy4: Add the values in VX and VY, storing them in VX (with carry in VF).
pub const ADD: u16 = 0x8004;
/// 0x8xy5: Substract VX with VY, storing the value in VX (with underflow in VF).
pub const SUB: u16 = 0x8005;
/// 0x8xy6: Put VY in VX, then do right shift (storing the lost bit in VF).
pub const SHR: u16 = 0x8006;
/// 0x8xy7: 0x8Substract VY with VX, storing the value in VX (with underflow in VF)
pub const SUBN: u16 = 0x8007;
/// 0x8xyE: Put VY in VX, then do left shift (storing the lost bit in VF).
pub const SHL: u16 = 0x800E;

// Instructions with first nibble equal (GROUP E).
/// 0xEx9E: Skip the next instruction if the key in VX is pressed.
pub const SKP: u16 = 0xE09E;
/// 0xExA1: SKip the next instruction if the key in VX is not pressed.
pub const NSKP: u16 = 0xE0A1;

// Instructions with first nibble equal (GROUP F).
/// 0xFx07: Load the delay reg in reg VX.
pub const ST_DELAY: u16 = 0xF007;
/// 0xFx0A: Enter in a infinite loop until a key is pressed, storing it in VX.
pub const WAIT_KEY: u16 = 0xF00A;
/// 0xFx15: Load in the delay reg the value in VX.
pub const LD_DELAY: u16 = 0xF015;
/// 0xFx18: Load in the sound reg the value in VX.
pub const LD_SOUND: u16 = 0xF018;
/// 0xFx1E: Values in I and VX are added, the stored in I.
pub const ADD_I: u16 = 0xF01E;
/// 0xFx29: Set I to the location of the sprite in VX.
pub const LD_SPRITE: u16 = 0xF029;
/// 0xFx33: Store in I, I+1, I+2 the digits in digital of VX.
pub const BCD: u16 = 0xF033;
/// 0xFx55: Store registers V0 through VX starting at location I.
pub const ST_MEM: u16 = 0xF055;
/// 0xFx65: store the values in memory starting in I storing from V0 to VX.
pub const LD_MEM: u16 = 0xF065;
