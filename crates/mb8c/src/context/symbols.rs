use super::{SymbolId, TypeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Local,
    Function,
    Parameter,
    Global { address: u16 },
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub ty: TypeId,
}

#[derive(Debug, Default, Clone)]
pub struct SymbolTable {
    pub symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn allocate(&mut self, symbol: Symbol) -> SymbolId {
        let id = self.symbols.len();
        self.symbols.push(symbol);
        SymbolId(id)
    }

    #[must_use]
    pub fn lookup(&self, symbol_id: SymbolId) -> Option<Symbol> {
        self.symbols.get(symbol_id.0).cloned()
    }
}
