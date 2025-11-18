use std::fmt::Display;

use mb8_isa::{registers::Register, GENERAL_PURPOSE_REGISTERS_COUNT, STACK_TOP};

/// API for accessing and manipulating the registers.
#[derive(Debug)]
pub struct Registers {
    /// General purpose registers.
    pub general_purpose: [u8; GENERAL_PURPOSE_REGISTERS_COUNT],
    /// Program counter register.
    pub program_counter: u16,
    /// Stack pointer register.
    pub stack_pointer: u16,
    /// Flag register.
    pub flag: u8,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            general_purpose: [0; GENERAL_PURPOSE_REGISTERS_COUNT],
            program_counter: 0xE000,
            stack_pointer: STACK_TOP as u16,
            flag: 0,
        }
    }
}

impl Registers {
    /// Write a value to a register.
    pub fn write(&mut self, register: Register, value: u16) {
        match register {
            Register::R0 => self.general_purpose[0] = value as u8,
            Register::R1 => self.general_purpose[1] = value as u8,
            Register::R2 => self.general_purpose[2] = value as u8,
            Register::R3 => self.general_purpose[3] = value as u8,
            Register::R4 => self.general_purpose[4] = value as u8,
            Register::R5 => self.general_purpose[5] = value as u8,
            Register::R6 => self.general_purpose[6] = value as u8,
            Register::R7 => self.general_purpose[7] = value as u8,
            Register::F => self.flag = value as u8,
            Register::PC => self.program_counter = value,
            Register::SP => self.stack_pointer = value,
        }
    }

    /// Read a value from a register.
    #[must_use]
    pub fn read(&self, register: Register) -> u16 {
        match register {
            Register::R0 => self.general_purpose[0] as u16,
            Register::R1 => self.general_purpose[1] as u16,
            Register::R2 => self.general_purpose[2] as u16,
            Register::R3 => self.general_purpose[3] as u16,
            Register::R4 => self.general_purpose[4] as u16,
            Register::R5 => self.general_purpose[5] as u16,
            Register::R6 => self.general_purpose[6] as u16,
            Register::R7 => self.general_purpose[7] as u16,
            Register::F => self.flag as u16,
            Register::PC => self.program_counter,
            Register::SP => self.stack_pointer,
        }
    }
}

impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "R0={}\t", self.read(Register::R0))?;
        write!(f, "R1={}\t", self.read(Register::R1))?;
        write!(f, "R2={}\t", self.read(Register::R2))?;
        write!(f, "R3={}\t", self.read(Register::R3))?;
        write!(f, "R4={}\t", self.read(Register::R4))?;
        write!(f, "R5={}\t", self.read(Register::R5))?;
        write!(f, "R6={}\t", self.read(Register::R6))?;
        write!(f, "R7={}\t", self.read(Register::R7))?;
        write!(f, "F={}\t", self.read(Register::F))?;
        write!(f, "PC={}\t", self.read(Register::PC))?;
        write!(f, "SP={}", self.read(Register::SP))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_general_purpose_register() {
        let mut registers = Registers::default();
        for gpr in [
            Register::R0,
            Register::R1,
            Register::R2,
            Register::R3,
            Register::R4,
            Register::R5,
            Register::R6,
            Register::R7,
        ] {
            registers.write(gpr, 42);
            assert_eq!(registers.read(gpr), 42);
        }
    }
}
