use mb8_isa::registers::{flags, Register};

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn shl(&mut self, dst: Register, src: Register) {
        let a = self.registers.read(dst);
        let b = self.registers.read(src);
        let result = a.wrapping_shl(b as u32);

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
    fn test_opcode_shl() {
        // VM executes SHL operation
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 0b1011_1100);
        vm.registers.write(Register::R1, 2);
        vm.execute(&Opcode::Shl {
            dst: Register::R0,
            src: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::R0), 0b1111_0000);
    }
}
