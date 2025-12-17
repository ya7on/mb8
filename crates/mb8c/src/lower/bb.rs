use crate::ir::{BasicBlock, BasicBlockId, BasicBlockTerminator, IRInstruction};

#[derive(Debug)]
pub struct BasicBlockBuilder {
    pub id: BasicBlockId,
    pub instructions: Vec<IRInstruction>,
}

impl BasicBlockBuilder {
    #[must_use]
    pub fn new(id: BasicBlockId) -> Self {
        Self {
            id,
            instructions: vec![],
        }
    }

    #[must_use]
    pub fn id(&self) -> BasicBlockId {
        self.id
    }

    pub fn emit(&mut self, instruction: IRInstruction) {
        self.instructions.push(instruction);
    }

    #[must_use]
    pub fn build(self, terminator: BasicBlockTerminator) -> BasicBlock {
        BasicBlock {
            id: self.id,
            instructions: self.instructions,
            terminator,
        }
    }
}
