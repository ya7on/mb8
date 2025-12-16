use crate::{error::CompileResult, hir::HIRProgram};

pub mod bb;
pub mod context;
pub mod function;

/// # Errors
/// Returns a `CompileError` if there was an lowering error
pub fn lower(hir: &HIRProgram) -> CompileResult<()> {
    for function in &hir.functions {
        println!("{function:?}");
    }

    Ok(())
}
