use crate::vm::VirtualMachine;

impl VirtualMachine {
    #[allow(clippy::unused_self)]
    pub fn nop(&mut self) {
        // No operation
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn executes_nop() {
        // VM does nothing on NOP
        let mut vm = VirtualMachine::default();
        vm.execute(&Opcode::Nop);
    }
}
