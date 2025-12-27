use crate::{
    context::{symbols::SymbolKind, CompileContext, SymbolId},
    error::{CompileError, CompileResult},
    ir::instructions::IRProgram,
    pipeline::CompilerPipe,
};

use super::{Layout, Place};

#[derive(Debug)]
pub struct LayoutPass {
    ctx: CompileContext,
    layout: Layout,
    offset: usize,
}

impl CompilerPipe for LayoutPass {
    type Prev = (IRProgram, CompileContext);
    type Next = (IRProgram, CompileContext, Layout);

    fn execute(prev: &Self::Prev) -> CompileResult<Self::Next, Vec<CompileError>> {
        let (program, ctx) = prev;

        let mut pass = LayoutPass {
            ctx: ctx.clone(),
            layout: Layout::default(),
            offset: 0,
        };

        pass.pass_program(&program).map_err(|err| vec![err])?;

        Ok((program.clone(), ctx.clone(), pass.layout))
    }
}

impl LayoutPass {
    pub fn pass_program(&mut self, ir: &IRProgram) -> CompileResult<()> {
        self.pass_globals(&ir.globals)?;

        for function in &ir.functions {
            self.pass_params(&function.params)?;
            self.pass_locals(&function.locals)?;
        }

        Ok(())
    }

    pub fn pass_globals(&mut self, globals: &[SymbolId]) -> CompileResult<()> {
        for symbol_id in globals {
            let symbol = self.ctx.lookup(*symbol_id).ok_or_else(|| todo!())?;
            let SymbolKind::Global { address } = symbol.kind else {
                unimplemented!()
            };
            self.layout.allocate(
                *symbol_id,
                Place::Global {
                    address: address as u32,
                },
            );
        }

        Ok(())
    }

    pub fn pass_params(&mut self, params: &[SymbolId]) -> CompileResult<()> {
        let mut offset = 2;
        for symbol_id in params {
            let symbol = self.ctx.lookup(*symbol_id).ok_or_else(|| todo!())?;
            let ty = self
                .ctx
                .type_table
                .lookup(symbol.ty)
                .ok_or_else(|| todo!())?;
            let SymbolKind::Parameter = symbol.kind else {
                unimplemented!()
            };
            self.layout
                .allocate(*symbol_id, Place::StackFrame { offset });
            offset += ty.width() as usize;
        }

        Ok(())
    }

    pub fn pass_locals(&mut self, locals: &[SymbolId]) -> CompileResult<()> {
        for symbol_id in locals {
            let symbol = self.ctx.lookup(*symbol_id).ok_or_else(|| todo!())?;
            let ty = self
                .ctx
                .type_table
                .lookup(symbol.ty)
                .ok_or_else(|| todo!())?;
            let SymbolKind::Parameter = symbol.kind else {
                unimplemented!()
            };
            self.layout.allocate(
                *symbol_id,
                Place::StackFrame {
                    offset: self.offset,
                },
            );
            self.offset += ty.width() as usize;
        }

        Ok(())
    }
}
