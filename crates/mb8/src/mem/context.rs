use mb8_isa::MEMORY_BANK_SIZE;

fn empty_bank() -> Box<[u8; MEMORY_BANK_SIZE]> {
    #[allow(clippy::unwrap_used)]
    vec![0; MEMORY_BANK_SIZE].try_into().unwrap()
}

#[derive(Debug)]
pub struct MemoryContext {
    rom: Box<[u8; MEMORY_BANK_SIZE]>,
    ram: Box<[u8; MEMORY_BANK_SIZE]>,
}

impl Default for MemoryContext {
    fn default() -> Self {
        Self {
            rom: empty_bank(),
            ram: empty_bank(),
        }
    }
}

impl MemoryContext {
    pub fn rom(&mut self) -> &mut [u8; MEMORY_BANK_SIZE] {
        &mut self.rom
    }

    pub fn ram(&mut self) -> &mut [u8; MEMORY_BANK_SIZE] {
        &mut self.ram
    }
}
