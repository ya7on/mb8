use crate::{error::CompileResult, hir::HIRProgram};

pub fn lower(_hir: &HIRProgram) -> CompileResult<()> {
    Ok(())
}
