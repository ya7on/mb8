use mb8_isa::registers::{flags, Register};

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn shr(&mut self, dst: Register, src: Register) {
        let a = self.registers.read(dst);
        let b = self.registers.read(src);
        let result = a.wrapping_shr(b as u32);

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
    fn shifts_right() {
        // VM executes SHR operation
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 0b1111_0011);
        vm.registers.write(Register::R1, 2);
        vm.execute(&Opcode::Shr {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::R0), 0b0011_1100);
    }

    #[test]
    fn sets_zero_flag_on_shift_right() {
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 0);
        vm.registers.write(Register::R1, 4);
        vm.execute(&Opcode::Shr {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::F), flags::Z_FLAG as u16);
    }

    #[test]
    fn keeps_carry_flag_cleared_on_shift_right() {
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 0xFF);
        vm.registers.write(Register::R1, 1);
        vm.execute(&Opcode::Shr {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::F) & flags::C_FLAG as u16, 0);
    }

    #[test]
    fn sets_negative_flag_on_shift_right_noop() {
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 0x80);
        vm.registers.write(Register::R1, 0);
        vm.execute(&Opcode::Shr {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::F), flags::N_FLAG as u16);
    }
}
