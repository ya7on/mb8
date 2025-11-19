use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn halt(&mut self) {
        self.halted = true;
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn halts_execution() {
        // VM halts execution when HALT opcode is encountered
        let mut vm = VirtualMachine::default();
        assert!(!vm.halted);
        vm.execute(&Opcode::Halt);
        assert!(vm.halted);
    }
}
