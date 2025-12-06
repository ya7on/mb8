use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn ldi(&mut self, dst: Register, value: u8) {
        self.registers.write(dst, value);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn loads_immediate_value() {
        // VM puts the value into the register
        let mut vm = VirtualMachine::default();
        vm.execute(&Opcode::Ldi {
            dst: Register::R0,
            value: 0x55,
        });
        assert_eq!(vm.registers.read(Register::R0), 0x55);
    }
}
