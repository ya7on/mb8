use crate::semantic::types::TypeKind;

#[derive(Debug)]
pub struct IRProgram {
    pub functions: Vec<IRFunction>,
}

#[derive(Debug)]
pub struct IRFunction {
    pub name: String,
    pub basic_blocks: Vec<BasicBlock>,
    pub size: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
}

#[derive(Debug)]
pub enum IRInstruction {
    LoadlArg {
        ty: TypeKind,
        index: usize,
        mem: Mem,
    },
    StorelArg {
        register: VirtualRegister,
        ty: TypeKind,
        index: usize,
    },
    LoadImm {
        register: VirtualRegister,
        value: u8,
        ty: TypeKind,
    },
    Store {
        src: VirtualRegister,
        mem: Mem,
        ty: TypeKind,
    },
    Load {
        dst: VirtualRegister,
        mem: Mem,
        ty: TypeKind,
    },
    Add {
        dst: VirtualRegister,
        lhs: VirtualRegister,
        rhs: VirtualRegister,
        ty: TypeKind,
    },
    Sub {
        dst: VirtualRegister,
        lhs: VirtualRegister,
        rhs: VirtualRegister,
        ty: TypeKind,
    },
    Mul {
        dst: VirtualRegister,
        lhs: VirtualRegister,
        rhs: VirtualRegister,
        ty: TypeKind,
    },
    Div {
        dst: VirtualRegister,
        lhs: VirtualRegister,
        rhs: VirtualRegister,
        ty: TypeKind,
    },
    Cmp {
        dst: VirtualRegister,
        lhs: VirtualRegister,
        rhs: VirtualRegister,
        ty: TypeKind,
    },
    Neg {
        dst: VirtualRegister,
        src: VirtualRegister,
        ty: TypeKind,
    },
    Call {
        result: VirtualRegister,
        label: String,
        args: Vec<VirtualRegister>,
        ty: TypeKind,
    },
}
