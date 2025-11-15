use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn ldi_i(&mut self, value: u16) {
        self.registers.write(Register::I, value);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn test_opcode_ldi_i() {
        // VM loads value into register I
        let mut vm = VirtualMachine::new();
        vm.execute(&Opcode::LdiI { value: 0x123 });
        assert_eq!(vm.registers.read(Register::I), 0x123);
    }
}
