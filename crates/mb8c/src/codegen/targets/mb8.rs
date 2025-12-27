use crate::{
    codegen::writter::ProgramWriter,
    context::CompileContext,
    error::{CompileError, CompileResult},
    ir::instructions::{IRFunction, IRInstruction, IRProgram},
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
    /// Generate mb8 assembly for the provided IR function.
    ///
    /// # Errors
    /// Returns an error if symbol lookups fail or the writer cannot emit output.
    pub fn codegen_function(&mut self, function: &IRFunction) -> CompileResult<()> {
        let symbol = self.ctx.lookup(function.id).ok_or_else(|| todo!())?;
        self.writter.label(symbol.name.to_string())?;

        for bb in &function.basic_blocks {
            self.writter.sublabel(format!("BB{}", bb.id.0))?;

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
                            Place::StackFrame { offset } => {
                                self.writter.emit(format!("LD R0 [FPH:FPL + {offset}]"))?;
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
                                self.writter.emit(format!("LD R0 [0x{address:X}]"))?;
                            }
                            Place::StackFrame { offset } => {
                                self.writter.emit(format!("LD R0 [FPH:FPL + {offset}]"))?;
                            }
                            Place::StaticFrame { offset } => {
                                self.writter.emit(format!("LD R0 [0x{offset:X}]"))?;
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
                    IRInstruction::Call { symbol, argc: _ } => {
                        let symbol = self.ctx.lookup(*symbol).ok_or_else(|| todo!())?;
                        let label = symbol.name;
                        self.writter.emit(format!("CALL [{label}]"))?;
                    }
                }
            }
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
                self.codegen_function(function)?;
                break;
            }
        }
        for function in &ir.functions {
            let symbol = self.ctx.lookup(function.id).ok_or_else(|| todo!())?;
            if symbol.name == "main" {
                continue;
            }
            self.codegen_function(function)?;
        }
        Ok(self.writter.finish())
    }
}
