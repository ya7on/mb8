use crate::{ir::Mem, lower::context::StoredSymbol};

pub fn get_memory_from_stored_symbol(stored_symbol: &StoredSymbol) -> Mem {
    match stored_symbol {
        StoredSymbol::Offset(offset) => Mem::Local { offset: *offset },
        StoredSymbol::Global(address) => Mem::Global { address: *address },
    }
}
