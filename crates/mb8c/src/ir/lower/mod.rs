use crate::{
    context::CompileContext,
    error::{CompileError, CompileResult},
    hir::instructions::HIRProgram,
    pipeline::CompilerPipe,
};

use super::instructions::IRProgram;

pub mod expr;
pub mod function;
pub mod program;
pub mod stmt;

#[derive(Debug, Default)]
pub struct IRLowerer {
    pub ctx: CompileContext,
}

impl CompilerPipe for IRLowerer {
    type Prev = HIRProgram;
    type Next = IRProgram;

    fn execute(prev: &Self::Prev) -> CompileResult<Self::Next, Vec<CompileError>> {
        let mut semantic = IRLowerer::default();
        let ir = semantic.lower_program(prev).map_err(|err| vec![err])?;
        Ok(ir)
    }
}
