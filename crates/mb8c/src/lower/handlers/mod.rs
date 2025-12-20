use crate::{
    error::{CompileError, CompileResult},
    hir::HIRProgram,
    ir::IRProgram,
    pipe::CompilerPipe,
};

pub mod expr;
pub mod function;
pub mod stmt;

#[derive(Debug)]
pub struct Lower {
    hir: HIRProgram,
}

impl CompilerPipe for Lower {
    type Prev = HIRProgram;
    type Next = IRProgram;

    fn execute(prev: &Self::Prev) -> CompileResult<Self::Next, Vec<CompileError>> {
        let mut lower = Self {
            hir: prev.to_owned(),
        };

        let mut functions = Vec::with_capacity(lower.hir.functions.len());

        for function in &prev.functions {
            functions.push(lower.lower_function(function).map_err(|err| vec![err])?);
        }

        Ok(IRProgram { functions })
    }
}
