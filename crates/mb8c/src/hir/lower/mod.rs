use crate::{hir::instructions::HIRProgram, parser::ast::ASTProgram, pipeline::CompilerPipe};

use super::context::SemanticContext;

pub mod expr;
pub mod function;
pub mod program;
pub mod stmt;

#[derive(Debug, Default)]
pub struct SemanticAnalysis {
    pub ctx: SemanticContext,
}

impl CompilerPipe for SemanticAnalysis {
    type Prev = ASTProgram;
    type Next = HIRProgram;

    fn execute(
        prev: &Self::Prev,
    ) -> crate::error::CompileResult<Self::Next, Vec<crate::error::CompileError>> {
        let mut semantic = SemanticAnalysis::default();
        let hir = semantic.analyze_program(prev).map_err(|err| vec![err])?;
        Ok(hir)
    }
}
