use mb8_isa::registers::{flags, Register};

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn xor(&mut self, dst: Register, src: Register) {
        let a = self.registers.read(dst);
        let b = self.registers.read(src);
        let result = a ^ b;

        let mut f_register = 0;
        if result as u8 == 0 {
            f_register |= flags::Z_FLAG;
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
    fn computes_bitwise_xor() {
        // VM executes XOR operation
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 0b1100_1010);
        vm.registers.write(Register::R1, 0b1111_0000);
        vm.execute(&Opcode::Xor {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::R0), 0b0011_1010);
    }

    #[test]
    fn sets_zero_flag_on_xor() {
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 0xAA);
        vm.registers.write(Register::R1, 0xAA);
        vm.execute(&Opcode::Xor {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::F), flags::Z_FLAG as u16);
    }

    #[test]
    fn sets_negative_flag_on_xor() {
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 0x80);
        vm.registers.write(Register::R1, 0x00);
        vm.execute(&Opcode::Xor {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::F), flags::N_FLAG as u16);
    }
}
