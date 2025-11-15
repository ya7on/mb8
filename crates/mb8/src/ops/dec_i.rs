use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn dec_i(&mut self, src: Register) {
        let value = self.registers.read(src);
        let index = self.registers.read(Register::I);
        self.registers.write(Register::I, index.wrapping_sub(value));
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn test_opcode_dec_i() {
        // VM decrements register I
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::I, 0x123);
        vm.registers.write(Register::R0, 0x11);
        vm.execute(&Opcode::DecI { src: Register::R0 });
        assert_eq!(vm.registers.read(Register::I), 0x112);
    }
}
