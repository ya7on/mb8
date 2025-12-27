use crate::ir::instructions::{BasicBlockId, IRInstruction};

use super::instructions::{BasicBlock, BasicBlockTerminator};

#[derive(Debug, Default)]
pub struct BasicBlockTable {
    pub next_id: usize,
    pub basic_blocks: Vec<BasicBlockBuilder>,

    pub break_stack: Vec<BasicBlockId>,
    pub continue_stack: Vec<BasicBlockId>,
}

impl BasicBlockTable {
    pub fn bb(&mut self) -> BasicBlockBuilder {
        let builder = BasicBlockBuilder::new(BasicBlockId(self.next_id));
        self.next_id += 1;
        builder
    }

    pub fn push_break(&mut self, id: BasicBlockId) {
        self.break_stack.push(id);
    }

    pub fn pop_break(&mut self) -> Option<BasicBlockId> {
        self.break_stack.pop()
    }

    #[must_use]
    pub fn top_break(&self) -> Option<BasicBlockId> {
        self.break_stack.last().copied()
    }

    pub fn push_continue(&mut self, id: BasicBlockId) {
        self.continue_stack.push(id);
    }

    pub fn pop_continue(&mut self) -> Option<BasicBlockId> {
        self.continue_stack.pop()
    }

    #[must_use]
    pub fn top_continue(&self) -> Option<BasicBlockId> {
        self.continue_stack.last().copied()
    }
}

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
            instructions: Vec::new(),
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
            successors: Vec::new(),
            predecessors: Vec::new(),
            stack_in: 0,
            stack_out: 0,
        }
    }
}
