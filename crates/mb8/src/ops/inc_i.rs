use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn inc_i(&mut self, src: Register) {
        let value = self.registers.read(src);
        let index = self.registers.read(Register::I);
        self.registers.write(Register::I, index.wrapping_add(value));
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn test_opcode_inc_i() {
        // VM increments register I
        let mut vm = VirtualMachine::new();
        vm.registers.write(Register::I, 0x123);
        vm.registers.write(Register::R0, 0x11);
        vm.execute(&Opcode::IncI { src: Register::R0 });
        assert_eq!(vm.registers.read(Register::I), 0x134,);
    }
}
