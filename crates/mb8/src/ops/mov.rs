use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn mov(&mut self, dst: Register, src: Register) {
        let value = self.registers.read(src);
        self.registers.write(dst, value);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn moves_register_value() {
        // VM moves registers values from source register to destination register
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 42);
        vm.execute(&Opcode::Mov {
            dst: Register::R1,
            src: Register::R0,
        });
        assert_eq!(vm.registers.read(Register::R1), 42);
    }
}
