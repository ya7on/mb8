use super::Device;

pub mod registers {
    pub const DATA: u16 = 0x00;
}

#[derive(Debug, Default)]
pub struct Rand {
    pub number: u8,
}

impl Rand {
    #[must_use]
    pub fn default(seed: u8) -> Self {
        Self {
            number: if seed == 0 { 1 } else { seed },
        }
    }

    pub fn seed(&mut self, value: u8) {
        self.number = value;
    }

    fn rand_gen(&mut self) -> u8 {
        let mut x = self.number;

        x ^= x << 3;
        x ^= x >> 5;
        x ^= x << 1;

        self.number = if x == 0 { 1 } else { x };

        x
    }
}

impl Device for Rand {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            registers::DATA => self.rand_gen(),
            _ => unimplemented!(),
        }
    }

    fn write(&mut self, _addr: u16, _value: u8) {}
}
