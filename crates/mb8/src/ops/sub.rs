use mb8_isa::registers::{flags, Register};

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn sub(&mut self, dst: Register, src: Register) {
        let a = self.registers.read(dst) as u8;
        let b = self.registers.read(src) as u8;
        let (result, overflow) = a.overflowing_sub(b);

        let mut f_register = 0;
        if result == 0 {
            f_register |= flags::Z_FLAG;
        }
        if overflow {
            f_register |= flags::C_FLAG;
        }
        if (result & 0x80) != 0 {
            f_register |= flags::N_FLAG;
        }

        self.registers.write(dst, result as u16);
        self.registers.write(Register::F, f_register as u16);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn performs_subtraction() {
        // VM subtracts two registers and stores the result in a third register
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 5);
        vm.registers.write(Register::R1, 3);
        vm.execute(&Opcode::Sub {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::R0), 2);
    }

    #[test]
    fn clears_flags_before_sub() {
        // VM clear the flags register before executing SUB instruction
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::F, 0xFF);
        vm.registers.write(Register::R0, 2);
        vm.registers.write(Register::R1, 1);
        vm.execute(&Opcode::Sub {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::R1), 1);
        assert_eq!(vm.registers.read(Register::F), 0);
    }

    #[test]
    fn sets_zero_flag_after_sub() {
        // VM clear the flags register before executing SUB instruction
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::F, 0xFF);
        vm.registers.write(Register::R0, 1);
        vm.registers.write(Register::R1, 1);
        vm.execute(&Opcode::Sub {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::R0), 0);
        assert_eq!(vm.registers.read(Register::F), flags::Z_FLAG as u16);
    }

    #[test]
    fn wraps_and_sets_carry_on_sub() {
        // VM handles subtraction overflow by wrapping around and setting the carry flag
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 1);
        vm.registers.write(Register::R1, 2);
        vm.execute(&Opcode::Sub {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::R0), 255);
        assert_eq!(
            vm.registers.read(Register::F),
            (flags::N_FLAG | flags::C_FLAG) as u16
        );
    }

    #[test]
    fn sets_zero_flag_on_subtraction() {
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 0x2A);
        vm.registers.write(Register::R1, 0x2A);
        vm.execute(&Opcode::Sub {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::F), flags::Z_FLAG as u16);
    }

    #[test]
    fn sets_carry_flag_on_subtraction() {
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 55);
        vm.registers.write(Register::R1, 200);
        vm.execute(&Opcode::Sub {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::F), flags::C_FLAG as u16);
    }

    #[test]
    fn sets_negative_flag_on_subtraction() {
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 0x90);
        vm.registers.write(Register::R1, 0x10);
        vm.execute(&Opcode::Sub {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::F), flags::N_FLAG as u16);
    }
}
