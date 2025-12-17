use crate::{
    error::CompileResult,
    hir::HIRStmt,
    ir::{BasicBlock, BasicBlockTerminator, IRInstruction},
};

use super::{bb::BasicBlockBuilder, context::LowerContext, expr::lower_expr};

/// # Errors
/// Returns a `CompileError` if there was an lowering error
#[allow(clippy::too_many_lines)]
pub fn lower_stmt(
    ctx: &mut LowerContext,
    mut builder: BasicBlockBuilder,
    stmt: &HIRStmt,
) -> CompileResult<(Option<BasicBlockBuilder>, Vec<BasicBlock>)> {
    let mut result = Vec::new();

    match stmt {
        HIRStmt::Block(stmts) => {
            for stmt in stmts {
                let (current, bbs) = lower_stmt(ctx, builder, stmt)?;
                result.extend(bbs);
                let Some(current) = current else {
                    return Ok((None, result));
                };
                builder = current;
            }
            Ok((Some(builder), result))
        }
        HIRStmt::Return(expr) => {
            let value = if let Some(expr) = expr {
                let (vreg, instructions) = lower_expr(ctx, expr)?;
                for instruction in instructions {
                    builder.emit(instruction);
                }
                Some(vreg)
            } else {
                None
            };
            result.push(builder.build(BasicBlockTerminator::Ret { value }));
            Ok((None, result))
        }
        HIRStmt::Expression(expr) => {
            let (_vreg, instructions) = lower_expr(ctx, expr)?;
            for instruction in instructions {
                builder.emit(instruction);
            }
            Ok((Some(builder), result))
        }
        HIRStmt::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let mut any_branch = false;
            let then_block = ctx.bb();
            let else_block = ctx.bb();
            let merge_block = ctx.bb();

            let (condition_vreg, instructions) = lower_expr(ctx, condition)?;
            for instruction in instructions {
                builder.emit(instruction);
            }

            result.push(builder.build(BasicBlockTerminator::Branch {
                condition: condition_vreg,
                then_branch: then_block.id(),
                else_branch: else_block.id(),
            }));

            let (then_block, bbs) = lower_stmt(ctx, then_block, then_branch)?;
            result.extend(bbs);
            if let Some(then_block) = then_block {
                any_branch = true;
                result.push(then_block.build(BasicBlockTerminator::Jmp {
                    next: merge_block.id(),
                }));
            }

            if let Some(else_branch) = else_branch {
                let (else_block, bbs) = lower_stmt(ctx, else_block, else_branch)?;
                result.extend(bbs);
                if let Some(else_block) = else_block {
                    any_branch = true;
                    result.push(else_block.build(BasicBlockTerminator::Jmp {
                        next: merge_block.id(),
                    }));
                }
            } else {
                any_branch = true;
                result.push(else_block.build(BasicBlockTerminator::Jmp {
                    next: merge_block.id(),
                }));
            }

            Ok((any_branch.then_some(merge_block), result))
        }
        HIRStmt::While { condition, body } => {
            let body_block = ctx.bb();
            let exit_block = ctx.bb();

            let (vreg, instructions) = lower_expr(ctx, condition)?;
            for instruction in instructions {
                builder.emit(instruction);
            }

            let (body_block, blocks) = lower_stmt(ctx, body_block, body)?;
            result.extend(blocks);
            let Some(body_block) = body_block else {
                return Ok((None, result));
            };
            let body_block_id = body_block.id();
            result.push(body_block.build(BasicBlockTerminator::Branch {
                condition: vreg,
                then_branch: body_block_id,
                else_branch: exit_block.id(),
            }));
            result.push(builder.build(BasicBlockTerminator::Branch {
                condition: vreg,
                then_branch: body_block_id,
                else_branch: exit_block.id(),
            }));

            Ok((Some(exit_block), result))
        }
        HIRStmt::Assign { symbol, ty, value } => {
            let (vreg, instructions) = lower_expr(ctx, value)?;
            for instruction in instructions {
                builder.emit(instruction);
            }

            let stored_symbol = ctx.lookup_name(symbol).ok_or_else(|| todo!())?;
            let type_kind = ctx.types.lookup(*ty).ok_or_else(|| todo!())?;
            if stored_symbol.size != type_kind.size() as usize {
                todo!()
            }

            builder.emit(IRInstruction::Store {
                offset: stored_symbol.offset,
                register: vreg,
            });

            Ok((Some(builder), result))
        }
    }
}
