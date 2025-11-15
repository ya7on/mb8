use context::MemoryContext;
use mb8_isa::{GRAPHIC_BUFFER_SIZE, MEMORY_BANK_SIZE, STACK_SIZE};
use regions::{
    general::GeneralRegion, graphic_buffer::GraphicBufferRegion, rom::ROMRegion, stack::StackRegion,
};

pub mod context;
pub mod regions;

#[derive(Debug, Default)]
pub struct Memory {
    host: MemoryContext,
}

impl Memory {
    pub fn stack(&mut self) -> StackRegion<'_> {
        let ram = self.host.ram();
        let stack_begin = 0;
        let stack_end = stack_begin + STACK_SIZE - 1;
        StackRegion::new(
            stack_begin,
            stack_end,
            &mut ram[stack_begin as usize..=(stack_end as usize)],
        )
    }

    pub fn graphic_buffer(&mut self) -> GraphicBufferRegion<'_> {
        let ram = self.host.ram();
        let begin = MEMORY_BANK_SIZE - GRAPHIC_BUFFER_SIZE;
        let end = MEMORY_BANK_SIZE - 1;
        GraphicBufferRegion::new(begin as u16, end as u16, &mut ram[begin..=end])
    }

    pub fn general(&mut self) -> GeneralRegion<'_> {
        let ram = self.host.ram();
        let begin = STACK_SIZE;
        let end = MEMORY_BANK_SIZE - GRAPHIC_BUFFER_SIZE - 1;
        GeneralRegion::new(begin, end as u16, &mut ram[begin as usize..=end])
    }

    pub fn rom(&mut self) -> ROMRegion<'_> {
        let ram = self.host.rom();
        let begin = 0;
        let end = MEMORY_BANK_SIZE - 1;
        ROMRegion::new(begin, end as u16, &mut ram[begin as usize..=end])
    }
}

#[cfg(test)]
mod tests {
    use crate::mem::regions::MemoryRegion;

    use super::*;

    #[test]
    fn test_stack_region() {
        let mut mem = Memory::default();
        let stack = mem.stack();
        assert_eq!(stack.begin(), 0);
        assert_eq!(stack.end(), 255);
        assert_eq!(stack.size(), 256);
    }

    #[test]
    fn test_graphic_buffer_region() {
        let mut mem = Memory::default();
        let stack = mem.graphic_buffer();
        assert_eq!(stack.begin(), 3840);
        assert_eq!(stack.end(), 4095);
        assert_eq!(stack.size(), 256);
    }

    #[test]
    fn test_general_region() {
        let mut mem = Memory::default();
        let stack = mem.general();
        assert_eq!(stack.begin(), 256);
        assert_eq!(stack.end(), 3839);
        assert_eq!(stack.size(), 3584);
    }
}
