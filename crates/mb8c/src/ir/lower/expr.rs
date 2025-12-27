use crate::{
    context::{SymbolId, TypeId},
    error::CompileResult,
    hir::instructions::{HIRBinaryOp, HIRExpr, HIRUnaryOp, Literal},
    ir::instructions::IRInstruction,
};

use super::IRLowerer;

impl IRLowerer {
    /// Lower a variable expression into IR instructions.
    ///
    /// # Errors
    /// Returns an error when the variable type cannot be resolved.
    pub fn lower_var_expr(
        &mut self,
        symbol_id: &SymbolId,
        ty: &TypeId,
    ) -> CompileResult<Vec<IRInstruction>> {
        let type_kind = self.ctx.type_table.lookup(*ty).ok_or_else(|| todo!())?;
        Ok(vec![IRInstruction::PushVar {
            symbol: *symbol_id,
            width: type_kind.width(),
        }])
    }

    /// Lower a literal expression into IR instructions.
    ///
    /// # Errors
    /// Returns an error when the literal type cannot be resolved.
    pub fn lower_literal_expr(
        &mut self,
        literal: &Literal,
        ty: &TypeId,
    ) -> CompileResult<Vec<IRInstruction>> {
        let type_kind = self.ctx.type_table.lookup(*ty).ok_or_else(|| todo!())?;
        let Literal::Int(value) = literal;
        Ok(vec![IRInstruction::LoadImm {
            value: *value,
            width: type_kind.width(),
        }])
    }

    /// Lower a binary expression into IR instructions.
    ///
    /// # Errors
    /// Returns an error when operand lowering or type resolution fails.
    pub fn lower_binary_expr(
        &mut self,
        op: &HIRBinaryOp,
        lhs: &HIRExpr,
        rhs: &HIRExpr,
        ty: &TypeId,
    ) -> CompileResult<Vec<IRInstruction>> {
        let mut instructions = Vec::new();
        instructions.extend(self.lower_expr(lhs)?);
        instructions.extend(self.lower_expr(rhs)?);

        let type_kind = self.ctx.type_table.lookup(*ty).ok_or_else(|| todo!())?;
        match op {
            HIRBinaryOp::Add => instructions.push(IRInstruction::Add {
                width: type_kind.width(),
            }),
            HIRBinaryOp::Sub => instructions.push(IRInstruction::Sub {
                width: type_kind.width(),
            }),
            HIRBinaryOp::Mul => instructions.push(IRInstruction::Mul {
                width: type_kind.width(),
            }),
            HIRBinaryOp::Div => instructions.push(IRInstruction::Div {
                width: type_kind.width(),
            }),
            HIRBinaryOp::Eq => instructions.push(IRInstruction::Eq {
                width: type_kind.width(),
            }),
        }
        Ok(instructions)
    }

    /// Lower a unary expression into IR instructions.
    ///
    /// # Errors
    /// Returns an error when operand lowering or type resolution fails.
    pub fn lower_unary_expr(
        &mut self,
        op: &HIRUnaryOp,
        expr: &HIRExpr,
        ty: &TypeId,
    ) -> CompileResult<Vec<IRInstruction>> {
        let mut instructions = Vec::new();
        instructions.extend(self.lower_expr(expr)?);

        let type_kind = self.ctx.type_table.lookup(*ty).ok_or_else(|| todo!())?;

        match op {
            HIRUnaryOp::Neg => instructions.push(IRInstruction::Neg {
                width: type_kind.width(),
            }),
        }
        Ok(instructions)
    }

    /// Lower a call expression into IR instructions.
    ///
    /// # Errors
    /// Returns an error when argument lowering fails.
    pub fn lower_call_expr(
        &mut self,
        symbol: &SymbolId,
        args: &[HIRExpr],
        _ty: &TypeId,
    ) -> CompileResult<Vec<IRInstruction>> {
        let mut instructions = Vec::new();

        for arg in args {
            instructions.extend(self.lower_expr(arg)?);
        }

        instructions.push(IRInstruction::Call {
            symbol: *symbol,
            argc: args.len(),
        });

        Ok(instructions)
    }

    /// Lower any expression node into IR instructions.
    ///
    /// # Errors
    /// Propagates errors from specific lowering routines.
    pub fn lower_expr(&mut self, expr: &HIRExpr) -> CompileResult<Vec<IRInstruction>> {
        match expr {
            HIRExpr::Var { symbol, ty } => self.lower_var_expr(symbol, ty),
            HIRExpr::Literal { literal, ty } => self.lower_literal_expr(literal, ty),
            HIRExpr::Binary { op, lhs, rhs, ty } => self.lower_binary_expr(op, lhs, rhs, ty),
            HIRExpr::Unary { op, expr, ty } => self.lower_unary_expr(op, expr, ty),
            HIRExpr::Call { symbol, args, ty } => self.lower_call_expr(symbol, args, ty),
        }
    }
}
