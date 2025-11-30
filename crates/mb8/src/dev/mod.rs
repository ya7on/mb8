pub mod bus;
pub mod disk;
pub mod gpu;
pub mod keyboard;
pub mod ram;
pub mod rand;
pub mod rom;
pub mod utils;

pub trait Device {
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}
