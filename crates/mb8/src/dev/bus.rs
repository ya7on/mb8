use super::{disk::Disk, gpu::GPU, keyboard::Keyboard, ram::RAM, rand::Rand, rom::ROM, Device};

#[derive(Debug, Default)]
pub struct Bus {
    rom: ROM,
    ram: RAM,
    gpu: GPU,
    keyboard: Keyboard,
    disk: Disk,
    rand: Rand,
}

impl Bus {
    #[must_use]
    pub fn gpu(&mut self) -> &mut GPU {
        &mut self.gpu
    }

    pub fn keyboard(&mut self) -> &mut Keyboard {
        &mut self.keyboard
    }

    pub fn disk(&mut self) -> &mut Disk {
        &mut self.disk
    }

    pub fn rand(&mut self) -> &mut Rand {
        &mut self.rand
    }

    #[must_use]
    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0xBFFF => self.ram.read(addr),
            0xC000..=0xDFFF => unimplemented!(),
            0xE000..=0xEFFF => self.rom.read(addr - 0xE000),
            0xF000..=0xF0FF => self.gpu.read(addr - 0xF000),
            0xF100..=0xF1FF => self.keyboard.read(addr - 0xF100),
            0xF200..=0xF3FF => self.disk.read(addr - 0xF200),
            0xF400 => self.rand.read(addr - 0xF400),
            0xF401..=0xFFFF => unimplemented!(),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0xBFFF => self.ram.write(addr, value),
            0xC000..=0xDFFF => unimplemented!(),
            0xE000..=0xEFFF => self.rom.write(addr - 0xE000, value),
            0xF000..=0xF0FF => self.gpu.write(addr - 0xF000, value),
            0xF100..=0xF1FF => self.keyboard.write(addr - 0xF100, value),
            0xF200..=0xF3FF => self.disk.write(addr - 0xF200, value),
            0xF400 => self.rand.write(addr - 0xF400, value),
            0xF401..=0xFFFF => unimplemented!(),
        }
    }
}
