use super::{ram::Ram, rom::Rom, Device};

#[derive(Debug, Default)]
pub struct Bus {
    rom: Rom,
    ram: Ram,
}

impl Bus {
    #[must_use]
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0xBFFF => self.ram.read(addr),
            0xC000..=0xDFFF => unimplemented!(),
            0xE000..=0xEFFF => self.rom.read(addr - 0xE000),
            0xF000..=0xFFFF => unimplemented!(),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0xBFFF => self.ram.write(addr, value),
            0xC000..=0xDFFF => unimplemented!(),
            0xE000..=0xEFFF => self.rom.write(addr - 0xE000, value),
            0xF000..=0xFFFF => unimplemented!(),
        }
    }
}
