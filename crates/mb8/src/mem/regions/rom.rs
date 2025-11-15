use super::MemoryRegion;

pub struct ROMRegion<'a> {
    start: u16,
    end: u16,
    data: &'a mut [u8],
}

impl<'a> ROMRegion<'a> {
    pub fn new(start: u16, end: u16, data: &'a mut [u8]) -> Self {
        ROMRegion { start, end, data }
    }

    pub fn next_instruction(&self, pc: u16) -> u16 {
        let hi = self.data[pc as usize];
        let lo = self.data[(pc + 1) as usize];

        u16::from_be_bytes([hi, lo])
    }
}

impl MemoryRegion for ROMRegion<'_> {
    fn begin(&self) -> u16 {
        self.start
    }

    fn end(&self) -> u16 {
        self.end
    }

    fn read(&self, addr: u16) -> u8 {
        self.data[(addr - self.start) as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.data[(addr - self.start) as usize] = value;
    }
}
