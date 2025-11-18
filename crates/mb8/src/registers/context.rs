use mb8_isa::{registers::Register, GENERAL_PURPOSE_REGISTERS_COUNT};

#[derive(Debug, Default)]
pub struct RegistersContext {
    /// General purpose registers.
    pub general_purpose: [u8; GENERAL_PURPOSE_REGISTERS_COUNT],
    /// Program counter register.
    pub program_counter: u16,
    /// Stack pointer register.
    pub stack_pointer: u8,
    /// Flag register.
    pub flag: u8,
}

impl RegistersContext {
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
            Register::SP => self.stack_pointer = value as u8,
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
            Register::SP => self.stack_pointer as u16,
        }
    }
}
