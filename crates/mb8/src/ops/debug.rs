use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn debug(&mut self) {
        self.debug_break = true;
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
        vm.execute(&Opcode::Debug);
        assert!(vm.halted);
    }
}
