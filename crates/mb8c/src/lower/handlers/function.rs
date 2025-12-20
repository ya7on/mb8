use std::collections::HashMap;

use crate::{
    error::{CompileError, CompileResult},
    hir::HIRFunction,
    ir::{BasicBlockTerminator, IRFunction, IRInstruction, Mem},
    lower::context::{LowerContext, StoredSymbol},
};

use super::Lower;

impl Lower {
    /// # Errors
    /// Returns a `CompileError` if there was an lowering error
    pub fn lower_function(&mut self, function: &HIRFunction) -> CompileResult<IRFunction> {
        let mut storage = HashMap::with_capacity(function.locals.len() + function.params.len());
        let mut offset = 1; // 0 is reserved for accumulator
        for param in &function.params {
            storage.insert(param.symbol, StoredSymbol { offset });
            let type_kind =
                self.hir
                    .types
                    .lookup(param.type_id)
                    .ok_or(CompileError::InternalError {
                        message: "Unknown type".to_string(),
                    })?;
            offset += type_kind.size();
        }
        for local in &function.locals {
            storage.insert(local.symbol, StoredSymbol { offset });
            let type_kind =
                self.hir
                    .types
                    .lookup(local.type_id)
                    .ok_or(CompileError::InternalError {
                        message: "Unknown type".to_string(),
                    })?;
            offset += type_kind.size();
        }

        let mut ctx = LowerContext::new(storage);

        let mut basic_blocks = Vec::new();
        let mut current = ctx.bb();
        // Initialize function arguments
        for (index, arg) in function.params.iter().enumerate() {
            let type_kind = self.hir.types.lookup(arg.type_id).ok_or_else(|| todo!())?;
            let stored_symbol = ctx.lookup_name(&arg.symbol).ok_or_else(|| todo!())?;
            current.emit(IRInstruction::LoadlArg {
                ty: type_kind.to_owned(),
                index,
                mem: Mem::Local {
                    offset: stored_symbol.offset,
                },
            });
        }
        let mut current = Some(current);
        for stmt in &function.body {
            if let Some(builder) = current {
                let (builder, bbs) = self.lower_stmt(builder, &mut ctx, stmt)?;
                basic_blocks.extend(bbs);
                current = builder;
            }
            if current.is_none() {
                break;
            }
        }
        if let Some(current) = current {
            basic_blocks.push(current.build(BasicBlockTerminator::Ret { value: None }));
        }

        Ok(IRFunction {
            name: function.name.clone(),
            basic_blocks,
            size: offset,
        })
    }
}
