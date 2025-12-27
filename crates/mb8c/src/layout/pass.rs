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
            offset: 10,
        };

        pass.pass_program(program).map_err(|err| vec![err])?;

        Ok((program.clone(), ctx.clone(), pass.layout))
    }
}

impl LayoutPass {
    /// Walk the IR program and allocate memory/layout slots.
    ///
    /// # Errors
    /// Returns an error when any subordinate pass encounters missing context data.
    pub fn pass_program(&mut self, ir: &IRProgram) -> CompileResult<()> {
        self.pass_globals(&ir.globals)?;

        for function in &ir.functions {
            self.pass_params(&function.params)?;
            self.pass_locals(&function.locals)?;
        }

        Ok(())
    }

    /// Allocate storage for global symbols.
    ///
    /// # Errors
    /// Returns an error when a global symbol cannot be resolved.
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

    /// Allocate storage for function parameters on the stack.
    ///
    /// # Errors
    /// Returns an error when parameter symbols or their types cannot be resolved.
    pub fn pass_params(&mut self, params: &[SymbolId]) -> CompileResult<()> {
        let mut offset = 0;
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

    /// Allocate storage for local variables on the stack.
    ///
    /// # Errors
    /// Returns an error when local symbols or their types cannot be resolved.
    pub fn pass_locals(&mut self, locals: &[SymbolId]) -> CompileResult<()> {
        for symbol_id in locals {
            let symbol = self.ctx.lookup(*symbol_id).ok_or_else(|| todo!())?;
            let ty = self
                .ctx
                .type_table
                .lookup(symbol.ty)
                .ok_or_else(|| todo!())?;
            let SymbolKind::Local = symbol.kind else {
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
