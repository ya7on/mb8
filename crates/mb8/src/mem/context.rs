use mb8_isa::{GRAPHIC_BUFFER_SIZE, MEMORY_BANK_SIZE, STACK_SIZE};

use super::regions::{
    general::GeneralRegion, graphic_buffer::GraphicBufferRegion, rom::ROMRegion, stack::StackRegion,
};

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
    pub fn stack(&mut self) -> StackRegion<'_> {
        let stack_begin = 0;
        let stack_end = stack_begin + STACK_SIZE - 1;
        StackRegion::new(
            stack_begin,
            stack_end,
            &mut self.ram[stack_begin as usize..=(stack_end as usize)],
        )
    }

    pub fn graphic_buffer(&mut self) -> GraphicBufferRegion<'_> {
        let begin = MEMORY_BANK_SIZE - GRAPHIC_BUFFER_SIZE;
        let end = MEMORY_BANK_SIZE - 1;
        GraphicBufferRegion::new(begin as u16, end as u16, &mut self.ram[begin..=end])
    }

    pub fn general(&mut self) -> GeneralRegion<'_> {
        let begin = STACK_SIZE;
        let end = MEMORY_BANK_SIZE - GRAPHIC_BUFFER_SIZE - 1;
        GeneralRegion::new(begin, end as u16, &mut self.ram[begin as usize..=end])
    }

    pub fn rom(&mut self) -> ROMRegion<'_> {
        let begin = 0;
        let end = MEMORY_BANK_SIZE - 1;
        ROMRegion::new(begin, end as u16, &mut self.rom[begin as usize..=end])
    }
}
