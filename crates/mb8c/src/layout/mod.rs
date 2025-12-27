use std::collections::HashMap;

use crate::context::SymbolId;

pub mod globals;
pub mod pass;

#[derive(Debug, Clone)]
pub enum Place {
    StaticFrame { offset: u16 },
    Global { address: u16 },
}

#[derive(Debug, Clone, Default)]
pub struct Layout {
    table: HashMap<SymbolId, Place>,
}

impl Layout {
    pub fn allocate(&mut self, symbol_id: SymbolId, place: Place) {
        self.table.insert(symbol_id, place);
    }

    #[must_use]
    pub fn lookup(&self, symbol_id: SymbolId) -> Option<&Place> {
        self.table.get(&symbol_id)
    }
}
