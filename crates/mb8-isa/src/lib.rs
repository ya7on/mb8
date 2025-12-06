pub mod decode;
pub mod encode;
pub mod opcodes;
pub mod registers;

/// Represents the size of the RAM in bytes.
pub const RAM_SIZE: usize = 0xC000;
/// Represents the top of the stack in bytes.
pub const STACK_TOP: usize = 0xBFFF;
/// Represents the bottom of the stack in bytes.
pub const STACK_BOTTOM: usize = 0xBF00;
/// Represents the size of the ROM in bytes.
pub const ROM_SIZE: usize = 0x1000;
/// Represents the general purpose registers count of the CPU.
pub const REGISTERS_COUNT: usize = 16;
