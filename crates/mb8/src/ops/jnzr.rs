use mb8_isa::registers::{flags, Register};

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn jnzr(&mut self, offset: i8) {
        let f_register = self.registers.read(Register::F);
        if f_register & flags::Z_FLAG == 0 {
            let program_counter = self.program_counter;
            let new_pc = program_counter.wrapping_add_signed(offset as i16);
            self.program_counter = new_pc;
        }
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn jumps_when_zero_flag_clear() {
        let mut vm = VirtualMachine::default();
        vm.program_counter = 0x0100;
        vm.registers.write(Register::F, 0);
        vm.execute(&Opcode::Jnzr { offset: 0x10 });
        assert_eq!(vm.program_counter, 0x0110);
    }

    #[test]
    fn does_not_jump_when_zero_flag_set() {
        let mut vm = VirtualMachine::default();
        vm.program_counter = 0x0100;
        vm.registers.write(Register::F, flags::Z_FLAG);
        vm.execute(&Opcode::Jnzr { offset: 0x10 });
        assert_eq!(vm.program_counter, 0x0100);
    }

    #[test]
    fn jumps_backward_when_zero_flag_clear() {
        let mut vm = VirtualMachine::default();
        vm.program_counter = 0x0100;
        vm.registers.write(Register::F, 0);
        vm.execute(&Opcode::Jnzr { offset: -0x20 });
        assert_eq!(vm.program_counter, 0x00E0);
    }
}
