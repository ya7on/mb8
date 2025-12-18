use crate::{
    error::{CompileError, CompileResult},
    hir::HIRProgram,
    ir::IRProgram,
    pipe::CompilerPipe,
};

use super::context::LowerContext;

pub mod expr;
pub mod function;
pub mod stmt;

#[derive(Debug)]
pub struct Lower {
    ctx: LowerContext,
}

impl CompilerPipe for Lower {
    type Prev = HIRProgram;
    type Next = IRProgram;

    fn execute(prev: &Self::Prev) -> CompileResult<Self::Next, Vec<CompileError>> {
        // let lower = Self {
        //     ctx: LowerContext::new(, hir_ctx)
        // };

        todo!()
    }
}
