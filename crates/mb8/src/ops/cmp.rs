use mb8_isa::registers::{flags, Register};

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn cmp(&mut self, dst: Register, src: Register) {
        let a = self.registers.read(dst);
        let b = self.registers.read(src);
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

        self.registers.write(Register::F, f_register);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn performs_compare() {
        // VM Compares two registers and stores the result in a third register
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 5);
        vm.registers.write(Register::R1, 3);
        vm.execute(&Opcode::Cmp {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::R0), 5);
    }

    #[test]
    fn clears_flags_before_cmp() {
        // VM clear the flags register before executing SUB instruction
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::F, 0xFF);
        vm.registers.write(Register::R0, 2);
        vm.registers.write(Register::R1, 1);
        vm.execute(&Opcode::Cmp {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::R1), 1);
        assert_eq!(vm.registers.read(Register::F), 0);
    }

    #[test]
    fn sets_zero_flag_after_cmp() {
        // VM clear the flags register before executing CMP instruction
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::F, 0xFF);
        vm.registers.write(Register::R0, 1);
        vm.registers.write(Register::R1, 1);
        vm.execute(&Opcode::Cmp {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::F), flags::Z_FLAG);
    }

    #[test]
    fn wraps_and_sets_carry_on_cmp() {
        // VM handles compare overflow by wrapping around and setting the carry flag
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 1);
        vm.registers.write(Register::R1, 2);
        vm.execute(&Opcode::Cmp {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(
            vm.registers.read(Register::F),
            flags::N_FLAG | flags::C_FLAG
        );
    }

    #[test]
    fn sets_zero_flag_on_compare() {
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 0x2A);
        vm.registers.write(Register::R1, 0x2A);
        vm.execute(&Opcode::Cmp {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::F), flags::Z_FLAG);
    }

    #[test]
    fn sets_carry_flag_on_compare() {
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 55);
        vm.registers.write(Register::R1, 200);
        vm.execute(&Opcode::Cmp {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::F), flags::C_FLAG);
    }

    #[test]
    fn sets_negative_flag_on_compare() {
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 0x90);
        vm.registers.write(Register::R1, 0x10);
        vm.execute(&Opcode::Cmp {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::F), flags::N_FLAG);
    }
}
