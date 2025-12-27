use crate::context::SymbolId;

#[derive(Debug, Clone)]
pub struct IRProgram {
    pub functions: Vec<IRFunction>,
    pub globals: Vec<SymbolId>,
}

#[derive(Debug, Clone)]
pub struct IRFunction {
    pub id: SymbolId,
    pub basic_blocks: Vec<BasicBlock>,
    pub params: Vec<SymbolId>,
    pub locals: Vec<SymbolId>,
}

#[derive(Debug, Clone, Copy)]
pub struct BasicBlockId(pub usize);

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: BasicBlockId,
    pub terminator: BasicBlockTerminator,
    pub instructions: Vec<IRInstruction>,
    pub successors: Vec<BasicBlockId>,
    pub predecessors: Vec<BasicBlockId>,
    pub stack_in: usize,
    pub stack_out: usize,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
