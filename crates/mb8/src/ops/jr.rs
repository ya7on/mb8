use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn jr(&mut self, offset: i8) {
        let program_counter = self.program_counter;
        let new_pc = program_counter.wrapping_add_signed(offset as i16);
        self.program_counter = new_pc;
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn jumps_forward_relative() {
        let mut vm = VirtualMachine::default();
        vm.program_counter = 0x0100;

        vm.execute(&Opcode::Jr { offset: 0x10 });

        assert_eq!(vm.program_counter, 0x0110);
    }

    #[test]
    fn jumps_backward_relative() {
        let mut vm = VirtualMachine::default();
        vm.program_counter = 0x0100;

        vm.execute(&Opcode::Jr { offset: -0x10 });

        assert_eq!(vm.program_counter, 0x00F0);
    }
}
