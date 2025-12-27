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
                IRInstruction::LoadImm { value, width } => match width {
                    1 => {
                        self.result.push(Mb8Asm::Ldi {
                            register: "R0".to_string(),
                            value: *value as u8,
                        });
                        self.result.push(Mb8Asm::Push {
                            register: "R0".to_string(),
                        });
                    }
                    2 => {
                        self.result.push(Mb8Asm::Ldi {
                            register: "R0".to_string(),
                            value: (*value >> 8) as u8,
                        });
                        self.result.push(Mb8Asm::Ldi {
                            register: "R1".to_string(),
                            value: (*value & 0xFF) as u8,
                        });
                        self.result.push(Mb8Asm::Push {
                            register: "R0".to_string(),
                        });
                        self.result.push(Mb8Asm::Push {
                            register: "R1".to_string(),
                        });
                    }
                    _ => {
                        unimplemented!()
                    }
                },
                IRInstruction::PushVar { symbol, width } => {
                    let place = self.layout.lookup(*symbol).ok_or_else(|| todo!())?;
                    match width {
                        1 => {
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
                        2 => {
                            match place {
                                Place::Global { address } => {
                                    self.result.push(Mb8Asm::Ld {
                                        register: "R0".to_string(),
                                        address: *address,
                                    });
                                    self.result.push(Mb8Asm::Ld {
                                        register: "R1".to_string(),
                                        address: *address + 1,
                                    });
                                }
                                Place::StaticFrame { offset } => {
                                    self.result.push(Mb8Asm::Ld {
                                        register: "R0".to_string(),
                                        address: *offset,
                                    });
                                    self.result.push(Mb8Asm::Ld {
                                        register: "R1".to_string(),
                                        address: *offset + 1,
                                    });
                                }
                            }

                            self.result.push(Mb8Asm::Push {
                                register: "R0".to_string(),
                            });
                            self.result.push(Mb8Asm::Push {
                                register: "R1".to_string(),
                            });
                        }
                        _ => unimplemented!(),
                    }
                }
                IRInstruction::StoreVar { symbol, width } => {
                    let place = self.layout.lookup(*symbol).ok_or_else(|| todo!())?;
                    match width {
                        1 => {
                            self.result.push(Mb8Asm::Pop {
                                register: "R0".to_string(),
                            });

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
                        2 => {
                            self.result.push(Mb8Asm::Pop {
                                register: "R1".to_string(),
                            });
                            self.result.push(Mb8Asm::Pop {
                                register: "R0".to_string(),
                            });

                            match place {
                                Place::Global { address } => {
                                    self.result.push(Mb8Asm::St {
                                        address: *address,
                                        register: "R0".to_string(),
                                    });
                                    self.result.push(Mb8Asm::St {
                                        address: *address + 1,
                                        register: "R1".to_string(),
                                    });
                                }
                                Place::StaticFrame { offset } => {
                                    self.result.push(Mb8Asm::St {
                                        address: *offset,
                                        register: "R0".to_string(),
                                    });
                                    self.result.push(Mb8Asm::St {
                                        address: *offset + 1,
                                        register: "R1".to_string(),
                                    });
                                }
                            }
                        }
                        _ => unimplemented!(),
                    }
                }
                IRInstruction::Add { width } => match width {
                    1 => {
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
                    2 => {
                        self.result.push(Mb8Asm::Pop {
                            register: "R1".to_string(),
                        });
                        self.result.push(Mb8Asm::Pop {
                            register: "R0".to_string(),
                        });
                        self.result.push(Mb8Asm::Pop {
                            register: "R4".to_string(),
                        });
                        self.result.push(Mb8Asm::Pop {
                            register: "R3".to_string(),
                        });

                        self.result.push(Mb8Asm::Add {
                            dst: "R1".to_string(),
                            src: "R4".to_string(),
                        });
                        let sublabel_id = self.result.len();
                        self.result
                            .push(Mb8Asm::Jncr(format!(".no_carry_{sublabel_id}")));
                        self.result.push(Mb8Asm::Inc {
                            register: "R0".to_string(),
                        });
                        self.result
                            .push(Mb8Asm::Sublabel(format!("no_carry_{sublabel_id}")));
                        self.result.push(Mb8Asm::Add {
                            dst: "R0".to_string(),
                            src: "R3".to_string(),
                        });
                        self.result.push(Mb8Asm::Push {
                            register: "R0".to_string(),
                        });
                        self.result.push(Mb8Asm::Push {
                            register: "R1".to_string(),
                        });
                    }
                    _ => unimplemented!(),
                },
                IRInstruction::Sub { width } => match width {
                    1 => {
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
                    2 => {
                        self.result.push(Mb8Asm::Pop {
                            register: "R1".to_string(),
                        });
                        self.result.push(Mb8Asm::Pop {
                            register: "R0".to_string(),
                        });
                        self.result.push(Mb8Asm::Pop {
                            register: "R4".to_string(),
                        });
                        self.result.push(Mb8Asm::Pop {
                            register: "R3".to_string(),
                        });

                        self.result.push(Mb8Asm::Sub {
                            dst: "R1".to_string(),
                            src: "R4".to_string(),
                        });
                        let sublabel_id = self.result.len();
                        self.result
                            .push(Mb8Asm::Jncr(format!(".no_carry_{sublabel_id}")));
                        self.result.push(Mb8Asm::Dec {
                            register: "R0".to_string(),
                        });
                        self.result
                            .push(Mb8Asm::Sublabel(format!("no_carry_{sublabel_id}")));
                        self.result.push(Mb8Asm::Sub {
                            dst: "R0".to_string(),
                            src: "R3".to_string(),
                        });
                        self.result.push(Mb8Asm::Push {
                            register: "R0".to_string(),
                        });
                        self.result.push(Mb8Asm::Push {
                            register: "R1".to_string(),
                        });
                    }
                    _ => unimplemented!(),
                },
                IRInstruction::Mul { width: _ } => {
                    todo!()
                }
                IRInstruction::Div { width: _ } => {
                    todo!()
                }
                IRInstruction::Eq { width } => match width {
                    1 => {
                        self.result.push(Mb8Asm::Pop {
                            register: "R1".to_string(),
                        });
                        self.result.push(Mb8Asm::Pop {
                            register: "R0".to_string(),
                        });
                        self.result.push(Mb8Asm::Cmp {
                            dst: "R0".to_string(),
                            src: "R1".to_string(),
                        });
                        let sublabel_id = self.result.len();
                        self.result.push(Mb8Asm::Ldi {
                            register: "R0".to_string(),
                            value: 1,
                        });
                        self.result
                            .push(Mb8Asm::Jnzr(format!(".not_equal_{sublabel_id}")));
                        self.result.push(Mb8Asm::Ldi {
                            register: "R0".to_string(),
                            value: 0,
                        });
                        self.result
                            .push(Mb8Asm::Sublabel(format!("not_equal_{sublabel_id}")));

                        self.result.push(Mb8Asm::Push {
                            register: "R0".to_string(),
                        });
                    }
                    2 => {
                        self.result.push(Mb8Asm::Pop {
                            register: "R1".to_string(),
                        });
                        self.result.push(Mb8Asm::Pop {
                            register: "R0".to_string(),
                        });
                        self.result.push(Mb8Asm::Pop {
                            register: "R4".to_string(),
                        });
                        self.result.push(Mb8Asm::Pop {
                            register: "R3".to_string(),
                        });
                        let sublabel_id = self.result.len();
                        self.result.push(Mb8Asm::Cmp {
                            dst: "R0".to_string(),
                            src: "R3".to_string(),
                        });
                        self.result
                            .push(Mb8Asm::Jnzr(format!(".not_equal_{sublabel_id}")));
                        self.result.push(Mb8Asm::Cmp {
                            dst: "R1".to_string(),
                            src: "R4".to_string(),
                        });
                        self.result
                            .push(Mb8Asm::Jnzr(format!(".not_equal_{sublabel_id}")));
                        self.result.push(Mb8Asm::Ldi {
                            register: "R0".to_string(),
                            value: 0,
                        });
                        self.result
                            .push(Mb8Asm::Jmp(format!(".eq_end_{sublabel_id}")));
                        self.result
                            .push(Mb8Asm::Sublabel(format!("not_equal_{sublabel_id}")));
                        self.result.push(Mb8Asm::Ldi {
                            register: "R0".to_string(),
                            value: 1,
                        });
                        self.result
                            .push(Mb8Asm::Sublabel(format!("eq_end_{sublabel_id}")));
                        self.result.push(Mb8Asm::Push {
                            register: "R0".to_string(),
                        });
                    }
                    _ => unimplemented!(),
                },
                IRInstruction::Neg { width: _ } => {
                    todo!()
                }
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
                        TypeKind::Unsigned8 | TypeKind::Bool => {
                            self.result.push(Mb8Asm::Push {
                                register: "R0".to_string(),
                            });
                        }
                        TypeKind::Unsigned16 => {
                            self.result.push(Mb8Asm::Push {
                                register: "R0".to_string(),
                            });
                            self.result.push(Mb8Asm::Push {
                                register: "R1".to_string(),
                            });
                        }
                        TypeKind::Function { .. } => {
                            unimplemented!()
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
                self.result.push(Mb8Asm::Ldi {
                    register: "R1".to_string(),
                    value: 0,
                });
                self.result.push(Mb8Asm::Cmp {
                    dst: "R0".to_string(),
                    src: "R1".to_string(),
                });
                self.result
                    .push(Mb8Asm::Jnzr(format!(".BB{}", else_branch.0)));
                self.result
                    .push(Mb8Asm::Jmp(format!(".BB{}", then_branch.0)));
            }
            BasicBlockTerminator::Jmp { next } => {
                self.result.push(Mb8Asm::Jmp(format!(".BB{}", next.0)));
            }
            BasicBlockTerminator::Ret { width } => {
                if is_main {
                    // SYS_EXIT
                    self.result.push(Mb8Asm::Ldi {
                        register: "R0".to_string(),
                        value: 0x0F,
                    });
                    self.result.push(Mb8Asm::Call("0xE500".to_string()));
                } else {
                    match width {
                        0 => {}
                        1 => {
                            self.result.push(Mb8Asm::Pop {
                                register: "R0".to_string(),
                            });
                        }
                        2 => {
                            self.result.push(Mb8Asm::Pop {
                                register: "R1".to_string(),
                            });
                            self.result.push(Mb8Asm::Pop {
                                register: "R0".to_string(),
                            });
                        }
                        _ => unimplemented!(),
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
