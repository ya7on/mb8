use mb8_isa::RAM_SIZE;

use super::{utils::empty_memory, Device};

#[derive(Debug)]
pub struct RAM {
    data: Box<[u8; RAM_SIZE]>,
}

impl Default for RAM {
    fn default() -> Self {
        Self {
            data: empty_memory(),
        }
    }
}

impl Device for RAM {
    fn read(&mut self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }
}
