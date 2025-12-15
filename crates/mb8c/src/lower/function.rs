use crate::{
    error::CompileResult,
    hir::{HIRExpr, HIRFunction, HIRStmt},
    ir::{BasicBlock, BasicBlockTerminator, IRFunction, IRInstruction, VirtualRegister},
    lower::context::LowerContext,
};

use super::bb::BasicBlockBuilder;

pub fn lower_function(function: &HIRFunction) -> CompileResult<IRFunction> {
    let mut ctx = LowerContext::default();

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

    Ok(IRFunction { basic_blocks })
}

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
                let (vreg, instructions) = lower_expr(expr)?;
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
            let (_vreg, instructions) = lower_expr(expr)?;
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

            let (condition_vreg, instructions) = lower_expr(condition)?;
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
        HIRStmt::While {
            condition: _,
            body: _,
        } => {
            todo!()
        }
    }
}

pub fn lower_expr(_expr: &HIRExpr) -> CompileResult<(VirtualRegister, Vec<IRInstruction>)> {
    todo!()
}
