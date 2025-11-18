use mb8_isa::registers::{flags, Register};

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn jcr(&mut self, offset: i8) {
        let f_register = self.registers.read(Register::F) as u8;
        if f_register & flags::C_FLAG != 0 {
            let program_counter = self.registers.read(Register::PC);
            let new_pc = program_counter.wrapping_add_signed(offset as i16);
            self.registers.write(Register::PC, new_pc);
        }
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn jumps_when_carry_flag_set() {
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::PC, 0x0100);
        vm.registers.write(Register::F, flags::C_FLAG as u16);
        vm.execute(&Opcode::Jcr { offset: 0x20 });
        assert_eq!(vm.registers.read(Register::PC), 0x0120);
    }

    #[test]
    fn does_not_jump_when_carry_flag_clear() {
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::PC, 0x0100);
        vm.registers.write(Register::F, 0);
        vm.execute(&Opcode::Jcr { offset: 0x20 });
        assert_eq!(vm.registers.read(Register::PC), 0x0100);
    }

    #[test]
    fn jumps_backward_when_carry_flag_set() {
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::PC, 0x0100);
        vm.registers.write(Register::F, flags::C_FLAG as u16);
        vm.execute(&Opcode::Jcr { offset: -0x20 });
        assert_eq!(vm.registers.read(Register::PC), 0x00E0);
    }
}
