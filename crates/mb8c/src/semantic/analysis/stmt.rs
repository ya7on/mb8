use crate::{
    ast::ASTStmt,
    error::{CompileError, CompileResult},
    hir::{HIRStmt, TypeId},
    semantic::{
        context::Context,
        helpers::{fetch_expr_type, lower_type},
        symbols::{Symbol, SymbolKind},
        types::TypeKind,
    },
};

use super::expr::analyze_expr;

/// # Errors
/// Returns error if there are semantic issues
#[allow(clippy::too_many_lines)]
pub fn analyze_stmt(
    ctx: &mut Context,
    stmt: &ASTStmt,
    expected_ty: TypeId,
) -> CompileResult<HIRStmt> {
    match stmt {
        ASTStmt::Block(stmts) => {
            let mut result = Vec::new();
            ctx.scope.enter();
            for stmt in stmts {
                result.push(analyze_stmt(ctx, stmt, expected_ty)?);
            }
            ctx.scope.exit();
            Ok(HIRStmt::Block(result))
        }
        ASTStmt::Declaration {
            name,
            ty,
            init,
            span,
        } => {
            let scope = ctx.scope.current();

            let type_id = ctx.types.entry(lower_type(*ty));
            let symbol_id = ctx.symbols.allocate(Symbol {
                name: name.to_owned(),
                ty: type_id,
                kind: SymbolKind::Variable,
            });
            scope.allocate(name.to_owned(), symbol_id, span)?;

            let init = if let Some(expr) = init {
                let value = analyze_expr(ctx, expr, type_id)?;
                let expr_type_id = fetch_expr_type(&value);
                if expr_type_id != type_id {
                    return Err(CompileError::TypeMismatch {
                        expected: ctx.types.lookup(type_id).cloned().unwrap_or_default(),
                        actual: ctx.types.lookup(expr_type_id).cloned().unwrap_or_default(),
                        start: span.start,
                        end: span.end,
                    });
                }
                Some(value)
            } else {
                None
            };

            Ok(HIRStmt::Declaration {
                symbol: symbol_id,
                ty: type_id,
                init,
            })
        }
        ASTStmt::Return { expr, span } => {
            let value = if let Some(expr) = expr {
                let value = analyze_expr(ctx, expr, expected_ty)?;

                let type_id = fetch_expr_type(&value);
                if type_id != expected_ty {
                    return Err(CompileError::TypeMismatch {
                        expected: ctx.types.lookup(expected_ty).cloned().unwrap_or_default(),
                        actual: ctx.types.lookup(type_id).cloned().unwrap_or_default(),
                        start: span.start,
                        end: span.end,
                    });
                }

                Some(value)
            } else {
                let type_id = ctx.types.entry(TypeKind::Void);
                if type_id != expected_ty {
                    return Err(CompileError::TypeMismatch {
                        expected: ctx.types.lookup(expected_ty).cloned().unwrap_or_default(),
                        actual: ctx.types.lookup(type_id).cloned().unwrap_or_default(),
                        start: span.start,
                        end: span.end,
                    });
                }

                None
            };

            Ok(HIRStmt::Return(value))
        }
        ASTStmt::Expression { expr, span: _ } => {
            let value = analyze_expr(ctx, expr, expected_ty)?;
            Ok(HIRStmt::Expression(value))
        }
        ASTStmt::If {
            condition,
            then_branch,
            else_branch,
            span: _,
        } => {
            let condition = analyze_expr(ctx, condition, expected_ty)?;
            let then_branch = analyze_stmt(ctx, then_branch, expected_ty)?;
            let else_branch = else_branch
                .clone()
                .map(|expr| analyze_stmt(ctx, &expr, expected_ty))
                .transpose()?;

            Ok(HIRStmt::If {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch: else_branch.map(Box::new),
            })
        }
        ASTStmt::While {
            condition,
            body,
            span: _,
        } => {
            let condition = analyze_expr(ctx, condition, expected_ty)?;
            let body = analyze_stmt(ctx, body, expected_ty)?;

            Ok(HIRStmt::While {
                condition: Box::new(condition),
                body: Box::new(body),
            })
        }
    }
}
