use crate::{
    error::{CompileError, CompileResult},
    hir::instructions::{HIRBinaryOp, HIRExpr, HIRUnaryOp, Literal},
    hir::{helpers::fetch_expr_type, symbols::SymbolKind, types::TypeKind},
    parser::ast::{ASTBinaryOp, ASTExpr, ASTUnaryOp, Span},
};

use super::SemanticAnalysis;

impl SemanticAnalysis {
    #[allow(clippy::unnecessary_wraps)]
    fn analyze_int_literal_expr(&mut self, _span: &Span, value: u16) -> CompileResult<HIRExpr> {
        Ok(HIRExpr::Literal {
            literal: Literal::Int(value),
            ty: self.ctx.types.entry(TypeKind::Unsigned8),
        })
    }

    fn analyze_binary_op_expr(
        &mut self,
        op: &ASTBinaryOp,
        lhs: &ASTExpr,
        rhs: &ASTExpr,
        _span: &Span,
    ) -> CompileResult<HIRExpr> {
        let lhs_expr = self.analyze_expr(lhs)?;
        let rhs_expr = self.analyze_expr(rhs)?;
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
                expected: self.ctx.types.lookup(lhs_ty).cloned().unwrap_or_default(),
                actual: self.ctx.types.lookup(rhs_ty).cloned().unwrap_or_default(),
                start: 0,
                end: 0,
            });
        }

        let return_type = match op {
            HIRBinaryOp::Add | HIRBinaryOp::Sub | HIRBinaryOp::Mul | HIRBinaryOp::Div => lhs_ty,
            HIRBinaryOp::Eq => self.ctx.types.entry(TypeKind::Bool),
        };

        Ok(HIRExpr::Binary {
            op,
            lhs: Box::new(lhs_expr),
            rhs: Box::new(rhs_expr),
            ty: return_type,
        })
    }

    fn analyze_unary_op_expr(
        &mut self,
        _op: &ASTUnaryOp,
        expr: &ASTExpr,
        _span: &Span,
    ) -> CompileResult<HIRExpr> {
        let expr = self.analyze_expr(expr)?;
        let ty = fetch_expr_type(&expr);
        Ok(HIRExpr::Unary {
            op: HIRUnaryOp::Neg,
            expr: Box::new(expr),
            ty,
        })
    }

    fn analyze_var_expr(&mut self, name: &str, span: &Span) -> CompileResult<HIRExpr> {
        let symbol_id = self
            .ctx
            .scope
            .lookup(name)
            .ok_or(CompileError::UnknownSymbol {
                start: span.start,
                end: span.end,
                symbol: name.to_owned(),
            })?;
        let symbol = self
            .ctx
            .symbols
            .lookup(symbol_id)
            .ok_or(CompileError::UnknownSymbol {
                start: span.start,
                end: span.end,
                symbol: name.to_owned(),
            })?;

        Ok(HIRExpr::Var {
            symbol: symbol_id,
            ty: symbol.ty,
        })
    }

    fn analyze_call_expr(
        &mut self,
        name: &str,
        args: &[ASTExpr],
        span: &Span,
    ) -> CompileResult<HIRExpr> {
        let symbol_id = self
            .ctx
            .scope
            .lookup(name)
            .ok_or(CompileError::UnknownSymbol {
                start: span.start,
                end: span.end,
                symbol: name.to_owned(),
            })?;
        let symbol = self
            .ctx
            .symbols
            .lookup(symbol_id)
            .ok_or(CompileError::UnknownSymbol {
                start: span.start,
                end: span.end,
                symbol: name.to_owned(),
            })?;

        if symbol.kind != SymbolKind::Function {
            return Err(CompileError::SymbolIsNotCallable {
                start: span.start,
                end: span.end,
                symbol: name.to_owned(),
            });
        }

        let ty = self
            .ctx
            .types
            .lookup(symbol.ty)
            .ok_or(CompileError::UnknownSymbol {
                start: span.start,
                end: span.end,
                symbol: name.to_owned(),
            })?
            .clone();
        let TypeKind::Function { params, ret } = ty.clone() else {
            return Err(CompileError::SymbolIsNotCallable {
                start: span.start,
                end: span.end,
                symbol: name.to_owned(),
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

            let hir_arg = self.analyze_expr(&arg)?;
            let arg_ty = fetch_expr_type(&hir_arg);

            if arg_ty != param {
                return Err(CompileError::TypeMismatch {
                    expected: self.ctx.types.lookup(param).cloned().unwrap_or_default(),
                    actual: self.ctx.types.lookup(arg_ty).cloned().unwrap_or_default(),
                    start: span.start,
                    end: span.end,
                });
            }

            hir_args.push(hir_arg);
        }

        Ok(HIRExpr::Call {
            symbol: symbol_id,
            label: name.to_owned(),
            args: hir_args,
            ty: ret,
        })
    }

    /// Analyze AST Expression and lower it to HIR typed expression
    ///
    /// # Errors
    /// Returns error if there are semantic issues
    pub fn analyze_expr(&mut self, expr: &ASTExpr) -> CompileResult<HIRExpr> {
        match expr {
            ASTExpr::IntLiteral { span, value } => self.analyze_int_literal_expr(span, *value),
            ASTExpr::BinaryOp { op, lhs, rhs, span } => {
                self.analyze_binary_op_expr(op, lhs, rhs, span)
            }
            ASTExpr::UnaryOp { op, expr, span } => self.analyze_unary_op_expr(op, expr, span),
            ASTExpr::Var { name, span } => self.analyze_var_expr(name, span),
            ASTExpr::Call { name, args, span } => self.analyze_call_expr(name, args, span),
        }
    }
}
