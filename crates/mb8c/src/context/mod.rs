use symbols::{Symbol, SymbolKind, SymbolTable};
use types::{TypeKind, TypeTable};

pub mod symbols;
pub mod types;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct SymbolId(pub usize);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct TypeId(pub usize);

#[derive(Debug, Default, Clone)]
pub struct CompileContext {
    pub symbol_table: SymbolTable,
    pub type_table: TypeTable,
}

impl CompileContext {
    pub fn allocate_parameter(&mut self, name: &str, ty: TypeKind) -> SymbolId {
        let type_id = self.type_table.entry(ty);
        self.symbol_table.allocate(Symbol {
            name: name.to_owned(),
            kind: SymbolKind::Parameter,
            ty: type_id,
        })
    }

    pub fn allocate_local(&mut self, name: &str, ty: TypeKind) -> SymbolId {
        let type_id = self.type_table.entry(ty);
        self.symbol_table.allocate(Symbol {
            name: name.to_owned(),
            kind: SymbolKind::Local,
            ty: type_id,
        })
    }

    #[must_use]
    pub fn lookup(&self, symbol: SymbolId) -> Option<Symbol> {
        self.symbol_table.lookup(symbol)
    }
}
