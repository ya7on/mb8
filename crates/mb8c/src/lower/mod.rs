use crate::{
    error::CompileResult, hir::HIRProgram, ir::IRProgram, semantic::context::SemanticContext,
};

pub mod bb;
pub mod context;
pub mod handlers;

/// # Errors
/// Returns a `CompileError` if there was an lowering error
pub fn lower(hir_ctx: &SemanticContext, hir: &HIRProgram) -> CompileResult<IRProgram> {
    todo!()
    // let mut functions = Vec::with_capacity(hir.functions.len());

    // for function in &hir.functions {
    //     functions.push(lower_function(hir_ctx, function)?);
    // }

    // Ok(IRProgram { functions })
}
