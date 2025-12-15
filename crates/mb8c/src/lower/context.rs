use crate::ir::{BasicBlockId, VirtualRegister};

use super::bb::BasicBlockBuilder;

#[derive(Debug, Default)]
pub struct LowerContext {
    pub next_bb: usize,
    pub next_register: usize,
}

impl LowerContext {
    pub fn bb(&mut self) -> BasicBlockBuilder {
        let id = self.next_bb;
        self.next_bb += 1;
        BasicBlockBuilder::new(BasicBlockId(id))
    }

    pub fn vreg(&mut self) -> VirtualRegister {
        let id = self.next_register;
        self.next_register += 1;
        VirtualRegister(id)
    }
}
