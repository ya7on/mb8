use mb8_isa::registers::{flags, Register};

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn add(&mut self, dst: Register, src: Register) {
        let a = self.registers.read(dst);
        let b = self.registers.read(src);
        let result = a + b;

        let mut f_register = 0;
        if result as u8 == 0 {
            f_register |= flags::Z_FLAG;
        }
        if result > 255 {
            f_register |= flags::C_FLAG;
        }
        if (result & 0x80) != 0 {
            f_register |= flags::N_FLAG;
        }

        self.registers.write(dst, result);
        self.registers.write(Register::F, f_register as u16);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn test_opcode_add() {
        // VM adds two registers and stores the result in a third register
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 5);
        vm.registers.write(Register::R1, 3);
        vm.execute(&Opcode::Add {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::R0), 8);
    }

    #[test]
    fn test_opcode_add_clear_flags() {
        // VM clear the flags register before executing ADD instruction
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::F, 0xFF);
        vm.registers.write(Register::R0, 5);
        vm.registers.write(Register::R1, 3);
        vm.execute(&Opcode::Add {
            dst: Register::R0,
            src: Register::R1,
        });
        vm.registers.write(Register::R0, 8);
        assert_eq!(vm.registers.read(Register::F), 0);
    }

    #[test]
    fn test_opcode_add_zero() {
        // VM clear the flags register before executing ADD instruction
        let mut vm = VirtualMachine::default();
        vm.execute(&Opcode::Add {
            dst: Register::R0,
            src: Register::R1,
        });
        vm.registers.write(Register::R0, 0);
        assert_eq!(vm.registers.read(Register::F), flags::Z_FLAG as u16);
    }

    #[test]
    fn test_opcode_add_overflow() {
        // VM handles addition overflow by wrapping around and setting the carry flag
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 255);
        vm.registers.write(Register::R1, 255);
        vm.execute(&Opcode::Add {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::R0), 254);
        assert_eq!(
            vm.registers.read(Register::F),
            (flags::C_FLAG | flags::N_FLAG) as u16
        );
    }
}
