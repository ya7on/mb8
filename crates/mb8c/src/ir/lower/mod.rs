use std::collections::HashMap;

use crate::{
    error::{CompileError, CompileResult},
    hir::instructions::HIRProgram,
    ir::instructions::IRProgram,
    pipeline::CompilerPipe,
};

use super::context::StoredSymbol;

pub mod expr;
pub mod function;
pub mod helpers;
pub mod stmt;

#[derive(Debug)]
pub struct Lower {
    hir: HIRProgram,
}

impl CompilerPipe for Lower {
    type Prev = HIRProgram;
    type Next = IRProgram;

    fn execute(prev: &Self::Prev) -> CompileResult<Self::Next, Vec<CompileError>> {
        let mut lower = Self {
            hir: prev.to_owned(),
        };

        let mut functions = Vec::with_capacity(lower.hir.functions.len());

        let mut initial_storage = HashMap::new();
        for global in &prev.globals {
            initial_storage.insert(global.symbol, StoredSymbol::Global(global.at));
        }

        for function in &prev.functions {
            functions.push(
                lower
                    .lower_function(function, initial_storage.clone())
                    .map_err(|err| vec![err])?,
            );
        }

        Ok(IRProgram { functions })
    }
}
