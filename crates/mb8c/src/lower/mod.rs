use context::LowerContext;
use function::lower_function;

use crate::{error::CompileResult, hir::HIRProgram, ir::IRProgram};

pub mod bb;
pub mod context;
pub mod expr;
pub mod function;
pub mod stmt;

/// # Errors
/// Returns a `CompileError` if there was an lowering error
pub fn lower(hir: &HIRProgram) -> CompileResult<IRProgram> {
    let mut functions = Vec::with_capacity(hir.functions.len());

    for function in &hir.functions {
        functions.push(lower_function(function)?);
    }

    Ok(IRProgram { functions })
}
