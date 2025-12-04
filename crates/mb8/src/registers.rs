use std::fmt::Display;

use mb8_isa::{registers::Register, REGISTERS_COUNT};

/// API for accessing and manipulating the registers.
#[derive(Debug)]
pub struct Registers {
    /// General purpose registers.
    pub registers: [u8; REGISTERS_COUNT],
}

impl Default for Registers {
    fn default() -> Self {
        let mut registers = [0; REGISTERS_COUNT];
        registers[0xD] = 0xBF;
        registers[0xE] = 0xFF;
        Self { registers }
    }
}

impl Registers {
    /// Write a value to a register.
    pub fn write(&mut self, register: impl Into<Register>, value: u8) {
        let register = register.into();
        match register {
            Register::R0 | Register::A => self.registers[0x0] = value,
            Register::R1 => self.registers[0x1] = value,
            Register::R2 => self.registers[0x2] = value,
            Register::R3 => self.registers[0x3] = value,
            Register::R4 => self.registers[0x4] = value,
            Register::R5 => self.registers[0x5] = value,
            Register::R6 => self.registers[0x6] = value,
            Register::R7 => self.registers[0x7] = value,
            Register::R8 => self.registers[0x8] = value,
            Register::R9 | Register::IH => self.registers[0x9] = value,
            Register::R10 | Register::IL => self.registers[0xA] = value,
            Register::R11 | Register::FPH => self.registers[0xB] = value,
            Register::R12 | Register::FPL => self.registers[0xC] = value,
            Register::R13 | Register::SPH => self.registers[0xD] = value,
            Register::R14 | Register::SPL => self.registers[0xE] = value,
            Register::R15 | Register::F => self.registers[0xF] = value,
        }
    }

    /// Read a value from a register.
    #[must_use]
    pub fn read(&self, register: Register) -> u8 {
        match register {
            Register::R0 | Register::A => self.registers[0x0],
            Register::R1 => self.registers[0x1],
            Register::R2 => self.registers[0x2],
            Register::R3 => self.registers[0x3],
            Register::R4 => self.registers[0x4],
            Register::R5 => self.registers[0x5],
            Register::R6 => self.registers[0x6],
            Register::R7 => self.registers[0x7],
            Register::R8 => self.registers[0x8],
            Register::R9 | Register::IH => self.registers[0x9],
            Register::R10 | Register::IL => self.registers[0xA],
            Register::R11 | Register::FPH => self.registers[0xB],
            Register::R12 | Register::FPL => self.registers[0xC],
            Register::R13 | Register::SPH => self.registers[0xD],
            Register::R14 | Register::SPL => self.registers[0xE],
            Register::R15 | Register::F => self.registers[0xF],
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
        write!(f, "F={}\t", self.read(Register::F))
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
