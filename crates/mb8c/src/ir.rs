#[derive(Debug)]
pub struct IRProgram {
    pub functions: Vec<IRFunction>,
}

#[derive(Debug)]
pub struct IRFunction {
    pub name: String,
    pub basic_blocks: Vec<BasicBlock>,
}

#[derive(Debug)]
pub enum IRType {
    Unsigned8,
    Bool,
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
pub enum Mem {
    Local { offset: usize },
    Param { number: usize },
}

#[derive(Debug)]
pub enum IRInstruction {
    LoadImm {
        register: VirtualRegister,
        value: u8,
        ty: IRType,
    },
    Store {
        src: VirtualRegister,
        mem: Mem,
        ty: IRType,
    },
    Load {
        dst: VirtualRegister,
        mem: Mem,
        ty: IRType,
    },
    Add {
        dst: VirtualRegister,
        lhs: VirtualRegister,
        rhs: VirtualRegister,
        ty: IRType,
    },
    Sub {
        dst: VirtualRegister,
        lhs: VirtualRegister,
        rhs: VirtualRegister,
        ty: IRType,
    },
    Mul {
        dst: VirtualRegister,
        lhs: VirtualRegister,
        rhs: VirtualRegister,
        ty: IRType,
    },
    Div {
        dst: VirtualRegister,
        lhs: VirtualRegister,
        rhs: VirtualRegister,
        ty: IRType,
    },
    Cmp {
        dst: VirtualRegister,
        lhs: VirtualRegister,
        rhs: VirtualRegister,
        ty: IRType,
    },
    Neg {
        dst: VirtualRegister,
        src: VirtualRegister,
        ty: IRType,
    },
    Call {
        result: VirtualRegister,
        label: String,
        args: Vec<VirtualRegister>,
    },
}
