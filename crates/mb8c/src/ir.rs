#[derive(Debug)]
pub struct IRFunction {
    pub basic_blocks: Vec<BasicBlock>,
}

#[derive(Debug, Clone, Copy)]
pub struct VirtualRegister(pub usize);

#[derive(Debug, Clone, Copy)]
pub struct BasicBlockId(pub usize);

#[derive(Debug)]
pub struct BasicBlock {
    pub id: BasicBlockId,
    pub terminator: BasicBlockTerminator,
    pub instructions: Vec<IRInstruction>,
}

#[derive(Debug)]
pub enum BasicBlockTerminator {
    Branch {
        condition: VirtualRegister,
        then_branch: BasicBlockId,
        else_branch: BasicBlockId,
    },
    Jmp {
        next: BasicBlockId,
    },
    Ret {
        value: Option<VirtualRegister>,
    },
}

#[derive(Debug)]
pub enum IRInstruction {
    Call,
}
