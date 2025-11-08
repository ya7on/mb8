use mb8_isa::{opcodes::Opcode, registers::Register};

use crate::{parser::parse, registers::Registers};

const MEMORY_SIZE: usize = 65536;

/// MB8 Virtual Machine
#[derive(Debug)]
pub struct VirtualMachine {
    pub mem: Box<[u8; MEMORY_SIZE]>,
    pub registers: Registers,
    pub halted: bool,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            #[allow(clippy::unwrap_used)]
            mem: vec![0; MEMORY_SIZE].into_boxed_slice().try_into().unwrap(),
            registers: Registers::default(),
            halted: false,
        }
    }

    /// Load memory into the virtual machine.
    pub fn load_memory(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.mem[i] = byte;
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
        }
    }

    pub fn step(&mut self) {
        let pc = self.registers.read(Register::PC);
        self.registers.write(Register::PC, pc.saturating_add(2));

        if usize::from(pc) >= MEMORY_SIZE - 1 {
            self.halted = true;
            return;
        }

        let binary_instruction = [self.mem[pc as usize], self.mem[pc as usize + 1]];
        let Some(opcode) = parse(u16::from_be_bytes(binary_instruction)) else {
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
        assert_eq!(vm.registers.read(Register::PC), 4);
    }
}
