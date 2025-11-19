use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn ret(&mut self) {
        let mut stack_pointer = self.registers.read(Register::SP);
        if stack_pointer + 2 > 0xBFFF {
            self.halted = true;
            return;
        }
        stack_pointer += 1;
        let hi = self.devices.read(stack_pointer);
        stack_pointer += 1;
        let lo = self.devices.read(stack_pointer);
        self.registers.write(Register::SP, stack_pointer);
        self.registers
            .write(Register::PC, u16::from_be_bytes([hi, lo]));
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn restores_pc_from_stack() {
        // VM returns from a subroutine
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::PC, 0x100);
        vm.registers.write(Register::R0, 0x02);
        vm.registers.write(Register::R1, 0x00);
        vm.execute(&Opcode::Call {
            hi: Register::R0,
            lo: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::SP), 0xBFFF - 2);
        assert_eq!(vm.registers.read(Register::PC), 0x200);
        assert_eq!(vm.devices.read(0xBFFF - 1), 0x01);
        assert_eq!(vm.devices.read(0xBFFF), 0x00);
        vm.execute(&Opcode::Ret);
        assert_eq!(vm.registers.read(Register::SP), 0xBFFF);
        assert_eq!(vm.registers.read(Register::PC), 0x100);
    }

    #[test]
    fn halts_on_ret_underflow() {
        // VM returns from a subroutine
        let mut vm = VirtualMachine::default();
        vm.execute(&Opcode::Ret);
        assert_eq!(vm.registers.read(Register::SP), 0xBFFF);
        assert_eq!(vm.registers.read(Register::PC), 0xE000);
        assert!(vm.halted);
    }
}
