use builder::IRBuilder;

use crate::{
    error::CompileResult,
    parser::ast::{Function, Program, Type},
};

pub mod builder;
pub mod lower;

#[derive(Debug, Clone, Copy)]
pub struct Reg(pub u32);

#[derive(Debug)]
pub enum BinOperation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum IROpcode {
    LoadImm { imm: i64 },
    LoadLocal { local: u32, size: u8 },
    StoreLocal { local: u32, size: u8 },
    Bin { op: BinOperation },
    Call { name: String, args: Vec<Reg> },
    Return,
}

#[derive(Debug)]
pub struct IRInstruction {
    pub opcode: IROpcode,
    pub dst: Option<Reg>,
    pub src1: Option<Reg>,
    pub src2: Option<Reg>,
}

#[derive(Debug)]
pub struct LocalInfo {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug)]
pub struct IRFunction {
    pub name: String,
    pub return_type: Type,
    pub locals: Vec<LocalInfo>,
    pub code: Vec<IRInstruction>,
}

/// Lower a function into an `IRFunction`.
///
/// # Errors
/// Returns an error if the function cannot be lowered.
pub fn lower_function(function: &Function) -> CompileResult<IRFunction> {
    let mut builder = IRBuilder::new(function.name.clone(), function.return_type);

    for (name, ty) in &function.params {
        builder.add_local(name.clone(), *ty);
    }

    builder.lower_stmt(&function.body)?;

    Ok(builder.func)
}

/// Lower a function into an `IRFunction`.
///
/// # Errors
/// Returns an error if the function cannot be lowered.
pub fn lower_program(program: &Program) -> CompileResult<Vec<IRFunction>> {
    program.functions.iter().map(lower_function).collect()
}
