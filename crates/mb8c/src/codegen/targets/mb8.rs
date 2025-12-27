use crate::{
    codegen::writter::ProgramWriter,
    context::CompileContext,
    error::{CompileError, CompileResult},
    ir::instructions::{BasicBlock, BasicBlockTerminator, IRFunction, IRInstruction, IRProgram},
    layout::{Layout, Place},
    pipeline::CompilerPipe,
};

#[derive(Debug)]
pub struct Mb8Codegen {
    ctx: CompileContext,
    layout: Layout,
    writter: ProgramWriter,
}

impl CompilerPipe for Mb8Codegen {
    type Prev = (IRProgram, CompileContext, Layout);
    type Next = String;

    fn execute(prev: &Self::Prev) -> CompileResult<Self::Next, Vec<CompileError>> {
        let (ir, ctx, layout) = prev;
        let mut codegen = Mb8Codegen {
            ctx: ctx.clone(),
            layout: layout.clone(),
            writter: ProgramWriter::default(),
        };
        codegen.codegen(ir).map_err(|err| vec![err])
    }
}

impl Mb8Codegen {
    /// Generate code for a basic block.
    ///
    /// # Errors
    /// This function will return an error if the basic block cannot be generated.
    pub fn codegen_basic_block(&mut self, bb: &BasicBlock, is_main: bool) -> CompileResult<()> {
        self.writter
            .sublabel(ProgramWriter::basic_block_label(bb.id.0))?;

        for inst in &bb.instructions {
            match inst {
                IRInstruction::LoadImm { value, width: _ } => {
                    self.writter.emit(format!("LDI R0 {value}"))?;
                    self.writter.emit("PUSH R0")?;
                }
                IRInstruction::PushVar { symbol, width: _ } => {
                    let place = self.layout.lookup(*symbol).ok_or_else(|| todo!())?;
                    match place {
                        Place::Global { address } => {
                            self.writter.emit(format!("LD R0 [0x{address:X}]"))?;
                        }
                        Place::StaticFrame { offset } => {
                            self.writter.emit(format!("LD R0 [0x{offset:X}]"))?;
                        }
                    }

                    self.writter.emit("PUSH R0")?;
                }
                IRInstruction::StoreVar { symbol, width: _ } => {
                    self.writter.emit("POP R0")?;

                    let place = self.layout.lookup(*symbol).ok_or_else(|| todo!())?;
                    match place {
                        Place::Global { address } => {
                            self.writter.emit(format!("ST [0x{address:X}] R0"))?;
                        }
                        Place::StaticFrame { offset } => {
                            self.writter.emit(format!("ST [0x{offset:X}] R0"))?;
                        }
                    }
                }
                IRInstruction::Add { width: _ } => {
                    self.writter.emit("POP R0")?;
                    self.writter.emit("POP R1")?;
                    self.writter.emit("ADD R0 R1")?;
                    self.writter.emit("PUSH R0")?;
                }
                IRInstruction::Sub { width: _ } => {
                    self.writter.emit("POP R0")?;
                    self.writter.emit("POP R1")?;
                    self.writter.emit("SUB R0 R1")?;
                    self.writter.emit("PUSH R0")?;
                }
                IRInstruction::Mul { width: _ } => {
                    self.writter.emit("POP R0")?;
                    self.writter.emit("POP R1")?;
                    self.writter.emit("MUL R0 R1")?;
                    self.writter.emit("PUSH R0")?;
                }
                IRInstruction::Div { width: _ } => {
                    self.writter.emit("POP R0")?;
                    self.writter.emit("POP R1")?;
                    self.writter.emit("DIV R0 R1")?;
                    self.writter.emit("PUSH R0")?;
                }
                IRInstruction::Eq { width: _ } | IRInstruction::Neg { width: _ } => {}
                IRInstruction::Call { symbol, argc } => {
                    let symbol = self.ctx.lookup(*symbol).ok_or_else(|| todo!())?;
                    let label = symbol.name;
                    for argn in 0..*argc {
                        self.writter.emit(format!("POP R{argn}"))?;
                    }
                    self.writter.emit(format!("CALL [{label}]"))?;
                }
            }
        }

        match bb.terminator {
            BasicBlockTerminator::Branch {
                then_branch,
                else_branch,
            } => {
                let then_label = ProgramWriter::basic_block_label(then_branch.0);
                let else_label = ProgramWriter::basic_block_label(else_branch.0);
                self.writter.emit("POP R0")?;
                self.writter.emit(format!("JZ R0 [{else_label}]"))?;
                self.writter.emit(format!("JMP [{then_label}]"))?;
            }
            BasicBlockTerminator::Jmp { next } => {
                let label = ProgramWriter::basic_block_label(next.0);
                self.writter.emit(format!("JMP [{label}]"))?;
            }
            BasicBlockTerminator::Ret { void: _ } => {
                if is_main {
                    self.writter.emit("LDI R0 0x0F")?;
                    self.writter.emit("CALL [0xE500]")?;
                } else {
                    self.writter.emit("RET")?;
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
        self.writter.label(symbol.name.clone())?;

        for (index, symbol_id) in function.params.iter().enumerate() {
            let symbol = self.layout.lookup(*symbol_id).ok_or_else(|| todo!())?;
            let Place::StaticFrame { offset } = symbol else {
                unimplemented!()
            };
            self.writter.emit(format!("ST [0x{offset:X}] R{index}"))?;
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
    pub fn codegen(&mut self, ir: &IRProgram) -> CompileResult<String> {
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
        Ok(self.writter.finish())
    }
}
