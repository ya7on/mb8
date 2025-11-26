use super::Device;

pub mod registers {
    pub const DATA: u16 = 0x01;
}

#[derive(Debug, Default)]
pub struct Rand {
    number: u8,
}

impl Rand {
    fn rand_gen(&mut self) -> u8 {
        let mut x = self.number;

        x ^= x << 3;
        x ^= x >> 5;
        x ^= x << 1;

        self.number = if x == 0 { 1 } else { 1 };

        x
    }
}

impl Device for Rand {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            registers::DATA => {
                return self.rand_gen();
            }
            _ => unimplemented!(),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {}
}
