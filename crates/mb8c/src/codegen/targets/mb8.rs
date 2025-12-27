use crate::{
    codegen::writter::ProgramWriter,
    context::CompileContext,
    error::{CompileError, CompileResult},
    ir::instructions::{IRInstruction, IRProgram},
    pipeline::CompilerPipe,
};

#[derive(Debug)]
pub struct Mb8Codegen {
    ctx: CompileContext,
    writter: ProgramWriter,
}

impl CompilerPipe for Mb8Codegen {
    type Prev = (IRProgram, CompileContext);
    type Next = String;

    fn execute(prev: &Self::Prev) -> CompileResult<Self::Next, Vec<CompileError>> {
        let (ir, ctx) = prev;
        let mut codegen = Mb8Codegen {
            ctx: ctx.clone(),
            writter: ProgramWriter::default(),
        };
        codegen.codegen(ir).map_err(|err| vec![err])
    }
}

impl Mb8Codegen {
    pub fn codegen(&mut self, ir: &IRProgram) -> CompileResult<String> {
        for function in &ir.functions {
            let symbol = self.ctx.lookup(function.id).ok_or_else(|| todo!())?;
            self.writter.label(format!("{}", symbol.name))?;

            for bb in &function.basic_blocks {
                self.writter.sublabel(format!("BB{}", bb.id.0))?;

                for inst in &bb.instructions {
                    match inst {
                        IRInstruction::LoadImm { value, width: _ } => {
                            self.writter.emit(format!("LDI R0 {}", value))?;
                            self.writter.emit("PUSH R0")?;
                        }
                        IRInstruction::PushVar { symbol, width: _ } => {
                            // let symbol = self.ctx.lookup(*symbol).ok_or_else(|| todo!())?;
                            self.writter.emit(format!("LD R0 [ADDR]"))?;
                            self.writter.emit("PUSH R0")?;
                        }
                        IRInstruction::StoreVar { symbol, width: _ } => {
                            // let symbol = self.ctx.lookup(*symbol).ok_or_else(|| todo!())?;
                            self.writter.emit("POP R0")?;
                            self.writter.emit(format!("ST [ADDR] R0"))?;
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
                        IRInstruction::Eq { width: _ } => {}
                        IRInstruction::Neg { width: _ } => {}
                        IRInstruction::Call { symbol: _, argc: _ } => {}
                    }
                }
            }
        }
        Ok(self.writter.finish())
    }
}
