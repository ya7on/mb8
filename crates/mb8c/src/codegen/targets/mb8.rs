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

impl PhysicalRegister {
    fn to_string(&self) -> String {
        match self {
            PhysicalRegister::R0 => "R0".to_string(),
            PhysicalRegister::R1 => "R1".to_string(),
            PhysicalRegister::R2 => "R2".to_string(),
            PhysicalRegister::R3 => "R3".to_string(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Location {
    Register(PhysicalRegister),
    Spilled(usize),
}

#[derive(Debug)]
pub struct RegisterAllocator {
    // Instruction index
    uses: HashMap<VirtualRegister, Vec<usize>>,
    // defs: HashMap<VirtualRegister, Vec<usize>>,
    location: HashMap<VirtualRegister, Location>,
    offsets: HashMap<VirtualRegister, usize>,
    free: Vec<PhysicalRegister>,
    offset: usize,
}

impl RegisterAllocator {
    pub fn new(basic_block: &BasicBlock) -> Self {
        let mut uses = HashMap::<VirtualRegister, Vec<usize>>::new();
        let location = HashMap::<VirtualRegister, Location>::new();
        let free = vec![
            PhysicalRegister::R0,
            PhysicalRegister::R1,
            PhysicalRegister::R2,
            PhysicalRegister::R3,
        ];

        for (index, instruction) in basic_block.instructions.iter().enumerate() {
            match instruction {
                IRInstruction::LoadlArg { .. }
                | IRInstruction::StorelArg { .. }
                | IRInstruction::LoadImm { .. }
                | IRInstruction::Load { .. } => {
                    continue;
                }
                IRInstruction::Store { src, mem: _, ty: _ } => {
                    uses.entry(*src).or_default().push(index);
                }
                IRInstruction::Add {
                    dst: _,
                    lhs,
                    rhs,
                    ty: _,
                }
                | IRInstruction::Sub {
                    dst: _,
                    lhs,
                    rhs,
                    ty: _,
                }
                | IRInstruction::Mul {
                    dst: _,
                    lhs,
                    rhs,
                    ty: _,
                }
                | IRInstruction::Div {
                    dst: _,
                    lhs,
                    rhs,
                    ty: _,
                }
                | IRInstruction::Cmp {
                    dst: _,
                    lhs,
                    rhs,
                    ty: _,
                } => {
                    uses.entry(*lhs).or_default().push(index);
                    uses.entry(*rhs).or_default().push(index);
                }
                IRInstruction::Neg { dst: _, src, ty: _ } => {
                    uses.entry(*src).or_default().push(index);
                }
                IRInstruction::Call {
                    result: _,
                    label: _,
                    args,
                    ty: _,
                } => {
                    for arg in args {
                        uses.entry(*arg).or_default().push(index);
                    }
                }
            }
        }
        Self {
            uses,
            free,
            location,
            offsets: HashMap::new(),
            offset: 0,
        }
    }

    fn alloc_dst(
        &mut self,
        virtual_register: VirtualRegister,
        current_index: usize,
        base: usize,
        writer: &mut ProgramWriter,
    ) -> CompileResult<PhysicalRegister> {
        if let Some(Location::Register(register)) = self.location.get(&virtual_register).cloned() {
            return Ok(register);
        }

        if let Some(register) = self.free.pop() {
            self.location
                .insert(virtual_register, Location::Register(register));
            return Ok(register);
        }

        let (victim, register, next_use) = self
            .location
            .iter()
            .filter_map(|(vreg, location)| match location {
                Location::Register(register) => {
                    let next_use = self
                        .uses
                        .get(vreg)
                        .and_then(|used| {
                            used.iter()
                                .filter(|index| **index >= current_index)
                                .min()
                                .copied()
                        })
                        .unwrap_or(usize::MAX);
                    Some((*vreg, *register, next_use))
                }
                Location::Spilled(_) => None,
            })
            .max_by_key(|(_, _, next_use)| *next_use)
            .ok_or_else(|| CompileError::InternalError {
                message: "No registers available".to_string(),
            })?;

        if next_use != usize::MAX {
            let offset = *self.offsets.entry(victim).or_insert_with(|| {
                let offset = self.offset;
                self.offset += 1; // TODO: size
                offset
            });

            writer.emit(format!(
                "ST [{} + {}] {}",
                base,
                offset,
                register.to_string()
            ))?;
            self.location.insert(victim, Location::Spilled(offset));
        } else {
            self.location.remove(&victim);
        }

        self.location
            .insert(virtual_register, Location::Register(register));
        Ok(register)
    }

    fn ensure_in_reg(
        &mut self,
        virtual_register: VirtualRegister,
        current_index: usize,
        base: usize,
        writer: &mut ProgramWriter,
    ) -> CompileResult<PhysicalRegister> {
        if let Some(uses) = self.uses.get_mut(&virtual_register) {
            if let Some(position) = uses.iter().position(|index| *index == current_index) {
                uses.remove(position);
            }
        }

        match self.location.get(&virtual_register).cloned() {
            Some(Location::Register(physical_register)) => {
                return Ok(physical_register);
            }
            Some(Location::Spilled(offset)) => {
                let register = if let Some(register) = self.free.pop() {
                    register
                } else {
                    let (victim, register, next_use) = self
                        .location
                        .iter()
                        .filter_map(|(vreg, location)| match location {
                            Location::Register(register) => {
                                let next_use = self
                                    .uses
                                    .get(vreg)
                                    .and_then(|used| {
                                        used.iter()
                                            .filter(|index| **index >= current_index)
                                            .min()
                                            .copied()
                                    })
                                    .unwrap_or(usize::MAX);
                                Some((*vreg, *register, next_use))
                            }
                            Location::Spilled(_) => None,
                        })
                        .max_by_key(|(_, _, next_use)| *next_use)
                        .ok_or_else(|| CompileError::InternalError {
                            message: "No registers available".to_string(),
                        })?;

                    if next_use != usize::MAX {
                        let location = *self.offsets.entry(victim).or_insert_with(|| {
                            let offset = self.offset;
                            self.offset += 1; // TODO: size
                            offset
                        });
                        writer.emit(format!(
                            "ST [{} + {}] {}",
                            base,
                            location,
                            register.to_string()
                        ))?;
                        self.location.insert(victim, Location::Spilled(location));
                    } else {
                        self.location.remove(&victim);
                    }

                    register
                };

                self.location
                    .insert(virtual_register, Location::Register(register));
                writer.emit(format!(
                    "LD {}, [{} + {}]",
                    register.to_string(),
                    base,
                    offset
                ))?;
                return Ok(register);
            }
            None => {}
        };

        Err(CompileError::InternalError {
            message: "Register not allocated".to_string(),
        })
    }
}

#[derive(Debug, Default)]
pub struct Mb8Codegen {
    writer: ProgramWriter,
}

impl Mb8Codegen {
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
        let spill_base = offset + function.size;
        let mut spilled = 0;

        for basic_block in &function.basic_blocks {
            let mut register_allocator = RegisterAllocator::new(basic_block);

            let basic_block_label =
                ProgramWriter::basic_block_label(&function.name, basic_block.id.0);
            self.writer.sublabel(&basic_block_label)?;

            for (index, instruction) in basic_block.instructions.iter().enumerate() {
                self.codegen_instruction(
                    instruction,
                    offset,
                    spill_base,
                    index,
                    &mut register_allocator,
                )?;
                spilled = spilled.max(register_allocator.offset);
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
                    let register = register_allocator.ensure_in_reg(
                        condition,
                        basic_block.instructions.len(),
                        spill_base,
                        &mut self.writer,
                    )?;
                    let then_block_label =
                        ProgramWriter::basic_block_label(&function.name, then_branch.0);
                    let else_block_label =
                        ProgramWriter::basic_block_label(&function.name, else_branch.0);
                    self.writer
                        .emit(format!("CMPI {} 0", register.to_string()))?;
                    self.writer.emit(format!("JNZR [.{then_block_label}]"))?;
                    self.writer.emit(format!("JZR [.{else_block_label}]"))?;
                    spilled = spilled.max(register_allocator.offset);
                }
                BasicBlockTerminator::Ret { value } => {
                    if let Some(value) = value {
                        let register = register_allocator.ensure_in_reg(
                            value,
                            basic_block.instructions.len(),
                            spill_base,
                            &mut self.writer,
                        )?;
                        if register != PhysicalRegister::R0 {
                            self.writer
                                .emit(format!("MOV R0 {}", register.to_string()))?;
                        }
                    }
                    self.writer.emit("RET")?;
                }
            }
        }

        Ok(spilled)
    }

    fn codegen_instruction(
        &mut self,
        instruction: &IRInstruction,
        base: usize,
        spill_base: usize,
        current_index: usize,
        register_allocator: &mut RegisterAllocator,
    ) -> CompileResult<()> {
        match instruction {
            IRInstruction::LoadlArg {
                ty: _,
                index: _, // TODO
                mem,
            } => {
                let Mem::Local { offset } = mem;
                self.writer.emit("POP R0")?;
                self.writer.emit(format!("ST [0x{:X}] R0", base + offset))?;
                Ok(())
            }
            IRInstruction::StorelArg {
                register,
                ty: _,
                index: _,
            } => {
                let register = register_allocator.ensure_in_reg(
                    *register,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                self.writer.emit(format!("PUSH {}", register.to_string()))?;
                Ok(())
            }
            IRInstruction::LoadImm {
                register, value, ..
            } => {
                let register = register_allocator.alloc_dst(
                    *register,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                self.writer
                    .emit(format!("LDI {} {:#X}", register.to_string(), value))?;
                Ok(())
            }
            IRInstruction::Store { src, mem, ty: _ } => {
                let Mem::Local { offset } = mem;
                let register = register_allocator.ensure_in_reg(
                    *src,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                self.writer.emit(format!(
                    "ST [0x{:X}] {}",
                    base + offset,
                    register.to_string()
                ))?;
                Ok(())
            }
            IRInstruction::Load { dst, mem, ty: _ } => {
                let Mem::Local { offset } = mem;
                let register = register_allocator.alloc_dst(
                    *dst,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                self.writer.emit(format!(
                    "LD {} [0x{:X}]",
                    register.to_string(),
                    base + offset
                ))?;
                Ok(())
            }
            IRInstruction::Add {
                dst,
                lhs,
                rhs,
                ty: _,
            } => {
                let lhs = register_allocator.ensure_in_reg(
                    *lhs,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                let rhs = register_allocator.ensure_in_reg(
                    *rhs,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                let dst = register_allocator.alloc_dst(
                    *dst,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                if dst != lhs {
                    self.writer
                        .emit(format!("MOV {} {}", dst.to_string(), lhs.to_string()))?;
                }
                self.writer
                    .emit(format!("ADD {} {}", dst.to_string(), rhs.to_string()))?;
                Ok(())
            }
            IRInstruction::Sub {
                dst,
                lhs,
                rhs,
                ty: _,
            } => {
                let lhs = register_allocator.ensure_in_reg(
                    *lhs,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                let rhs = register_allocator.ensure_in_reg(
                    *rhs,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                let dst = register_allocator.alloc_dst(
                    *dst,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                if dst != lhs {
                    self.writer
                        .emit(format!("MOV {} {}", dst.to_string(), lhs.to_string()))?;
                }
                self.writer
                    .emit(format!("SUB {} {}", dst.to_string(), rhs.to_string()))?;
                Ok(())
            }
            IRInstruction::Mul {
                dst,
                lhs,
                rhs,
                ty: _,
            } => {
                let lhs = register_allocator.ensure_in_reg(
                    *lhs,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                let rhs = register_allocator.ensure_in_reg(
                    *rhs,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                let dst = register_allocator.alloc_dst(
                    *dst,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                self.writer.emit(format!(
                    "MUL {} {} {}",
                    dst.to_string(),
                    lhs.to_string(),
                    rhs.to_string()
                ))?;
                Ok(())
            }
            IRInstruction::Div {
                dst,
                lhs,
                rhs,
                ty: _,
            } => {
                let lhs = register_allocator.ensure_in_reg(
                    *lhs,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                let rhs = register_allocator.ensure_in_reg(
                    *rhs,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                let dst = register_allocator.alloc_dst(
                    *dst,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                self.writer.emit(format!(
                    "DIV {} {} {}",
                    dst.to_string(),
                    lhs.to_string(),
                    rhs.to_string()
                ))?;
                Ok(())
            }
            IRInstruction::Cmp {
                dst,
                lhs,
                rhs,
                ty: _,
            } => {
                let lhs = register_allocator.ensure_in_reg(
                    *lhs,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                let rhs = register_allocator.ensure_in_reg(
                    *rhs,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                let dst = register_allocator.alloc_dst(
                    *dst,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                let true_label = format!("cmp_true_{}", current_index);
                let end_label = format!("cmp_end_{}", current_index);
                self.writer
                    .emit(format!("CMP {} {}", lhs.to_string(), rhs.to_string()))?;
                self.writer.emit(format!("ZERO {}", dst.to_string()))?;
                self.writer.emit(format!("JNZR [.{true_label}]"))?;
                self.writer.emit(format!("JR [.{end_label}]"))?;
                self.writer.sublabel(true_label)?;
                self.writer.emit(format!("LDI {} 1", dst.to_string()))?;
                self.writer.sublabel(end_label)?;
                Ok(())
            }
            IRInstruction::Neg { dst, src, ty: _ } => {
                let src = register_allocator.ensure_in_reg(
                    *src,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                let dst = register_allocator.alloc_dst(
                    *dst,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                self.writer.emit(format!("ZERO {}", dst.to_string()))?;
                self.writer
                    .emit(format!("SUB {} {}", dst.to_string(), src.to_string()))?;
                Ok(())
            }
            IRInstruction::Call {
                result,
                label,
                args: _,
                ty: _,
            } => {
                let dst = register_allocator.alloc_dst(
                    *result,
                    current_index,
                    spill_base,
                    &mut self.writer,
                )?;
                self.writer.emit(format!("CALL [.{label}]"))?;
                if dst != PhysicalRegister::R0 {
                    self.writer.emit(format!("MOV {} R0", dst.to_string()))?;
                }
                Ok(())
            }
        }
    }
}
