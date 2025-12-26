use crate::{
    context::CompileContext, hir::instructions::HIRProgram, parser::ast::ASTProgram,
    pipeline::CompilerPipe,
};

use super::scope::ScopeStack;

pub mod expr;
pub mod function;
pub mod program;
pub mod stmt;

#[derive(Debug, Default)]
pub struct HIRLowerer {
    pub ctx: CompileContext,
    pub scope: ScopeStack,
}

impl CompilerPipe for HIRLowerer {
    type Prev = ASTProgram;
    type Next = (HIRProgram, CompileContext);

    fn execute(
        prev: &Self::Prev,
    ) -> crate::error::CompileResult<Self::Next, Vec<crate::error::CompileError>> {
        let mut semantic = HIRLowerer::default();
        let hir = semantic.analyze_program(prev).map_err(|err| vec![err])?;
        Ok((hir, semantic.ctx))
    }
}
