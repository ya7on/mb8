use std::collections::VecDeque;

use super::Device;

pub mod registers {
    pub const STATUS: u16 = 0x00;
    pub const DATA: u16 = 0x01;
}

#[derive(Debug, Default)]
pub struct Keyboard {
    queue: VecDeque<u8>,
}

impl Keyboard {
    pub fn key_pressed(&mut self, key: u8) {
        self.queue.push_back(key);
    }
}

impl Device for Keyboard {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            registers::STATUS => !self.queue.is_empty() as u8,
            registers::DATA => self.queue.pop_front().unwrap_or_default(),
            _ => unimplemented!(),
        }
    }

    fn write(&mut self, _addr: u16, _value: u8) {
        unimplemented!()
    }
}
