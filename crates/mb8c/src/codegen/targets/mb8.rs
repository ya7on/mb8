use crate::{
    context::{types::TypeKind, CompileContext},
    error::{CompileError, CompileResult},
    ir::instructions::{BasicBlock, BasicBlockTerminator, IRFunction, IRInstruction, IRProgram},
    layout::{Layout, Place},
    pipeline::CompilerPipe,
};

use super::asm::Mb8Asm;

#[derive(Debug)]
pub struct Mb8Codegen {
    ctx: CompileContext,
    layout: Layout,
    result: Vec<Mb8Asm>,
}

impl CompilerPipe for Mb8Codegen {
    type Prev = (IRProgram, CompileContext, Layout);
    type Next = Vec<Mb8Asm>;

    fn execute(prev: &Self::Prev) -> CompileResult<Self::Next, Vec<CompileError>> {
        let (ir, ctx, layout) = prev;
        let mut codegen = Mb8Codegen {
            ctx: ctx.clone(),
            layout: layout.clone(),
            result: vec![
                Mb8Asm::Import("../asm/cpu.asm".to_string()),
                Mb8Asm::Import("../asm/ext.asm".to_string()),
            ],
        };
        codegen.codegen(ir).map_err(|err| vec![err])
    }
}

impl Mb8Codegen {
    /// Generate code for a basic block.
    ///
    /// # Errors
    /// This function will return an error if the basic block cannot be generated.
    #[allow(clippy::too_many_lines)]
    pub fn codegen_basic_block(&mut self, bb: &BasicBlock, is_main: bool) -> CompileResult<()> {
        self.result.push(Mb8Asm::Sublabel(format!("BB{}", bb.id.0)));

        for inst in &bb.instructions {
            match inst {
                IRInstruction::LoadImm { value, width: _ } => {
                    self.result.push(Mb8Asm::Ldi {
                        register: "R0".to_string(),
                        value: *value,
                    });
                    self.result.push(Mb8Asm::Push {
                        register: "R0".to_string(),
                    });
                }
                IRInstruction::PushVar { symbol, width: _ } => {
                    let place = self.layout.lookup(*symbol).ok_or_else(|| todo!())?;
                    match place {
                        Place::Global { address } => {
                            self.result.push(Mb8Asm::Ld {
                                register: "R0".to_string(),
                                address: *address,
                            });
                        }
                        Place::StaticFrame { offset } => {
                            self.result.push(Mb8Asm::Ld {
                                register: "R0".to_string(),
                                address: *offset,
                            });
                        }
                    }

                    self.result.push(Mb8Asm::Push {
                        register: "R0".to_string(),
                    });
                }
                IRInstruction::StoreVar { symbol, width: _ } => {
                    self.result.push(Mb8Asm::Pop {
                        register: "R0".to_string(),
                    });

                    let place = self.layout.lookup(*symbol).ok_or_else(|| todo!())?;
                    match place {
                        Place::Global { address } => {
                            self.result.push(Mb8Asm::St {
                                address: *address,
                                register: "R0".to_string(),
                            });
                        }
                        Place::StaticFrame { offset } => {
                            self.result.push(Mb8Asm::St {
                                address: *offset,
                                register: "R0".to_string(),
                            });
                        }
                    }
                }
                IRInstruction::Add { width: _ } => {
                    self.result.push(Mb8Asm::Pop {
                        register: "R0".to_string(),
                    });
                    self.result.push(Mb8Asm::Pop {
                        register: "R1".to_string(),
                    });
                    self.result.push(Mb8Asm::Add {
                        dst: "R0".to_string(),
                        src: "R1".to_string(),
                    });
                    self.result.push(Mb8Asm::Push {
                        register: "R0".to_string(),
                    });
                }
                IRInstruction::Sub { width: _ } => {
                    self.result.push(Mb8Asm::Pop {
                        register: "R0".to_string(),
                    });
                    self.result.push(Mb8Asm::Pop {
                        register: "R1".to_string(),
                    });
                    self.result.push(Mb8Asm::Sub {
                        dst: "R0".to_string(),
                        src: "R1".to_string(),
                    });
                    self.result.push(Mb8Asm::Push {
                        register: "R0".to_string(),
                    });
                }
                IRInstruction::Mul { width: _ } => {
                    self.result.push(Mb8Asm::Pop {
                        register: "R0".to_string(),
                    });
                    self.result.push(Mb8Asm::Pop {
                        register: "R1".to_string(),
                    });
                    self.result.push(Mb8Asm::Mul {
                        dst: "R0".to_string(),
                        src: "R1".to_string(),
                    });
                    self.result.push(Mb8Asm::Push {
                        register: "R0".to_string(),
                    });
                }
                IRInstruction::Div { width: _ } => {
                    self.result.push(Mb8Asm::Pop {
                        register: "R0".to_string(),
                    });
                    self.result.push(Mb8Asm::Pop {
                        register: "R1".to_string(),
                    });
                    self.result.push(Mb8Asm::Div {
                        dst: "R0".to_string(),
                        src: "R1".to_string(),
                    });
                    self.result.push(Mb8Asm::Push {
                        register: "R0".to_string(),
                    });
                }
                IRInstruction::Eq { width: _ } | IRInstruction::Neg { width: _ } => {}
                IRInstruction::Call { symbol, argc } => {
                    let symbol = self.ctx.lookup(*symbol).ok_or_else(|| todo!())?;
                    let type_kind = self
                        .ctx
                        .type_table
                        .lookup(symbol.ty)
                        .ok_or_else(|| todo!())?;
                    for argn in 0..*argc {
                        self.result.push(Mb8Asm::Pop {
                            register: format!("R{argn}"),
                        });
                    }
                    self.result.push(Mb8Asm::Call(symbol.name));
                    let TypeKind::Function { params: _, ret } = type_kind else {
                        unimplemented!()
                    };
                    let ret_type_kind = self.ctx.type_table.lookup(*ret).ok_or_else(|| todo!())?;
                    match ret_type_kind {
                        TypeKind::Void => {}
                        _ => {
                            self.result.push(Mb8Asm::Push {
                                register: "R0".to_string(),
                            });
                        }
                    }
                }
            }
        }

        match bb.terminator {
            BasicBlockTerminator::Branch {
                then_branch,
                else_branch,
            } => {
                self.result.push(Mb8Asm::Pop {
                    register: "R0".to_string(),
                });
                self.result
                    .push(Mb8Asm::Jzr(format!("BB{}", else_branch.0)));
                self.result
                    .push(Mb8Asm::Jmp(format!("BB{}", then_branch.0)));
            }
            BasicBlockTerminator::Jmp { next } => {
                self.result.push(Mb8Asm::Jmp(format!("BB{}", next.0)));
            }
            BasicBlockTerminator::Ret { void } => {
                if is_main {
                    // SYS_EXIT
                    self.result.push(Mb8Asm::Ldi {
                        register: "R0".to_string(),
                        value: 0x0F,
                    });
                    self.result.push(Mb8Asm::Call("0xE500".to_string()));
                } else {
                    if !void {
                        self.result.push(Mb8Asm::Pop {
                            register: "R0".to_string(),
                        });
                    }

                    self.result.push(Mb8Asm::Ret);
                }
            }
        }

        Ok(())
    }

