use mb8_isa::{registers::Register, STACK_BOTTOM};

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn push(&mut self, src: Register) {
        let mut stack_pointer = u16::from_be_bytes([
            self.registers.read(Register::SPH),
            self.registers.read(Register::SPL),
        ]);
        let value = self.registers.read(src);

        self.devices.write(stack_pointer, value);

        stack_pointer -= 1;

        let [sp_hi, sp_lo] = stack_pointer.to_be_bytes();
        self.registers.write(Register::SPH, sp_hi);
        self.registers.write(Register::SPL, sp_lo);

        if stack_pointer - 1 <= STACK_BOTTOM as u16 {
            self.halted = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn pushes_byte_onto_stack() {
        // VM pushes a value onto the stack
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 0x45);
        vm.execute(&Opcode::Push { src: Register::R0 });
        assert_eq!(
            (
                vm.registers.read(Register::SPH),
                vm.registers.read(Register::SPL)
            ),
            (0xBF, 0xFE)
        );
        assert_eq!(vm.devices.read(0xBFFF), 0x45);
    }

    #[test]
    fn halts_on_stack_overflow() {
        // VM halts when the stack overflows
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::SPH, 0xBF);
        vm.registers.write(Register::SPL, 0x00);
        vm.registers.write(Register::R0, 0x45);
        vm.execute(&Opcode::Push { src: Register::R0 });
        assert!(vm.halted);
    }
}
