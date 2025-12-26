use crate::context::SymbolId;

#[derive(Debug)]
pub struct IRProgram {
    pub functions: Vec<IRFunction>,
}

#[derive(Debug)]
pub struct IRFunction {
    pub id: SymbolId,
    pub basic_blocks: Vec<BasicBlock>,
}

#[derive(Debug, Clone, Copy)]
pub struct BasicBlockId(pub usize);

#[derive(Debug)]
pub struct BasicBlock {
    pub id: BasicBlockId,
    pub terminator: BasicBlockTerminator,
    pub instructions: Vec<IRInstruction>,
    pub successors: Vec<BasicBlockId>,
    pub predecessors: Vec<BasicBlockId>,
    pub stack_in: usize,
    pub stack_out: usize,
}

#[derive(Debug)]
pub enum BasicBlockTerminator {
    Branch {
        then_branch: BasicBlockId,
        else_branch: BasicBlockId,
    },
    Jmp {
        next: BasicBlockId,
    },
    Ret {
        void: bool,
    },
}

#[derive(Debug)]
pub enum IRInstruction {
    LoadImm { value: u16, width: u8 },
    PushVar { symbol: SymbolId, width: u8 },
    StoreVar { symbol: SymbolId, width: u8 },
    Add { width: u8 },
    Sub { width: u8 },
    Mul { width: u8 },
    Div { width: u8 },
    Eq { width: u8 },
    Neg { width: u8 },
    Call { symbol: SymbolId, argc: usize },
}
