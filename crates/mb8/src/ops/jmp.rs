use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn jmp(&mut self, hi: Register, lo: Register) {
        let hi = self.registers.read(hi);
        let lo = self.registers.read(lo);
        let addr = (hi << 8) | lo;
        self.registers.write(Register::PC, addr);
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
        vm.registers.write(Register::PC, 0x100);
        vm.registers.write(Register::R0, 0x02);
        vm.registers.write(Register::R1, 0x00);
        vm.execute(&Opcode::Jmp {
            hi: Register::R0,
            lo: Register::R1,
        });
        assert_eq!(vm.registers.read(Register::PC), 0x200);
    }
}
