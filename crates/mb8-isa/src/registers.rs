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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Register {
    #[must_use]
    pub const fn physical_index(&self) -> u8 {
        match self {
            Self::R0 | Self::A => 0,
            Self::R1 => 1,
            Self::R2 => 2,
            Self::R3 => 3,
            Self::R4 => 4,
            Self::R5 => 5,
            Self::R6 => 6,
            Self::R7 => 7,
            Self::R8 => 8,
            Self::R9 | Self::IH => 9,
            Self::R10 | Self::IL => 10,
            Self::R11 | Self::FPH => 11,
            Self::R12 | Self::FPL => 12,
            Self::R13 | Self::SPH => 13,
            Self::R14 | Self::SPL => 14,
            Self::R15 | Self::F => 15,
        }
    }
}
