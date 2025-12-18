use std::collections::HashMap;

use crate::{
    hir::{HIRFunctionParam, SymbolId},
    ir::{BasicBlockId, VirtualRegister},
    semantic::{context::SemanticContext, symbols::SymbolTable, types::TypeTable},
};

use super::bb::BasicBlockBuilder;

#[derive(Debug)]
pub struct StoredSymbol {
    pub offset: usize,
    pub size: usize,
}

#[derive(Debug)]
pub struct LowerContext {
    pub next_bb: usize,
    pub next_register: usize,
    pub types: TypeTable,
    pub symbols: SymbolTable,
    pub storage: HashMap<SymbolId, StoredSymbol>,
}

impl LowerContext {
    #[must_use]
    pub fn new(params: &[HIRFunctionParam], hir_ctx: &SemanticContext) -> Self {
        Self {
            next_bb: 0,
            next_register: 0,
            types: hir_ctx.types.clone(),
            symbols: hir_ctx.symbols.clone(),
            storage: params
                .iter()
                .map(|param| {
                    (
                        param.symbol,
                        StoredSymbol {
                            offset: param.offset,
                            size: param.size,
                        },
                    )
                })
                .collect(),
        }
    }

    pub fn bb(&mut self) -> BasicBlockBuilder {
        let id = self.next_bb;
        self.next_bb += 1;
        BasicBlockBuilder::new(BasicBlockId(id))
    }

    pub fn vreg(&mut self) -> VirtualRegister {
        let id = self.next_register;
        self.next_register += 1;
        VirtualRegister(id)
    }

    #[must_use]
    pub fn lookup_name(&self, name: &SymbolId) -> Option<&StoredSymbol> {
        self.storage.get(name)
    }
}
