use crate::{
    context::{SymbolId, TypeId},
    error::CompileResult,
    hir::{
        helpers::fetch_expr_type,
        instructions::{HIRExpr, HIRStmt},
    },
    ir::{
        bb::{BasicBlockBuilder, BasicBlockTable},
        instructions::{BasicBlock, BasicBlockTerminator, IRInstruction},
    },
};

use super::IRLowerer;

impl IRLowerer {
    /// Lower a block of statements, returning the resulting basic blocks.
    ///
    /// # Errors
    /// Returns an error when lowering any contained statement fails.
    pub fn lower_block_stmt(
        &mut self,
        stmts: &[HIRStmt],
        mut builder: BasicBlockBuilder,
        bbs: &mut BasicBlockTable,
    ) -> CompileResult<(Option<BasicBlockBuilder>, Vec<BasicBlock>)> {
        let mut result = Vec::new();
        for stmt in stmts {
            let (maybe_builder, bbs) = self.lower_stmt(stmt, builder, bbs)?;
            result.extend(bbs);
            let Some(new_builder) = maybe_builder else {
                return Ok((None, result));
            };
            builder = new_builder;
        }
        Ok((Some(builder), result))
    }

    /// Lower a return statement.
    ///
    /// # Errors
    /// Returns an error when the returned expression fails to lower.
    pub fn lower_return_stmt(
        &mut self,
        value: &Option<HIRExpr>,
        mut builder: BasicBlockBuilder,
    ) -> CompileResult<(Option<BasicBlockBuilder>, Vec<BasicBlock>)> {
        let mut result = Vec::new();
        let mut width = 0;
        if let Some(expr) = value {
            let instructions = self.lower_expr(expr)?;
            let type_id = fetch_expr_type(expr);
            let type_kind = self.ctx.type_table.lookup(type_id).ok_or_else(|| todo!())?;
            width = type_kind.width() as usize;
            for instruction in instructions {
                builder.emit(instruction);
            }
        }
        result.push(builder.build(BasicBlockTerminator::Ret { width }));
        Ok((None, result))
    }

    /// Lower a standalone expression statement.
    ///
    /// # Errors
    /// Returns an error when the expression fails to lower.
    pub fn lower_expression_stmt(
        &mut self,
        expr: &HIRExpr,
        mut builder: BasicBlockBuilder,
    ) -> CompileResult<(Option<BasicBlockBuilder>, Vec<BasicBlock>)> {
        let instructions = self.lower_expr(expr)?;
        for instruction in instructions {
            builder.emit(instruction);
        }
        Ok((Some(builder), vec![]))
    }

    /// Lower an if statement into branching basic blocks.
    ///
    /// # Errors
    /// Returns an error when condition or branch lowering fails.
    pub fn lower_if_stmt(
        &mut self,
        condition: &HIRExpr,
        then_branch: &HIRStmt,
        else_branch: &Option<Box<HIRStmt>>,
        mut builder: BasicBlockBuilder,
        bbs: &mut BasicBlockTable,
    ) -> CompileResult<(Option<BasicBlockBuilder>, Vec<BasicBlock>)> {
        let mut result = Vec::new();

        let then_block = bbs.bb();
        let else_block = bbs.bb();
        let merge_block = bbs.bb();

        let instructions = self.lower_expr(condition)?;
        for instruction in instructions {
            builder.emit(instruction);
        }

        result.push(builder.build(BasicBlockTerminator::Branch {
            then_branch: then_block.id(),
            else_branch: else_block.id(),
        }));

        let (then_block, then_blocks) = self.lower_stmt(then_branch, then_block, bbs)?;
        result.extend(then_blocks);
        if let Some(then_block) = then_block {
            result.push(then_block.build(BasicBlockTerminator::Jmp {
                next: merge_block.id(),
            }));
        }

        if let Some(else_branch) = else_branch {
            let (else_block, else_blocks) = self.lower_stmt(else_branch, else_block, bbs)?;
            result.extend(else_blocks);
            if let Some(else_block) = else_block {
                result.push(else_block.build(BasicBlockTerminator::Jmp {
                    next: merge_block.id(),
                }));
            }
        } else {
            result.push(else_block.build(BasicBlockTerminator::Jmp {
                next: merge_block.id(),
            }));
        }

        Ok((Some(merge_block), result))
    }

    /// Lower a while loop into IR basic blocks.
    ///
    /// # Errors
    /// Returns an error when lowering the condition or body fails.
    pub fn lower_while_stmt(
        &mut self,
        condition: &HIRExpr,
        body: &HIRStmt,
        builder: BasicBlockBuilder,
        bbs: &mut BasicBlockTable,
    ) -> CompileResult<(Option<BasicBlockBuilder>, Vec<BasicBlock>)> {
        let mut result = Vec::new();

        let mut cond_block = bbs.bb();
        let body_block = bbs.bb();
        let exit_block = bbs.bb();

        result.push(builder.build(BasicBlockTerminator::Jmp {
            next: cond_block.id,
        }));

        let instructions = self.lower_expr(condition)?;
        for instruction in instructions {
            cond_block.emit(instruction);
        }

        let (body_block, body_blocks) = self.lower_stmt(body, body_block, bbs)?;
        result.extend(body_blocks);
        let Some(body_block) = body_block else {
            return Ok((None, result));
        };
        let body_block_id = body_block.id();
        result.push(body_block.build(BasicBlockTerminator::Jmp {
            next: cond_block.id(),
        }));
        result.push(cond_block.build(BasicBlockTerminator::Branch {
            then_branch: body_block_id,
            else_branch: exit_block.id(),
        }));

        Ok((Some(exit_block), result))
    }

    /// Lower an assignment statement.
    ///
    /// # Errors
    /// Returns an error when the value expression cannot be lowered or the type lookup fails.
    pub fn lower_assign_stmt(
        &mut self,
        symbol_id: &SymbolId,
        ty: &TypeId,
        value: &HIRExpr,
        mut builder: BasicBlockBuilder,
    ) -> CompileResult<(Option<BasicBlockBuilder>, Vec<BasicBlock>)> {
        let instructions = self.lower_expr(value)?;
        for instruction in instructions {
            builder.emit(instruction);
        }

        let type_kind = self.ctx.type_table.lookup(*ty).ok_or_else(|| todo!())?;

        builder.emit(IRInstruction::StoreVar {
            symbol: *symbol_id,
            width: type_kind.width(),
        });

        Ok((Some(builder), vec![]))
    }

    /// Dispatch lowering for any statement node.
    ///
    /// # Errors
    /// Propagates errors from specific lowering routines.
    pub fn lower_stmt(
        &mut self,
        stmt: &HIRStmt,
        builder: BasicBlockBuilder,
        bbs: &mut BasicBlockTable,
    ) -> CompileResult<(Option<BasicBlockBuilder>, Vec<BasicBlock>)> {
        match stmt {
            HIRStmt::Block(stmts) => self.lower_block_stmt(stmts, builder, bbs),
            HIRStmt::Return(value) => self.lower_return_stmt(value, builder),
            HIRStmt::Expression(expr) => self.lower_expression_stmt(expr, builder),
            HIRStmt::If {
                condition,
                then_branch,
                else_branch,
            } => self.lower_if_stmt(condition, then_branch, else_branch, builder, bbs),
            HIRStmt::While { condition, body } => {
                self.lower_while_stmt(condition, body, builder, bbs)
            }
            HIRStmt::Assign {
                symbol_id: symbol,
                ty,
                value,
            } => self.lower_assign_stmt(symbol, ty, value, builder),
        }
    }
}
