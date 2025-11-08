use mb8_isa::{decode::decode, opcodes::Opcode, registers::Register, MEMORY_SIZE, STACK_SIZE};

use crate::{mem::Memory, registers::Registers};

/// MB8 Virtual Machine
#[derive(Debug)]
pub struct VirtualMachine {
    pub mem: Memory,
    pub registers: Registers,
    pub halted: bool,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            mem: Memory::default(),
            registers: Registers::default(),
            halted: false,
        }
    }

    /// Load memory into the virtual machine.
    pub fn load_memory(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.mem.write_u8(i as u16 + STACK_SIZE, byte);
        }
    }

    /// Execute a single instruction.
    pub fn execute(&mut self, instruction: &Opcode) {
        match instruction {
            Opcode::Nop => self.nop(),
            Opcode::Halt => self.halt(),
            Opcode::Mov { dst, src } => self.mov(*dst, *src),
            Opcode::Add { dst, src } => self.add(*dst, *src),
            Opcode::Sub { dst, src } => self.sub(*dst, *src),
            Opcode::Ldi { dst, value } => self.ldi(*dst, *value),
            Opcode::Jmp { addr } => self.jmp(*addr),
            Opcode::Jz { addr } => self.jz(*addr),
            Opcode::Jnz { addr } => self.jnz(*addr),
            Opcode::Call { addr } => self.call(*addr),
            Opcode::Ret => self.ret(),
        }
    }

    pub fn step(&mut self) {
        let pc = self.registers.read(Register::PC);
        self.registers.write(Register::PC, pc.saturating_add(2));

        if usize::from(pc) >= MEMORY_SIZE - 1 {
            self.halted = true;
            return;
        }

        let binary_instruction = [self.mem.read_u8(pc), self.mem.read_u8(pc + 1)];
        let Some(opcode) = decode(u16::from_be_bytes(binary_instruction)) else {
            self.halted = true;
            return;
        };

        self.execute(&opcode);
    }

    /// Execute a program.
    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm() {
        let mut vm = VirtualMachine::new();
        vm.load_memory(&[0x00, 0x00, 0x01, 0x00]);
        vm.run();
        assert_eq!(vm.registers.read(Register::PC), STACK_SIZE as u16 + 4);
    }

    #[test]
    fn test_end_of_memory() {
        let mut vm = VirtualMachine::new();
        vm.registers.write(Register::PC, 4095);
        vm.step();
        assert!(vm.halted);
    }

    #[test]
    fn test_invalid_opcode() {
        let mut vm = VirtualMachine::new();
        vm.load_memory(&[0xFF]);
        vm.step();
        assert!(vm.halted);
    }
}
