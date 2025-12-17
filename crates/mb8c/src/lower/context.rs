use std::collections::HashMap;

use crate::{
    hir::{HIRFunctionParam, SymbolId},
    ir::{BasicBlockId, VirtualRegister},
    semantic::types::TypeTable,
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
    pub storage: HashMap<SymbolId, StoredSymbol>,
}

impl LowerContext {
    pub fn new(params: &[HIRFunctionParam]) -> Self {
        Self {
            next_bb: 0,
            next_register: 0,
            types: TypeTable::default(),
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

    pub fn vreg(&mut self, size: u8) -> VirtualRegister {
        let id = self.next_register;
        self.next_register += 1;
        VirtualRegister { id, size }
    }

    pub fn lookup_name(&self, name: &SymbolId) -> Option<&StoredSymbol> {
        self.storage.get(name)
    }
}
