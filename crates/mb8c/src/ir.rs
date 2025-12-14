#[derive(Debug)]
pub struct VirtualRegister(pub usize);

#[derive(Debug)]
pub struct BasicBlockId(pub usize);

#[derive(Debug)]
pub struct BasicBlock {
    pub id: BasicBlockId,
    pub next_id: BasicBlockId,
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
