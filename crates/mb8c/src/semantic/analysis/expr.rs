use crate::{
    ast::{ASTBinaryOp, ASTExpr, ASTUnaryOp},
    error::{CompileError, CompileResult},
    hir::{HIRBinaryOp, HIRExpr, HIRUnaryOp, Literal},
    semantic::{context::Context, helpers::fetch_expr_type, symbols::SymbolKind, types::TypeKind},
};

/// Analyze AST Expression and lower it to HIR typed expression
///
/// # Errors
/// Returns error if there are semantic issues
#[allow(clippy::too_many_lines)]
pub fn analyze_expr(ctx: &mut Context, expr: &ASTExpr) -> CompileResult<HIRExpr> {
    match expr {
        ASTExpr::IntLiteral { span: _, value } => Ok(HIRExpr::Literal {
            literal: Literal::Int(*value),
            ty: ctx.types.entry(TypeKind::Unsigned8),
        }),
        ASTExpr::BinaryOp {
            op,
            lhs,
            rhs,
            span: _,
        } => {
            let lhs_expr = analyze_expr(ctx, lhs)?;
            let rhs_expr = analyze_expr(ctx, rhs)?;
            let op = match op {
                ASTBinaryOp::Add => HIRBinaryOp::Add,
                ASTBinaryOp::Sub => HIRBinaryOp::Sub,
                ASTBinaryOp::Mul => HIRBinaryOp::Mul,
                ASTBinaryOp::Div => HIRBinaryOp::Div,
                ASTBinaryOp::Eq => HIRBinaryOp::Eq,
            };

            let lhs_ty = fetch_expr_type(&lhs_expr);
            let rhs_ty = fetch_expr_type(&rhs_expr);

            if lhs_ty != rhs_ty {
                return Err(CompileError::TypeMismatch {
                    expected: ctx.types.lookup(lhs_ty).cloned().unwrap_or_default(),
                    actual: ctx.types.lookup(rhs_ty).cloned().unwrap_or_default(),
                    start: 0,
                    end: 0,
                });
            }

            let return_type = match op {
                HIRBinaryOp::Add | HIRBinaryOp::Sub | HIRBinaryOp::Mul | HIRBinaryOp::Div => lhs_ty,
                HIRBinaryOp::Eq => ctx.types.entry(TypeKind::Bool),
            };

            Ok(HIRExpr::Binary {
                op,
                lhs: Box::new(lhs_expr),
                rhs: Box::new(rhs_expr),
                ty: return_type,
            })
        }
        ASTExpr::UnaryOp {
            op: ASTUnaryOp::Neg,
            expr,
            span: _,
        } => {
            let expr = analyze_expr(ctx, expr)?;
            let ty = fetch_expr_type(&expr);
            Ok(HIRExpr::Unary {
                op: HIRUnaryOp::Neg,
                expr: Box::new(expr),
                ty,
            })
        }
        ASTExpr::Var { name, span } => {
            let symbol_id = ctx.scope.lookup(name).ok_or(CompileError::UnknownSymbol {
                start: span.start,
                end: span.end,
                symbol: name.clone(),
            })?;
            let symbol = ctx
                .symbols
                .lookup(symbol_id)
                .ok_or(CompileError::UnknownSymbol {
                    start: span.start,
                    end: span.end,
                    symbol: name.clone(),
                })?;

            Ok(HIRExpr::Var {
                symbol: symbol_id,
                ty: symbol.ty,
            })
        }
        ASTExpr::Assign { name, value, span } => {
            let symbol_id = ctx.scope.lookup(name).ok_or(CompileError::UnknownSymbol {
                start: span.start,
                end: span.end,
                symbol: name.clone(),
            })?;
            let symbol = ctx
                .symbols
                .lookup(symbol_id)
                .ok_or(CompileError::UnknownSymbol {
                    start: span.start,
                    end: span.end,
                    symbol: name.clone(),
                })?;

            let value = analyze_expr(ctx, value)?;
            let value_ty = fetch_expr_type(&value);

            if symbol.ty != value_ty {
                return Err(CompileError::TypeMismatch {
                    expected: ctx.types.lookup(value_ty).cloned().unwrap_or_default(),
                    actual: ctx.types.lookup(symbol.ty).cloned().unwrap_or_default(),
                    start: span.start,
                    end: span.end,
                });
            }

            Ok(HIRExpr::Assign {
                symbol: symbol_id,
                value: Box::new(value),
                ty: value_ty,
            })
        }
        ASTExpr::Call { name, args, span } => {
            let symbol_id = ctx.scope.lookup(name).ok_or(CompileError::UnknownSymbol {
                start: span.start,
                end: span.end,
                symbol: name.clone(),
            })?;
            let symbol = ctx
                .symbols
                .lookup(symbol_id)
                .ok_or(CompileError::UnknownSymbol {
                    start: span.start,
                    end: span.end,
                    symbol: name.clone(),
                })?;

            if symbol.kind != SymbolKind::Function {
                return Err(CompileError::SymbolIsNotCallable {
                    start: span.start,
                    end: span.end,
                    symbol: name.clone(),
                });
            }

            let ty = ctx
                .types
                .lookup(symbol.ty)
                .ok_or(CompileError::UnknownSymbol {
                    start: span.start,
                    end: span.end,
                    symbol: name.clone(),
                })?
                .clone();
            let TypeKind::Function { params, ret } = ty.clone() else {
                return Err(CompileError::SymbolIsNotCallable {
                    start: span.start,
                    end: span.end,
                    symbol: name.clone(),
                });
            };

            if args.len() != params.len() {
                return Err(CompileError::WrongArgumentsCount {
                    actual: args.len(),
                    expected: params.len(),
                    start: span.start,
                    end: span.end,
                });
            }

            let mut hir_args = Vec::with_capacity(args.len());

            for i in 0..args.len() {
                let arg = args[i].clone();
                let param = params[i];

                let hir_arg = analyze_expr(ctx, &arg)?;
                let arg_ty = fetch_expr_type(&hir_arg);

                if arg_ty != param {
                    return Err(CompileError::TypeMismatch {
                        expected: ctx.types.lookup(param).cloned().unwrap_or_default(),
                        actual: ctx.types.lookup(arg_ty).cloned().unwrap_or_default(),
                        start: span.start,
                        end: span.end,
                    });
                }

                hir_args.push(hir_arg);
            }

            Ok(HIRExpr::Call {
                func: symbol_id,
                args: hir_args,
                ty: ret,
            })
        }
    }
}
