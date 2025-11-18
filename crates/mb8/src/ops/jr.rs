use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn jr(&mut self, offset: i8) {
        let program_counter = self.registers.read(Register::PC);
        let new_pc = program_counter.wrapping_add_signed(offset as i16);
        self.registers.write(Register::PC, new_pc);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn jumps_forward_relative() {
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::PC, 0x0100);

        vm.execute(&Opcode::Jr { offset: 0x10 });

        assert_eq!(vm.registers.read(Register::PC), 0x0110);
    }

    #[test]
    fn jumps_backward_relative() {
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::PC, 0x0100);

        vm.execute(&Opcode::Jr { offset: -0x10 });

        assert_eq!(vm.registers.read(Register::PC), 0x00F0);
    }
}
