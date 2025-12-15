use crate::{error::CompileResult, hir::HIRProgram};

pub fn lower(_hir: &HIRProgram) -> CompileResult<()> {
    // for function in &hir.functions {
    //     println!("{function:?}");
    // }

    Ok(())
}
