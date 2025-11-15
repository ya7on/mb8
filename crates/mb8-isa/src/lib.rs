pub mod decode;
pub mod encode;
pub mod opcodes;
pub mod registers;

/// MB8 Machine memory bank size
pub const MEMORY_BANK_SIZE: usize = 4096;
/// MB8 Machine stack size
pub const STACK_SIZE: u16 = 256;
/// MB8 Machine graphic buffer size
pub const GRAPHIC_BUFFER_SIZE: usize = 256;
/// Represents the general purpose registers count of the CPU.
pub const GENERAL_PURPOSE_REGISTERS_COUNT: usize = 8;