    /// Generate mb8 assembly for the provided IR function.
    ///
    /// # Errors
    /// Returns an error if symbol lookups fail or the writer cannot emit output.
    pub fn codegen_function(&mut self, function: &IRFunction, is_main: bool) -> CompileResult<()> {
        let symbol = self.ctx.lookup(function.id).ok_or_else(|| todo!())?;
        self.result.push(Mb8Asm::Label(symbol.name.clone()));

        for (index, symbol_id) in function.params.iter().enumerate() {
            let symbol = self.layout.lookup(*symbol_id).ok_or_else(|| todo!())?;
            let Place::StaticFrame { offset } = symbol else {
                unimplemented!()
            };
            self.result.push(Mb8Asm::St {
                register: format!("R{index}"),
                address: *offset,
            });
        }

        for bb in &function.basic_blocks {
            self.codegen_basic_block(bb, is_main)?;
        }

        Ok(())
    }

    /// Generate mb8 assembly for the provided IR program.
    ///
    /// # Errors
    /// Returns an error if symbol lookups fail or the writer cannot emit output.
    pub fn codegen(&mut self, ir: &IRProgram) -> CompileResult<Vec<Mb8Asm>> {
        for function in &ir.functions {
            let symbol = self.ctx.lookup(function.id).ok_or_else(|| todo!())?;
            if symbol.name == "main" {
                self.codegen_function(function, true)?;
                break;
            }
        }
        for function in &ir.functions {
            let symbol = self.ctx.lookup(function.id).ok_or_else(|| todo!())?;
            if symbol.name == "main" {
                continue;
            }
            self.codegen_function(function, false)?;
        }
        Ok(self.result.clone())
    }
}
