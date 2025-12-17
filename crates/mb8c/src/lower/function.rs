use crate::{
    error::CompileResult,
    hir::HIRFunction,
    ir::{BasicBlockTerminator, IRFunction},
    lower::context::LowerContext,
    semantic::context::SemanticContext,
};

use super::stmt::lower_stmt;

/// # Errors
/// Returns a `CompileError` if there was an lowering error
pub fn lower_function(
    hir_ctx: &SemanticContext,
    function: &HIRFunction,
) -> CompileResult<IRFunction> {
    let mut ctx = LowerContext::new(function.params.as_ref(), hir_ctx);

    let mut basic_blocks = Vec::new();
    let mut current = Some(ctx.bb());
    for stmt in &function.body {
        if let Some(builder) = current {
            let (builder, bbs) = lower_stmt(&mut ctx, builder, stmt)?;
            basic_blocks.extend(bbs);
            current = builder;
        }
        if current.is_none() {
            break;
        }
    }
    if let Some(current) = current {
        basic_blocks.push(current.build(BasicBlockTerminator::Ret { value: None }));
    }

    Ok(IRFunction {
        name: function.name.clone(),
        basic_blocks,
    })
}
