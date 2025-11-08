use mb8_isa::{MEMORY_SIZE, STACK_SIZE};

#[derive(Debug)]
pub struct Memory {
    data: Box<[u8; MEMORY_SIZE]>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            #[allow(clippy::unwrap_used)]
            data: vec![0; MEMORY_SIZE].into_boxed_slice().try_into().unwrap(),
        }
    }
}

impl Memory {
    /// Directly read a byte from memory.
    pub fn read_u8(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    /// Directly write a byte to memory.
    pub fn write_u8(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }

    /// Directly read a word from memory.
    #[allow(dead_code)]
    pub fn read_u16(&self, addr: u16) -> u16 {
        let hi = self.read_u8(addr);
        let lo = self.read_u8(addr + 1);
        ((hi as u16) << 8) | (lo as u16)
    }

    /// Directly write a word to memory.
    pub fn write_u16(&mut self, addr: u16, value: u16) {
        for (i, byte) in value.to_be_bytes().iter().enumerate() {
            self.write_u8(addr + i as u16, *byte);
        }
    }

    /// Push a byte onto the stack.
    #[allow(dead_code)]
    pub fn push_u8(&mut self, mut stack_pointer: u16, value: u8) -> u16 {
        stack_pointer += 1;
        self.write_u8(STACK_SIZE - stack_pointer, value);
        stack_pointer
    }

    /// Pop a byte from the stack.
    #[allow(dead_code)]
    pub fn pop_u8(&mut self, stack_pointer: u16) -> (u8, u16) {
        let value = self.read_u8(STACK_SIZE - stack_pointer);
        self.write_u8(STACK_SIZE - stack_pointer, 0);
        (value, stack_pointer - 1)
    }

    /// Push a word onto the stack.
    pub fn push_u16(&mut self, mut stack_pointer: u16, value: u16) -> u16 {
        stack_pointer += 2;
        self.write_u16(STACK_SIZE - stack_pointer, value);
        stack_pointer
    }

    /// Pop a word from the stack.
    #[allow(dead_code)]
    pub fn pop_u16(&mut self, stack_pointer: u16) -> (u16, u16) {
        let value = self.read_u16(STACK_SIZE - stack_pointer);
        self.write_u16(STACK_SIZE - stack_pointer, 0);
        (value, stack_pointer - 2)
    }

    /// Get the stack slice.
    #[allow(dead_code)]
    pub fn stack(&self) -> &[u8] {
        &self.data[..STACK_SIZE as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write_u8() {
        let mut mem = Memory::default();
        mem.write_u8(0x100, 0x55);
        assert_eq!(mem.read_u8(0x100), 0x55);
    }

    #[test]
    fn test_read_write_u16() {
        let mut mem = Memory::default();
        mem.write_u16(0x100, 0x1234);
        assert_eq!(mem.read_u16(0x100), 0x1234);
    }

    #[test]
    fn test_read_write_u8_u16() {
        let mut mem = Memory::default();
        mem.write_u8(0x100, 0x55);
        mem.write_u8(0x101, 0x55);
        assert_eq!(mem.read_u16(0x100), 0x5555);

        mem.write_u16(0x102, 0x5678);
        assert_eq!(mem.read_u8(0x102), 0x56);
        assert_eq!(mem.read_u8(0x103), 0x78);
    }

    #[test]
    fn test_push_pop_u8() {
        let mut mem = Memory::default();
        let mut stack_pointer = 0;

        stack_pointer = mem.push_u8(stack_pointer, 0x55);

        assert_eq!(mem.pop_u8(stack_pointer), (0x55, stack_pointer - 1));
    }

    #[test]
    fn test_push_pop_u16() {
        let mut mem = Memory::default();
        let mut stack_pointer = 0;

        stack_pointer = mem.push_u16(stack_pointer, 0x1234);

        assert_eq!(mem.pop_u16(stack_pointer), (0x1234, stack_pointer - 2));
    }

    #[test]
    fn test_push_pop_u8_u16() {
        let mut mem = Memory::default();
        let mut stack_pointer = 0;

        stack_pointer = mem.push_u8(stack_pointer, 0x12);
        stack_pointer = mem.push_u8(stack_pointer, 0x34);
        stack_pointer = mem.push_u16(stack_pointer, 0x5678);

        let (value, stack_pointer) = mem.pop_u8(stack_pointer);
        assert_eq!(value, 0x56);
        let (value, stack_pointer) = mem.pop_u8(stack_pointer);
        assert_eq!(value, 0x78);

        let (value, _) = mem.pop_u16(stack_pointer);
        assert_eq!(value, 0x3412);
    }

    #[test]
    fn test_stack() {
        let mut mem = Memory::default();
        let mut stack_pointer = 0;
        stack_pointer = mem.push_u8(stack_pointer, 0x12);
        mem.push_u8(stack_pointer, 0x34);

        assert_eq!(&mem.stack()[254..256], &[0x34, 0x12]);
    }
}
