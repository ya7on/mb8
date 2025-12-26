use crate::{ir::context::StoredSymbol, ir::instructions::Mem};

#[must_use]
pub fn get_memory_from_stored_symbol(stored_symbol: &StoredSymbol) -> Mem {
    match stored_symbol {
        StoredSymbol::Offset(offset) => Mem::Local { offset: *offset },
        StoredSymbol::Global(address) => Mem::Global { address: *address },
    }
}
