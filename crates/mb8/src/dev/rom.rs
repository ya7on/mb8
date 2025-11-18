use mb8_isa::ROM_SIZE;

use super::{utils::empty_memory, Device};

#[derive(Debug)]
pub struct Rom {
    data: Box<[u8; ROM_SIZE]>,
}

impl Default for Rom {
    fn default() -> Self {
        Self {
            data: empty_memory::<ROM_SIZE>(),
        }
    }
}

impl Device for Rom {
    fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }
}
