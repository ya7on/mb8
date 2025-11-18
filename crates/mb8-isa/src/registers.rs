//! Register definitions for the MB8 VM.

pub mod flags {
    /// Zero flag for the flag register
    pub const Z_FLAG: u8 = 0b0000_0001;
    /// Negative flag for the flag register
    pub const N_FLAG: u8 = 0b0000_0010;
    /// Carry flag for the flag register
    pub const C_FLAG: u8 = 0b0000_0100;
    // /// Overflow flag for the flag register
    // pub const V_FLAG: u8 = 0b0000_1000;
}

/// List of registers supported by the MB8 VM.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    /// General-purpose register 0
    R0,
    /// General-purpose register 1
    R1,
    /// General-purpose register 2
    R2,
    /// General-purpose register 3
    R3,
    /// General-purpose register 4
    R4,
    /// General-purpose register 5
    R5,
    /// General-purpose register 6
    R6,
    /// General-purpose register 7
    R7,
    /// Stack pointer
    SP,
    /// Program counter
    PC,
    /// Flag register
    F,
}
