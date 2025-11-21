use super::{utils::empty_memory, Device};

pub mod registers {
    pub const DISK_BLOCK: u16 = 0x0000;
    pub const DISK_CMD: u16 = 0x0001;
    pub const DISK_BUFFER_START: u16 = 0x0002;
    pub const DISK_BUFFER_END: u16 = 0x0002 + 256;

    pub const DISK_CMD_NOP: u8 = 0x00;
    pub const DISK_CMD_READ: u8 = 0x01;
    pub const DISK_CMD_WRITE: u8 = 0x02;
}

#[derive(Debug)]
pub struct Disk {
    img: Box<[u8; 65536]>,
    buffer: Box<[u8; 256]>,
    block: u8,
}

impl Default for Disk {
    fn default() -> Self {
        Disk {
            img: empty_memory(),
            buffer: empty_memory(),
            block: Default::default(),
        }
    }
}

impl Disk {
    pub fn set(&mut self, img: Box<[u8; 65536]>) {
        self.img = img;
    }

    #[must_use]
    pub fn dump(&self) -> &[u8] {
        self.img.as_slice()
    }
}

impl Device for Disk {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            registers::DISK_BLOCK => self.block,
            registers::DISK_BUFFER_START..=registers::DISK_BUFFER_END => {
                self.buffer[(addr - registers::DISK_BUFFER_START) as usize]
            }
            _ => unimplemented!(),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            registers::DISK_BLOCK => self.block = value,
            registers::DISK_CMD => match value {
                registers::DISK_CMD_NOP => {}
                registers::DISK_CMD_READ => {
                    let offset = self.block as usize * 256;
                    self.buffer.copy_from_slice(&self.img[offset..offset + 256]);
                }
                registers::DISK_CMD_WRITE => {
                    let offset = self.block as usize * 256;
                    self.img[offset..offset + 256].copy_from_slice(self.buffer.as_slice());
                }
                _ => unimplemented!(),
            },
            registers::DISK_BUFFER_START..=registers::DISK_BUFFER_END => {
                self.buffer[(addr - registers::DISK_BUFFER_START) as usize] = value;
            }
            _ => unimplemented!(),
        }
    }
}
