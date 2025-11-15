use super::MemoryRegion;

#[derive(Debug)]
pub struct GeneralRegion<'a> {
    start: u16,
    end: u16,
    data: &'a mut [u8],
}

impl<'a> GeneralRegion<'a> {
    pub fn new(start: u16, end: u16, data: &'a mut [u8]) -> Self {
        GeneralRegion { start, end, data }
    }
}

impl MemoryRegion for GeneralRegion<'_> {
    fn begin(&self) -> u16 {
        self.start
    }

    fn end(&self) -> u16 {
        self.end
    }

    fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }
}
