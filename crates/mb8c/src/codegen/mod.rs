use mb6::Mb8Asm;

use crate::{
    error::{CompileError, CompileResult},
    ir::{IRFunction, IROpcode, Reg},
};

pub mod mb6;

const BASE_ADDR: u16 = 0x0100;

#[derive(Debug)]
pub struct FunctionContext<'a> {
    function: &'a IRFunction,
    local_slots: Vec<u16>,
    reg_slots: Vec<u16>,
}

impl<'a> FunctionContext<'a> {
    #[must_use]
    pub fn new(function: &'a IRFunction) -> Self {
        FunctionContext {
            function,
            local_slots: Vec::new(),
            reg_slots: Vec::new(),
        }
    }

    fn layout_slots(&mut self) {
        let mut addr = BASE_ADDR;

        self.local_slots = Vec::with_capacity(self.function.locals.len());
        for local in &self.function.locals {
            self.local_slots.push(addr);
            addr += local.ty.size_in_bytes() as u16;
        }

        let num_regs = self
            .function
            .code
            .iter()
            .filter_map(|inst| inst.dst)
            .map(|r| r.0)
            .max()
            .map_or(0, |m| m + 1);

        self.reg_slots = Vec::with_capacity(num_regs as usize);
        for _ in 0..num_regs {
            self.reg_slots.push(addr);
            addr += 2;
        }
    }

    fn local_addr(&self, index: u32) -> u16 {
        self.local_slots[index as usize]
    }

    fn reg_addr(&self, reg: Reg) -> u16 {
        self.reg_slots[reg.0 as usize]
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn generate(&mut self) -> CompileResult<String> {
        self.layout_slots();

        let mut result = Mb8Asm::default();
        let func_name = &self.function.name;

        result.label(func_name)?;
        for instruction in &self.function.code {
            match &instruction.opcode {
                IROpcode::LoadImm { imm } => {
                    let dst = instruction.dst.ok_or_else(|| CompileError::InternalError {
                        message: "LoadImm must have dst".to_string(),
                    })?;
                    let addr = self.reg_addr(dst);

                    let imm_u = *imm as u16;
                    let lo = (imm_u & 0x00FF) as u8;
                    let hi = (imm_u >> 8) as u8;

                    result.ldi("R0", lo)?;
                    result.st_addr("R0", addr)?;

                    result.ldi("R0", hi)?;
                    result.st_addr("R0", addr + 1)?;
                }
                IROpcode::Return => {
                    if let Some(src) = instruction.dst {
                        let src_addr = self.reg_addr(src);
                        result.ld_addr("R0", src_addr)?;
                    }
                    result.ret()?;
                }
                IROpcode::Call { name, args } => {
                    for (i, reg) in args.iter().enumerate() {
                        let arg_addr = self.reg_addr(*reg);
                        let hw_reg = match i {
                            0 => "R0",
                            1 => "R1",
                            2 => "R2",
                            _ => {
                                return Err(CompileError::InternalError {
                                    message: "Too many arguments".to_string(),
                                })
                            }
                        };
                        result.ld_addr(hw_reg, arg_addr)?;
                    }

                    result.call(name)?;

                    if let Some(dst) = instruction.dst {
                        let dst_addr = self.reg_addr(dst);
                        result.st_addr("R0", dst_addr)?;
                    }
                }
                IROpcode::StoreLocal { local, size } => {
                    let src = instruction
                        .src1
                        .ok_or_else(|| CompileError::InternalError {
                            message: "StoreLocal must have src1".to_string(),
                        })?;
                    let src_addr = self.reg_addr(src);
                    let dst_addr = self.local_addr(*local);

                    match size {
                        1 => {
                            result.ld_addr("R0", src_addr)?;
                            result.st_addr("R0", dst_addr)?;
                        }
                        2 => {
                            result.ld_addr("R0", src_addr)?;
                            result.st_addr("R0", dst_addr)?;

                            result.ld_addr("R0", src_addr + 1)?;
                            result.st_addr("R0", dst_addr + 1)?;
                        }
                        _ => unreachable!("invalid size"),
                    }
                }
                _ => {}
            }
        }

        Ok(result.code)
    }
}

#[derive(Debug)]
pub struct CodeGenerator {
    ir: Vec<IRFunction>,
}

impl CodeGenerator {
    #[must_use]
    pub fn new(ir: Vec<IRFunction>) -> Self {
        CodeGenerator { ir }
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn generate(&self) -> CompileResult<String> {
        let mut code = String::new();
        for func in &self.ir {
            let mut ctx = FunctionContext::new(func);
            code.push_str(&ctx.generate()?);
        }
        Ok(code)
    }
}
