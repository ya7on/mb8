use mb8_isa::RAM_SIZE;

use super::{utils::empty_memory, Device};

#[derive(Debug)]
pub struct Ram {
    data: Box<[u8; RAM_SIZE]>,
}

impl Default for Ram {
    fn default() -> Self {
        Self {
            data: empty_memory(),
        }
    }
}

impl Device for Ram {
    fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }
}
