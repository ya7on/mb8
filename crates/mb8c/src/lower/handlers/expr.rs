use crate::{
    error::CompileResult,
    hir::{HIRBinaryOp, HIRExpr, HIRUnaryOp, Literal},
    ir::{IRInstruction, VirtualRegister},
};

use super::Lower;

impl Lower {
    /// # Errors
    /// Returns a `CompileError` if there was an lowering error
    pub fn lower_expr(
        &mut self,
        expr: &HIRExpr,
    ) -> CompileResult<(VirtualRegister, Vec<IRInstruction>)> {
        match expr {
            _ => todo!(), // HIRExpr::Var { symbol, ty } => {
                          //     let type_kind = self.ctx.types.lookup(*ty).ok_or_else(|| todo!())?;
                          //     let vreg = self.ctx.vreg(type_kind.size());
                          //     let stored_symbol = self.ctx.lookup_name(symbol).ok_or_else(|| todo!())?;
                          //     let symbol = self.ctx.symbols.lookup(*symbol).ok_or_else(|| todo!())?;
                          //     let instructions = vec![IRInstruction::Load {
                          //         offset: stored_symbol.offset,
                          //         register: vreg,
                          //     }];
                          //     Ok((vreg, instructions))
                          // }
                          // HIRExpr::Literal { literal, ty } => {
                          //     let type_kind = self.ctx.types.lookup(*ty).ok_or_else(|| todo!())?;
                          //     match literal {
                          //         Literal::Int(value) => {
                          //             let vreg = self.ctx.vreg(type_kind.size());
                          //             let instructions = vec![IRInstruction::LoadImm {
                          //                 register: vreg,
                          //                 value: *value as u8,
                          //             }];
                          //             Ok((vreg, instructions))
                          //         }
                          //         // TODO: Add String literal
                          //         #[allow(unreachable_patterns)]
                          //         _ => {
                          //             unreachable!()
                          //         }
                          //     }
                          // }
                          // HIRExpr::Binary { op, lhs, rhs, ty } => {
                          //     let mut result = Vec::new();
                          //     let (lhs_reg, instructions) = self.lower_expr(ctx, lhs)?;
                          //     result.extend(instructions);
                          //     let (rhs_reg, instructions) = self.lower_expr(ctx, rhs)?;
                          //     result.extend(instructions);

                          //     if lhs_reg.size != rhs_reg.size {
                          //         todo!()
                          //     }

                          //     let type_kind = self.ctx.types.lookup(*ty).ok_or_else(|| todo!())?;
                          //     let dst_reg = self.ctx.vreg(type_kind.size());

                          //     let instructions = match op {
                          //         HIRBinaryOp::Add => vec![IRInstruction::Add {
                          //             dst: dst_reg,
                          //             lhs: lhs_reg,
                          //             rhs: rhs_reg,
                          //         }],
                          //         HIRBinaryOp::Sub => vec![IRInstruction::Sub {
                          //             dst: dst_reg,
                          //             lhs: lhs_reg,
                          //             rhs: rhs_reg,
                          //         }],
                          //         HIRBinaryOp::Mul => vec![IRInstruction::Mul {
                          //             dst: dst_reg,
                          //             lhs: lhs_reg,
                          //             rhs: rhs_reg,
                          //         }],
                          //         HIRBinaryOp::Div => vec![IRInstruction::Div {
                          //             dst: dst_reg,
                          //             lhs: lhs_reg,
                          //             rhs: rhs_reg,
                          //         }],
                          //         HIRBinaryOp::Eq => vec![IRInstruction::Cmp {
                          //             dst: dst_reg,
                          //             lhs: lhs_reg,
                          //             rhs: rhs_reg,
                          //         }],
                          //     };
                          //     result.extend(instructions);

                          //     Ok((dst_reg, result))
                          // }
                          // HIRExpr::Unary { op, expr, ty: _ } => {
                          //     let (src, mut instructions) = self.lower_expr(ctx, expr)?;
                          //     let dst = self.ctx.vreg(src.size);
                          //     match op {
                          //         HIRUnaryOp::Neg => {
                          //             instructions.push(IRInstruction::Neg { dst, src });
                          //         }
                          //     }
                          //     Ok((dst, instructions))
                          // }
                          // HIRExpr::Call { func, args, ty } => {
                          //     let type_kind = self.ctx.types.lookup(*ty).ok_or_else(|| todo!())?;
                          //     let dst = self.ctx.vreg(type_kind.size());

                          //     let mut instructions = Vec::new();
                          //     let mut regs = Vec::with_capacity(args.len());
                          //     for arg in args {
                          //         let (reg, instr) = self.lower_expr(ctx, arg)?;
                          //         instructions.extend(instr);
                          //         regs.push(reg);
                          //     }

                          //     let func = self.ctx.symbols.lookup(*func).ok_or_else(|| todo!())?;
                          //     instructions.push(IRInstruction::Call {
                          //         result: dst,
                          //         label: func.name,
                          //         args: regs,
                          //     });

                          //     Ok((dst, instructions))
                          // }
        }
    }
}
