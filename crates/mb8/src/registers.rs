use mb8_isa::registers::Register;

/// Represents the general purpose registers count of the CPU.
const GENERAL_PURPOSE_REGISTERS_COUNT: usize = 8;

/// API for accessing and manipulating the registers.
#[derive(Debug, Default)]
pub struct Registers {
    /// General purpose registers.
    pub general_purpose: [u8; GENERAL_PURPOSE_REGISTERS_COUNT],
    /// Program counter register.
    pub program_counter: u16,
    /// Stack pointer register.
    pub stack_pointer: u8,
    /// Flag register.
    pub flag: u8,
}

impl Registers {
    /// Write a value to a register.
    pub fn write(&mut self, register: Register, value: u16) {
        match register {
            Register::R0 => self.general_purpose[0] = u8::try_from(value).unwrap_or(0),
            Register::R1 => self.general_purpose[1] = u8::try_from(value).unwrap_or(0),
            Register::R2 => self.general_purpose[2] = u8::try_from(value).unwrap_or(0),
            Register::R3 => self.general_purpose[3] = u8::try_from(value).unwrap_or(0),
            Register::R4 => self.general_purpose[4] = u8::try_from(value).unwrap_or(0),
            Register::R5 => self.general_purpose[5] = u8::try_from(value).unwrap_or(0),
            Register::R6 => self.general_purpose[6] = u8::try_from(value).unwrap_or(0),
            Register::R7 => self.general_purpose[7] = u8::try_from(value).unwrap_or(0),
            Register::F => self.flag = u8::try_from(value).unwrap_or(0),
            Register::PC => self.program_counter = value,
            Register::SP => self.stack_pointer = u8::try_from(value).unwrap_or(0),
        }
    }

    /// Read a value from a register.
    pub fn read(&self, register: Register) -> u16 {
        match register {
            Register::R0 => u16::from(self.general_purpose[0]),
            Register::R1 => u16::from(self.general_purpose[1]),
            Register::R2 => u16::from(self.general_purpose[2]),
            Register::R3 => u16::from(self.general_purpose[3]),
            Register::R4 => u16::from(self.general_purpose[4]),
            Register::R5 => u16::from(self.general_purpose[5]),
            Register::R6 => u16::from(self.general_purpose[6]),
            Register::R7 => u16::from(self.general_purpose[7]),
            Register::F => u16::from(self.flag),
            Register::PC => self.program_counter,
            Register::SP => u16::from(self.stack_pointer),
        }
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
