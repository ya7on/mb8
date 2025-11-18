pub mod bus;
pub mod ram;
pub mod rom;
pub mod utils;

pub trait Device {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}
