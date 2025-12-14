use analysis::program::analyze_program;
use context::Context;

use crate::{ast::ASTProgram, error::CompileResult, hir::HIRProgram};

pub mod analysis;
pub mod context;
pub mod helpers;
pub mod scope;
pub mod symbols;
pub mod types;

pub fn analyze(ast: &ASTProgram) -> CompileResult<HIRProgram> {
    let mut context = Context::default();

    // Global scope
    context.scope.enter();

    analyze_program(&mut context, ast)
}
