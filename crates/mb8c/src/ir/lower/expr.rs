use crate::{
    error::CompileResult,
    hir::helpers::fetch_expr_type,
    hir::instructions::{HIRBinaryOp, HIRExpr, HIRUnaryOp, Literal},
    ir::context::LowerContext,
    ir::instructions::{IRInstruction, VirtualRegister},
};

use super::{helpers::get_memory_from_stored_symbol, Lower};

impl Lower {
    /// # Errors
    /// Returns a `CompileError` if there was an lowering error
    #[allow(clippy::too_many_lines)]
    pub fn lower_expr(
        &mut self,
        ctx: &mut LowerContext,
        expr: &HIRExpr,
    ) -> CompileResult<(VirtualRegister, Vec<IRInstruction>)> {
        match expr {
            HIRExpr::Var { symbol, ty } => {
                let vreg = ctx.vreg();

                let stored = ctx.lookup_name(symbol).ok_or_else(|| todo!())?;
                let type_kind = self.hir.types.lookup(*ty).ok_or_else(|| todo!())?;

                let instructions = vec![IRInstruction::Load {
                    dst: vreg,
                    mem: get_memory_from_stored_symbol(stored),
                    ty: type_kind.clone(),
                }];
                Ok((vreg, instructions))
            }
            HIRExpr::Literal { literal, ty } => {
                let type_kind = self.hir.types.lookup(*ty).ok_or_else(|| todo!())?;
                match literal {
                    Literal::Int(value) => {
                        let vreg = ctx.vreg();
                        let instructions = vec![IRInstruction::LoadImm {
                            register: vreg,
                            value: *value as u8,
                            ty: type_kind.clone(),
                        }];
                        Ok((vreg, instructions))
                    }
                    // TODO: Add String literal
                    #[allow(unreachable_patterns)]
                    _ => {
                        unreachable!()
                    }
                }
            }
            HIRExpr::Binary { op, lhs, rhs, ty } => {
                let mut result = Vec::new();
                let (lhs_reg, instructions) = self.lower_expr(ctx, lhs)?;
                result.extend(instructions);
                let (rhs_reg, instructions) = self.lower_expr(ctx, rhs)?;
                result.extend(instructions);

                let type_kind = self.hir.types.lookup(*ty).ok_or_else(|| todo!())?;
                let dst_reg = ctx.vreg();

                let instructions = match op {
                    HIRBinaryOp::Add => vec![IRInstruction::Add {
                        dst: dst_reg,
                        lhs: lhs_reg,
                        rhs: rhs_reg,
                        ty: type_kind.to_owned(),
                    }],
                    HIRBinaryOp::Sub => vec![IRInstruction::Sub {
                        dst: dst_reg,
                        lhs: lhs_reg,
                        rhs: rhs_reg,
                        ty: type_kind.to_owned(),
                    }],
                    HIRBinaryOp::Mul => vec![IRInstruction::Mul {
                        dst: dst_reg,
                        lhs: lhs_reg,
                        rhs: rhs_reg,
                        ty: type_kind.to_owned(),
                    }],
                    HIRBinaryOp::Div => vec![IRInstruction::Div {
                        dst: dst_reg,
                        lhs: lhs_reg,
                        rhs: rhs_reg,
                        ty: type_kind.to_owned(),
                    }],
                    HIRBinaryOp::Eq => vec![IRInstruction::Cmp {
                        dst: dst_reg,
                        lhs: lhs_reg,
                        rhs: rhs_reg,
                        ty: type_kind.to_owned(),
                    }],
                };
                result.extend(instructions);

                Ok((dst_reg, result))
            }
            HIRExpr::Unary { op, expr, ty } => {
                let (src, mut instructions) = self.lower_expr(ctx, expr)?;
                let type_kind = self.hir.types.lookup(*ty).ok_or_else(|| todo!())?;
                let dst = ctx.vreg();
                match op {
                    HIRUnaryOp::Neg => {
                        instructions.push(IRInstruction::Neg {
                            dst,
                            src,
                            ty: type_kind.to_owned(),
                        });
                    }
                }
                Ok((dst, instructions))
            }
            HIRExpr::Call {
                symbol: _,
                label,
                args,
                ty,
            } => {
                let dst = ctx.vreg();

                let mut instructions = Vec::new();
                let mut regs = Vec::with_capacity(args.len());
                for (index, arg) in args.iter().enumerate() {
                    let (reg, instr) = self.lower_expr(ctx, arg)?;
                    instructions.extend(instr);
                    let ty = fetch_expr_type(arg);
                    let type_kind = self.hir.types.lookup(ty).ok_or_else(|| todo!())?;
                    instructions.push(IRInstruction::StorelArg {
                        register: reg,
                        ty: type_kind.to_owned(),
                        index,
                    });
                    regs.push(reg);
                }

                let type_kind = self.hir.types.lookup(*ty).ok_or_else(|| todo!())?;
                instructions.push(IRInstruction::Call {
                    result: dst,
                    label: label.to_owned(),
                    args: regs,
                    ty: type_kind.to_owned(),
                });

                Ok((dst, instructions))
            }
        }
    }
}
