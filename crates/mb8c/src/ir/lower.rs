use crate::{
    error::{CompileError, CompileResult},
    parser::ast::{Expr, Stmt},
    tokenizer::token::Operator,
};

use super::{builder::IRBuilder, BinOperation, IROpcode, Reg};

impl IRBuilder {
    /// Lower an expression into an intermediate representation.
    ///
    /// # Errors
    /// Returns a `CompileError` if the expression cannot be lowered.
    #[allow(clippy::too_many_lines)]
    pub fn lower_expr(&mut self, expr: &Expr) -> CompileResult<Reg> {
        match expr {
            Expr::IntLiteral(n) => {
                let reg = self.new_reg();
                self.emit(IROpcode::LoadImm { imm: *n as i64 }, Some(reg), None, None);
                Ok(reg)
            }

            Expr::Var(name) => {
                let local =
                    *self
                        .locals_map
                        .get(name)
                        .ok_or_else(|| CompileError::InternalError {
                            message: "Variable not found".to_string(),
                        })?;
                let ty = self.func.locals[local as usize].ty;
                let reg = self.new_reg();
                self.emit(
                    IROpcode::LoadLocal {
                        local,
                        size: ty.size_in_bytes(),
                    },
                    Some(reg),
                    None,
                    None,
                );
                Ok(reg)
            }

            Expr::Assign { name, value } => {
                let rhs_reg = self.lower_expr(value)?;
                let local =
                    *self
                        .locals_map
                        .get(name)
                        .ok_or_else(|| CompileError::InternalError {
                            message: "Variable not found".to_string(),
                        })?;
                let ty = self.func.locals[local as usize].ty;
                self.emit(
                    IROpcode::StoreLocal {
                        local,
                        size: ty.size_in_bytes(),
                    },
                    None,
                    Some(rhs_reg),
                    None,
                );

                Ok(rhs_reg)
            }

            Expr::Negation(inner) => {
                let zero_reg = {
                    let r = self.new_reg();
                    self.emit(IROpcode::LoadImm { imm: 0 }, Some(r), None, None);
                    r
                };
                let x_reg = self.lower_expr(inner)?;
                let res = self.new_reg();
                self.emit(
                    IROpcode::Bin {
                        op: BinOperation::Sub,
                    },
                    Some(res),
                    Some(zero_reg),
                    Some(x_reg),
                );
                Ok(res)
            }

            Expr::BinaryOp { op, lhs, rhs } => {
                let l_reg = self.lower_expr(lhs)?;
                let r_reg = self.lower_expr(rhs)?;
                let res = self.new_reg();

                let bin = match op {
                    Operator::Plus => BinOperation::Add,
                    Operator::Minus => BinOperation::Sub,
                    Operator::Asterisk => BinOperation::Mul,
                    Operator::Slash => BinOperation::Div,
                    Operator::EqEq => BinOperation::Eq,
                    Operator::Eq => unimplemented!(),
                };

                self.emit(
                    IROpcode::Bin { op: bin },
                    Some(res),
                    Some(l_reg),
                    Some(r_reg),
                );
                Ok(res)
            }

            Expr::Call { name, args } => {
                let mut arg_regs = Vec::new();
                for arg in args {
                    let reg = self.lower_expr(arg)?;
                    arg_regs.push(reg);
                }

                let res = self.new_reg();
                self.emit(
                    IROpcode::Call {
                        name: name.clone(),
                        args: arg_regs.clone(),
                    },
                    Some(res),
                    None,
                    None,
                );

                Ok(res)
            }
        }
    }

    /// Lower a statement into IR instructions.
    ///
    /// # Errors
    /// Returns an error if the statement cannot be lowered.
    #[allow(clippy::too_many_lines)]
    pub fn lower_stmt(&mut self, stmt: &Stmt) -> CompileResult<()> {
        match stmt {
            Stmt::Block(stmts) => {
                for stmt in stmts {
                    self.lower_stmt(stmt)?;
                }
            }

            Stmt::Return(expr_opt) => {
                let reg_opt = expr_opt.as_ref().map(|e| self.lower_expr(e)).transpose()?;
                self.emit(IROpcode::Return, reg_opt, None, None);
            }

            Stmt::Declaration { name, ty, init } => {
                let local = self.add_local(name.clone(), *ty);
                if let Some(expr) = init {
                    let v = self.lower_expr(expr)?;
                    self.emit(
                        IROpcode::StoreLocal {
                            local,
                            size: ty.size_in_bytes(),
                        },
                        None,
                        Some(v),
                        None,
                    );
                }
            }

            Stmt::Expression(expr) => {
                self.lower_expr(expr)?;
            }

            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let result = self.lower_expr(condition)?;
                if !matches!(
                    condition,
                    Expr::BinaryOp {
                        op: Operator::EqEq,
                        lhs: _,
                        rhs: _
                    }
                ) {
                    let dst = self.new_reg();
                    self.emit(IROpcode::LoadImm { imm: 0 }, Some(dst), None, None);
                    self.emit(
                        IROpcode::Bin {
                            op: BinOperation::Eq,
                        },
                        Some(dst),
                        Some(result),
                        Some(dst),
                    );
                    //
                }
                let then_label = self.new_label();
                let else_label = self.new_label();

                self.emit(
                    IROpcode::JumpIfZero { label: then_label },
                    None,
                    Some(result),
                    None,
                );
                self.emit(
                    IROpcode::JumpIfNotZero { label: else_label },
                    None,
                    Some(result),
                    None,
                );

                self.emit(IROpcode::Branch { label: then_label }, None, None, None);
                self.lower_stmt(then_branch)?;
                self.emit(IROpcode::Branch { label: else_label }, None, None, None);
                if let Some(else_branch) = else_branch {
                    self.lower_stmt(else_branch)?;
                }
            }

            Stmt::While { condition, body } => {
                let loop_label = self.new_label();
                let exit_label = self.new_label();

                self.emit(IROpcode::Branch { label: loop_label }, None, None, None);
                self.lower_stmt(body)?;
                let result = self.lower_expr(condition)?;
                if !matches!(
                    condition,
                    Expr::BinaryOp {
                        op: Operator::EqEq,
                        lhs: _,
                        rhs: _
                    }
                ) {
                    let dst = self.new_reg();
                    self.emit(IROpcode::LoadImm { imm: 0 }, Some(dst), None, None);
                    self.emit(
                        IROpcode::Bin {
                            op: BinOperation::Eq,
                        },
                        Some(dst),
                        Some(result),
                        Some(dst),
                    );
                    //
                }
                self.emit(
                    IROpcode::JumpIfZero { label: loop_label },
                    None,
                    Some(result),
                    None,
                );
                self.emit(
                    IROpcode::JumpIfNotZero { label: exit_label },
                    None,
                    Some(result),
                    None,
                );
                self.emit(IROpcode::Branch { label: loop_label }, None, None, None);
                self.emit(IROpcode::Branch { label: exit_label }, None, None, None);
            }
        }
        Ok(())
    }
}
