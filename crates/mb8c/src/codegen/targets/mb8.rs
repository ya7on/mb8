use std::{collections::HashMap, fmt::Write};

use crate::{
    error::{CompileError, CompileResult},
    ir::{
        BasicBlock, BasicBlockTerminator, IRFunction, IRInstruction, IRProgram, Mem,
        VirtualRegister,
    },
};

#[derive(Debug, Default)]
pub struct ProgramWriter {
    result: String,
}

impl ProgramWriter {
    fn emit(&mut self, value: impl ToString) -> CompileResult<()> {
        writeln!(self.result, "\t{}", value.to_string()).map_err(|_| CompileError::InternalError {
            message: "Codegen error".to_string(),
        })
    }

    fn label(&mut self, label: impl ToString) -> CompileResult<()> {
        writeln!(self.result, "{}:", label.to_string()).map_err(|_| CompileError::InternalError {
            message: "Codegen error".to_string(),
        })
    }

    fn sublabel(&mut self, sublabel: impl ToString) -> CompileResult<()> {
        writeln!(self.result, ".{}:", sublabel.to_string()).map_err(|_| {
            CompileError::InternalError {
                message: "Codegen error".to_string(),
            }
        })
    }

    fn basic_block_label(function_name: impl ToString, basic_block_id: impl ToString) -> String {
        format!(
            "{}_{}",
            function_name.to_string(),
            basic_block_id.to_string()
        )
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum PhysicalRegister {
    R0,
    R1,
    R2,
    R3,
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Location {
    Register(PhysicalRegister),
    Spilled(usize),
}

#[derive(Debug)]
pub struct RegisterAllocator {
    // Instruction index
    uses: HashMap<VirtualRegister, Vec<usize>>,
    defs: HashMap<VirtualRegister, Vec<usize>>,
    location: HashMap<VirtualRegister, Location>,
    free: Vec<PhysicalRegister>,
    offset: usize,
}

impl RegisterAllocator {
    pub fn new(basic_block: &BasicBlock) -> Self {
        let mut uses = HashMap::<VirtualRegister, Vec<usize>>::new();
        let mut defs = HashMap::<VirtualRegister, Vec<usize>>::new();
        let mut location = HashMap::<VirtualRegister, Location>::new();
        let mut free = vec![
            PhysicalRegister::R0,
            PhysicalRegister::R1,
            PhysicalRegister::R2,
            PhysicalRegister::R3,
        ];

        for (index, instruction) in basic_block.instructions.iter().enumerate() {
            match instruction {
                IRInstruction::LoadlArg { .. } => {
                    continue;
                }
                IRInstruction::StorelArg {
                    register,
                    ty: _,
                    index: _,
                }
                | IRInstruction::LoadImm {
                    register,
                    value: _,
                    ty: _,
                } => {
                    defs.entry(*register).or_default().push(index);
                }
                IRInstruction::Store { src, mem: _, ty: _ } => {
                    uses.entry(*src).or_default().push(index);
                }
                IRInstruction::Load { dst, mem: _, ty: _ } => {
                    defs.entry(*dst).or_default().push(index);
                }
                IRInstruction::Add {
                    dst,
                    lhs,
                    rhs,
                    ty: _,
                }
                | IRInstruction::Sub {
                    dst,
                    lhs,
                    rhs,
                    ty: _,
                }
                | IRInstruction::Mul {
                    dst,
                    lhs,
                    rhs,
                    ty: _,
                }
                | IRInstruction::Div {
                    dst,
                    lhs,
                    rhs,
                    ty: _,
                }
                | IRInstruction::Cmp {
                    dst,
                    lhs,
                    rhs,
                    ty: _,
                } => {
                    defs.entry(*dst).or_default().push(index);
                    uses.entry(*lhs).or_default().push(index);
                    uses.entry(*rhs).or_default().push(index);
                }
                IRInstruction::Neg { dst, src, ty: _ } => {
                    defs.entry(*dst).or_default().push(index);
                    uses.entry(*src).or_default().push(index);
                }
                IRInstruction::Call {
                    result,
                    label: _,
                    args,
                    ty: _,
                } => {
                    defs.entry(*result).or_default().push(index);
                    for arg in args {
                        uses.entry(*arg).or_default().push(index);
                    }
                }
            }
        }
        Self {
            uses,
            defs,
            free,
            location,
            offset: 0,
        }
    }

    fn ensure_in_reg(
        &mut self,
        virtual_register: VirtualRegister,
        current_index: usize,
        writer: &mut ProgramWriter,
    ) -> CompileResult<PhysicalRegister> {
        if let Some(Location::Register(register)) = self.location.get(&virtual_register) {
            return Ok(*register);
        };

        if !self.free.is_empty() {
            let register = self.free.pop().unwrap();
            self.location
                .insert(virtual_register, Location::Register(register));
            writer.emit("value")?;
            return Ok(register);
        }

        let victim = self
            .uses
            .iter()
            .map(|(vreg, used)| {
                (
                    vreg,
                    used.iter()
                        .filter(|index| **index >= current_index)
                        .map(|index| *index)
                        .min()
                        .unwrap_or_default(),
                )
            })
            .min_by_key(|(_, index)| *index)
            .map(|(vreg, _)| *vreg)
            .unwrap();
        self.location.insert(victim, Location::Spilled(self.offset));
        self.offset += 1;

        todo!()
    }
}

#[derive(Debug, Default)]
pub struct Mb8Codegen {
    writer: ProgramWriter,
}

impl Mb8Codegen {
    fn load_vreg(&mut self, dst: impl ToString, vreg: &VirtualRegister) -> CompileResult<()> {
        self.writer
            .emit(format!("LD {} [0x{}]", dst.to_string(), vreg.0))?;
        Ok(())
    }

    /// # Errors
    /// Returns error if there are codegen issues
    pub fn codegen(&mut self, ir: &IRProgram) -> CompileResult<String> {
        let mut offset = 0;
        for function in &ir.functions {
            let spilled = self.codegen_function(function, offset)?;
            offset += function.size + spilled;
        }

        Ok(self.writer.result.clone())
    }

    fn codegen_function(&mut self, function: &IRFunction, offset: usize) -> CompileResult<usize> {
        self.writer.label(&function.name)?;

        for basic_block in &function.basic_blocks {
            let register_allocator = RegisterAllocator::new(basic_block);

            let basic_block_label =
                ProgramWriter::basic_block_label(&function.name, basic_block.id.0);
            self.writer.sublabel(&basic_block_label)?;

            for instruction in &basic_block.instructions {
                println!("{instruction:?}");
                self.codegen_instruction(instruction, offset, &register_allocator)?;
            }

            match basic_block.terminator {
                BasicBlockTerminator::Jmp { next } => {
                    let next_block_label = ProgramWriter::basic_block_label(&function.name, next.0);
                    self.writer.emit(format!("JMP [.{next_block_label}]"))?;
                }
                BasicBlockTerminator::Branch {
                    condition,
                    then_branch,
                    else_branch,
                } => {
                    let then_block_label =
                        ProgramWriter::basic_block_label(&function.name, then_branch.0);
                    let else_block_label =
                        ProgramWriter::basic_block_label(&function.name, else_branch.0);
                    self.load_vreg("R0", &condition)?;
                    self.writer.emit("CMP R0 0")?;
                    self.writer.emit(format!("JCR [.{then_block_label}]"))?;
                    self.writer.emit(format!("JNCR [.{else_block_label}]"))?;
                }
                BasicBlockTerminator::Ret { value } => {
                    if let Some(value) = value {
                        self.load_vreg("R0", &value)?;
                    }
                    self.writer.emit("RET")?;
                }
            }
        }

        Ok(0)
    }

    fn codegen_instruction(
        &mut self,
        instruction: &IRInstruction,
        base: usize,
        register_allocator: &RegisterAllocator,
    ) -> CompileResult<()> {
        match instruction {
            IRInstruction::LoadlArg {
                ty: _,
                index: _, // TODO
                mem,
            } => {
                let Mem::Local { offset } = mem;
                self.writer.emit("POP R0")?;
                self.writer
                    .emit(format!("ST [0x{base:X} - 0x{offset:X}]"))?;
                Ok(())
            }
            IRInstruction::StorelArg {
                register,
                ty,
                index,
            } => {
                todo!()
            }
            IRInstruction::LoadImm { .. } => todo!(),
            IRInstruction::Store { .. } => todo!(),
            IRInstruction::Load { dst, mem, ty } => {
                //
                todo!()
            }
            IRInstruction::Add { .. } => todo!(),
            IRInstruction::Sub { .. } => todo!(),
            IRInstruction::Mul { .. } => todo!(),
            IRInstruction::Div { .. } => todo!(),
            IRInstruction::Cmp { .. } => todo!(),
            IRInstruction::Neg { .. } => todo!(),
            IRInstruction::Call { .. } => todo!(),
        }
    }
}
