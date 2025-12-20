use std::collections::HashMap;

use crate::{
    hir::SymbolId,
    ir::{BasicBlockId, VirtualRegister},
};

use super::bb::BasicBlockBuilder;

#[derive(Debug)]
pub struct StoredSymbol {
    pub offset: usize,
}

#[derive(Debug)]
pub struct LowerContext {
    pub next_bb: usize,
    pub next_register: usize,
    pub storage: HashMap<SymbolId, StoredSymbol>,
}

impl LowerContext {
    #[must_use]
    pub fn new(storage: HashMap<SymbolId, StoredSymbol>) -> Self {
        Self {
            next_bb: 0,
            next_register: 0,
            storage,
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
