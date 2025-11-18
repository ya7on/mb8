use std::fmt::Display;

use context::RegistersContext;
use mb8_isa::{registers::Register, BOTS_LIMIT};

use crate::vm::Role;

mod context;

/// API for accessing and manipulating the registers.
#[derive(Debug, Default)]
pub struct Registers {
    current_context: Role,
    host: RegistersContext,
    bots: [RegistersContext; BOTS_LIMIT],
}

impl Registers {
    pub fn switch_context(&mut self, role: Role) {
        self.current_context = role;
    }

    /// Write a value to a register.
    pub fn write(&mut self, register: Register, value: u16) {
        let context = match self.current_context {
            Role::Judge => &mut self.host,
            Role::Bot(id) => &mut self.bots[id as usize],
        };
        context.write(register, value);
    }

    /// Read a value from a register.
    #[must_use]
    pub fn read(&self, register: Register) -> u16 {
        let context = match self.current_context {
            Role::Judge => &self.host,
            Role::Bot(id) => &self.bots[id as usize],
        };
        context.read(register)
    }

    pub fn clear(&mut self) {
        let context = match self.current_context {
            Role::Judge => &mut self.host,
            Role::Bot(id) => &mut self.bots[id as usize],
        };
        *context = RegistersContext::default();
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
