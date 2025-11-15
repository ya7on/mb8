use context::MemoryContext;
use mb8_isa::BOTS_LIMIT;
use regions::{
    general::GeneralRegion, graphic_buffer::GraphicBufferRegion, rom::ROMRegion, stack::StackRegion,
};

use crate::vm::Role;

pub mod context;
pub mod regions;

#[derive(Debug, Default)]
pub struct Memory {
    current_context: Role,
    host: MemoryContext,
    bots: [MemoryContext; BOTS_LIMIT],
}

impl Memory {
    pub fn current_context(&mut self) -> &mut MemoryContext {
        match self.current_context {
            Role::Judge => &mut self.host,
            Role::Bot(id) => &mut self.bots[id as usize],
        }
    }

    pub fn switch_context(&mut self, role: Role) {
        self.current_context = role;
    }

    pub fn host(&mut self) -> &mut MemoryContext {
        &mut self.host
    }

    pub fn bot(&mut self, id: u8) -> &mut MemoryContext {
        &mut self.bots[id as usize]
    }

    pub fn stack(&mut self) -> StackRegion<'_> {
        self.current_context().stack()
    }

    pub fn graphic_buffer(&mut self) -> GraphicBufferRegion<'_> {
        self.current_context().graphic_buffer()
    }

    pub fn general(&mut self) -> GeneralRegion<'_> {
        self.current_context().general()
    }

    pub fn rom(&mut self) -> ROMRegion<'_> {
        self.current_context().rom()
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
