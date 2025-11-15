use super::MemoryRegion;

#[derive(Debug, PartialEq)]
pub enum StackError {
    Overflow,
    Underflow,
}

pub struct StackRegion<'a> {
    start: u16,
    end: u16,
    data: &'a mut [u8],
}

impl<'a> StackRegion<'a> {
    pub fn new(start: u16, end: u16, data: &'a mut [u8]) -> Self {
        StackRegion { start, end, data }
    }

    pub fn push_u8(&mut self, sp: u16, value: u8) -> Result<u16, StackError> {
        if sp >= self.end {
            return Err(StackError::Overflow);
        }
        self.data[sp as usize] = value;
        Ok(sp + 1)
    }

    pub fn pop_u8(&mut self, sp: u16) -> Result<(u8, u16), StackError> {
        if sp <= self.start {
            return Err(StackError::Underflow);
        }
        let value = self.data[(sp - 1) as usize];
        Ok((value, sp - 1))
    }

    pub fn push_u16(&mut self, sp: u16, value: u16) -> Result<u16, StackError> {
        if sp >= self.end - 2 {
            return Err(StackError::Overflow);
        }
        for (i, byte) in value.to_be_bytes().iter().enumerate() {
            self.data[sp as usize + i] = *byte;
        }
        Ok(sp + 2)
    }

    pub fn pop_u16(&mut self, sp: u16) -> Result<(u16, u16), StackError> {
        if sp < self.start + 2 {
            return Err(StackError::Underflow);
        }
        let value =
            u16::from_be_bytes([self.data[(sp - 2) as usize], self.data[(sp - 1) as usize]]);
        Ok((value, sp - 2))
    }
}

impl MemoryRegion for StackRegion<'_> {
    fn begin(&self) -> u16 {
        self.start
    }

    fn end(&self) -> u16 {
        self.end
    }

    fn read(&self, addr: u16) -> u8 {
        self.data[(addr - self.start) as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.data[(addr - self.start) as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_u8() {
        let mut data = [0; 1024];
        let mut stack = StackRegion::new(0, 100, &mut data);

        let sp = stack.push_u8(0xA, 0x12).unwrap();
        assert_eq!(sp, 0xB);
        assert_eq!(data[0xA], 0x12);
    }

    #[test]
    fn test_pop_u8() {
        let mut data = [0; 1024];
        data[0xA] = 0x12;
        let mut stack = StackRegion::new(0, 100, &mut data);

        let (value, sp) = stack.pop_u8(0xB).unwrap();
        assert_eq!(sp, 0xA);
        assert_eq!(value, 0x12);
    }

    #[test]
    fn test_push_u16() {
        let mut data = [0; 1024];
        let mut stack = StackRegion::new(0, 100, &mut data);

        let sp = stack.push_u16(0xA, 0x1234).unwrap();
        assert_eq!(sp, 0xC);
        assert_eq!(data[0xA], 0x12);
        assert_eq!(data[0xB], 0x34);
    }

    #[test]
    fn test_pop_u16() {
        let mut data = [0; 1024];
        data[0xA] = 0x12;
        data[0xB] = 0x34;
        let mut stack = StackRegion::new(0, 100, &mut data);

        let (value, sp) = stack.pop_u16(0xC).unwrap();
        assert_eq!(sp, 0xA);
        assert_eq!(value, 0x1234);
    }
}
