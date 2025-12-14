use crate::{ast::ASTProgram, error::CompileResult, hir::HIRProgram, semantic::context::Context};

use super::function::{analyze_function, collect_function};

pub fn analyze_program(ctx: &mut Context, program: &ASTProgram) -> CompileResult<HIRProgram> {
    let mut hir = HIRProgram {
        functions: Vec::with_capacity(program.functions.len()),
    };

    // 1st iteration. collect function names
    for function in &program.functions {
        collect_function(ctx, function)?;
    }

    // 2nd iteration
    for function in &program.functions {
        let hir_function = analyze_function(ctx, function)?;
        hir.functions.push(hir_function);
    }

    Ok(hir)
}
