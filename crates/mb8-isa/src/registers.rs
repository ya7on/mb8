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
    /// General-purpose register 8
    R8,
    /// General-purpose register 9
    R9,
    /// General-purpose register 10
    R10,
    /// General-purpose register 11
    R11,
    /// General-purpose register 12
    R12,
    /// General-purpose register 13
    R13,
    /// General-purpose register 14
    R14,
    /// General-purpose register 15
    R15,

    /** Aliases */

    /// Accumulator register
    A,
    /// Index high byte
    IH,
    /// Index low byte
    IL,
    /// Frame pointer high byte
    FPH,
    /// Frame pointer low byte
    FPL,
    /// Stack pointer high byte
    SPH,
    /// Stack pointer low byte
    SPL,
    /// Flag register
    F,
}
