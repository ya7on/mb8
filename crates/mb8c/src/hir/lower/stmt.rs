use crate::{
    context::{types::TypeKind, TypeId},
    error::{CompileError, CompileResult},
    hir::{helpers::fetch_expr_type, instructions::HIRStmt},
    parser::ast::{ASTExpr, ASTStmt, Span},
};

use super::HIRLowerer;

impl HIRLowerer {
    fn analyze_block_stmt(
        &mut self,
        expected_ty: TypeId,
        stmts: &[ASTStmt],
    ) -> CompileResult<HIRStmt> {
        let mut result = Vec::new();
        self.scope.enter();
        for stmt in stmts {
            result.push(self.analyze_stmt(stmt, expected_ty)?);
        }
        self.scope.exit();
        Ok(HIRStmt::Block(result))
    }

    fn analyze_return_stmt(
        &mut self,
        expected_ty: TypeId,
        expr: Option<&ASTExpr>,
        span: &Span,
    ) -> CompileResult<HIRStmt> {
        let value = if let Some(expr) = expr {
            let value = self.analyze_expr(expr)?;

            let type_id = fetch_expr_type(&value);
            if type_id != expected_ty {
                return Err(CompileError::TypeMismatch {
                    expected: self
                        .ctx
                        .type_table
                        .lookup(expected_ty)
                        .cloned()
                        .unwrap_or_default(),
                    actual: self
                        .ctx
                        .type_table
                        .lookup(type_id)
                        .cloned()
                        .unwrap_or_default(),
                    start: span.start,
                    end: span.end,
                });
            }

            Some(value)
        } else {
            let type_id = self.ctx.type_table.entry(TypeKind::Void);
            if type_id != expected_ty {
                return Err(CompileError::TypeMismatch {
                    expected: self
                        .ctx
                        .type_table
                        .lookup(expected_ty)
                        .cloned()
                        .unwrap_or_default(),
                    actual: self
                        .ctx
                        .type_table
                        .lookup(type_id)
                        .cloned()
                        .unwrap_or_default(),
                    start: span.start,
                    end: span.end,
                });
            }

            None
        };

        Ok(HIRStmt::Return(value))
    }

    fn analyze_expr_stmt(
        &mut self,
        _expected_ty: TypeId,
        expr: &ASTExpr,
        _span: &Span,
    ) -> CompileResult<HIRStmt> {
        let value = self.analyze_expr(expr)?;
        Ok(HIRStmt::Expression(value))
    }

    fn analyze_if_stmt(
        &mut self,
        expected_ty: TypeId,
        condition: &ASTExpr,
        then_branch: &ASTStmt,
        else_branch: Option<&ASTStmt>,
        span: &Span,
    ) -> CompileResult<HIRStmt> {
        let bool = self.ctx.type_table.entry(TypeKind::Bool);
        let condition = self.analyze_expr(condition)?;
        let then_branch = self.analyze_stmt(then_branch, expected_ty)?;
        let else_branch = else_branch
            .map(|expr| self.analyze_stmt(expr, expected_ty))
            .transpose()?;

        let condition_ty = fetch_expr_type(&condition);
        if condition_ty != bool {
            return Err(CompileError::TypeMismatch {
                expected: TypeKind::Bool,
                actual: self
                    .ctx
                    .type_table
                    .lookup(condition_ty)
                    .cloned()
                    .unwrap_or_default(),
                start: span.start,
                end: span.end,
            });
        }

        Ok(HIRStmt::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        })
    }

    fn analyze_while_stmt(
        &mut self,
        expected_ty: TypeId,
        condition: &ASTExpr,
        body: &ASTStmt,
        span: &Span,
    ) -> CompileResult<HIRStmt> {
        let bool = self.ctx.type_table.entry(TypeKind::Bool);
        let condition = self.analyze_expr(condition)?;
        let body = self.analyze_stmt(body, expected_ty)?;

        let condition_ty = fetch_expr_type(&condition);
        if condition_ty != bool {
            return Err(CompileError::TypeMismatch {
                expected: TypeKind::Bool,
                actual: self
                    .ctx
                    .type_table
                    .lookup(condition_ty)
                    .cloned()
                    .unwrap_or_default(),
                start: span.start,
                end: span.end,
            });
        }

        Ok(HIRStmt::While {
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }

    fn analyze_assign_stmt(
        &mut self,
        _expected_ty: TypeId,
        name: &str,
        value: &ASTExpr,
        span: &Span,
    ) -> CompileResult<HIRStmt> {
        let symbol_id = self.scope.lookup(name).ok_or(CompileError::UnknownSymbol {
            start: span.start,
            end: span.end,
            symbol: name.to_owned(),
        })?;
        let symbol =
            self.ctx
                .symbol_table
                .lookup(symbol_id)
                .ok_or(CompileError::UnknownSymbol {
                    start: span.start,
                    end: span.end,
                    symbol: name.to_owned(),
                })?;

        let value = self.analyze_expr(value)?;
        let value_ty = fetch_expr_type(&value);

        if symbol.ty != value_ty {
            return Err(CompileError::TypeMismatch {
                expected: self
                    .ctx
                    .type_table
                    .lookup(value_ty)
                    .cloned()
                    .unwrap_or_default(),
                actual: self
                    .ctx
                    .type_table
                    .lookup(symbol.ty)
                    .cloned()
                    .unwrap_or_default(),
                start: span.start,
                end: span.end,
            });
        }

        Ok(HIRStmt::Assign {
            symbol_id,
            value,
            ty: symbol.ty,
        })
    }

    /// # Errors
    /// Returns error if there are semantic issues
    pub fn analyze_stmt(&mut self, stmt: &ASTStmt, expected_ty: TypeId) -> CompileResult<HIRStmt> {
        match stmt {
            ASTStmt::Block(stmts) => self.analyze_block_stmt(expected_ty, stmts),
            ASTStmt::Return { expr, span } => {
                self.analyze_return_stmt(expected_ty, expr.as_ref(), span)
            }
            ASTStmt::Expression { expr, span } => self.analyze_expr_stmt(expected_ty, expr, span),
            ASTStmt::If {
                condition,
                then_branch,
                else_branch,
                span,
            } => self.analyze_if_stmt(
                expected_ty,
                condition,
                then_branch,
                else_branch.as_deref(),
                span,
            ),
            ASTStmt::While {
                condition,
                body,
                span,
            } => self.analyze_while_stmt(expected_ty, condition, body, span),
            ASTStmt::Assign { name, value, span } => {
                self.analyze_assign_stmt(expected_ty, name, value, span)
            }
        }
    }
}
