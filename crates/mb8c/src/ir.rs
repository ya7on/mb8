#[derive(Debug)]
pub struct IRProgram {
    pub functions: Vec<IRFunction>,
}

#[derive(Debug)]
pub struct IRFunction {
    pub basic_blocks: Vec<BasicBlock>,
}

#[derive(Debug, Clone, Copy)]
pub struct VirtualRegister {
    pub id: usize,
    pub size: u8,
}

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
    LoadImm {
        register: VirtualRegister,
        value: u8,
    },
    Store {
        register: VirtualRegister,
        offset: usize,
    },
    Load {
        register: VirtualRegister,
        offset: usize,
    },
    Add {
        dst: VirtualRegister,
        lhs: VirtualRegister,
        rhs: VirtualRegister,
    },
    Sub {
        dst: VirtualRegister,
        lhs: VirtualRegister,
        rhs: VirtualRegister,
    },
    Mul {
        dst: VirtualRegister,
        lhs: VirtualRegister,
        rhs: VirtualRegister,
    },
    Div {
        dst: VirtualRegister,
        lhs: VirtualRegister,
        rhs: VirtualRegister,
    },
    Cmp {
        dst: VirtualRegister,
        lhs: VirtualRegister,
        rhs: VirtualRegister,
    },
    Neg {
        dst: VirtualRegister,
        src: VirtualRegister,
    },
    Call {
        result: VirtualRegister,
        label: String,
        args: Vec<VirtualRegister>,
    },
}
