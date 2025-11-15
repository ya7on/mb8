use mb8_isa::{
    registers::{flags, Register},
    STACK_SIZE,
};

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn jnz(&mut self, addr: u16) {
        let f_register = self.registers.read(Register::F) as u8;
        if f_register & flags::Z_FLAG == 0 {
            self.registers.write(Register::PC, addr + STACK_SIZE);
        }
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn test_opcode_jnz() {
        // VM jumps to a specific address
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::PC, 0x100);
        vm.registers.write(Register::F, 0);
        vm.execute(&Opcode::Jnz { addr: 0x100 });
        assert_eq!(vm.registers.read(Register::PC), 0x200);
    }

    #[test]
    fn test_opcode_jnz_zero() {
        // VM does not jump to a specific address
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::PC, 0x100);
        vm.registers.write(Register::F, flags::Z_FLAG as u16);
        vm.execute(&Opcode::Jnz { addr: 0x200 });
        assert_eq!(vm.registers.read(Register::PC), 0x100);
    }
}
