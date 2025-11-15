pub mod general;
pub mod graphic_buffer;
pub mod rom;
pub mod stack;

pub trait MemoryRegion {
    /// Returns the start address of the memory region.
    #[allow(dead_code)]
    fn begin(&self) -> u16;
    /// Returns the end address of the memory region.
    #[allow(dead_code)]
    fn end(&self) -> u16;
    /// Reads a byte from the memory region at the given address.
    fn read(&self, addr: u16) -> u8;
    /// Writes a byte to the memory region at the given address.
    fn write(&mut self, addr: u16, value: u8);

    /// Returns the size of the memory region.
    #[allow(dead_code)]
    fn size(&self) -> u16 {
        self.end() - self.begin() + 1
    }
}
