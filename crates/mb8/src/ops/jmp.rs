use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn jmp(&mut self, hi: Register, lo: Register) {
        let hi = self.registers.read(hi);
        let lo = self.registers.read(lo);
        let addr = u16::from_be_bytes([hi, lo]);
        self.program_counter = addr;
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn jumps_to_absolute_address() {
        // VM jumps to a specific address
        let mut vm = VirtualMachine::default();
        vm.program_counter = 0x100;
        vm.registers.write(Register::R0, 0x02);
        vm.registers.write(Register::R1, 0x00);
        vm.execute(&Opcode::Jmp {
            hi: Register::R0,
            lo: Register::R1,
        });
        assert_eq!(vm.program_counter, 0x200);
    }
}
