use super::{utils::empty_memory, Device};

pub mod registers {
    pub const TTY_ROWS: u8 = 16;
    pub const TTY_COLS: u8 = 32;
    pub const TTY_CELLS: u16 = TTY_ROWS as u16 * TTY_COLS as u16;

    pub const GPU_MODE_OFF: u8 = 0x00;
    pub const GPU_MODE_TTY: u8 = 0x01;

    pub const GPU_MODE: u16 = 0x0000;

    /// TTY mode registers
    pub const TTY_MODE: u16 = 0x0001;
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Mode {
    #[default]
    Off,
    Tty,
}

impl From<u8> for Mode {
    fn from(value: u8) -> Self {
        match value {
            registers::GPU_MODE_OFF => Mode::Off,
            registers::GPU_MODE_TTY => Mode::Tty,
            _ => unimplemented!(),
        }
    }
}

impl From<Mode> for u8 {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Off => registers::GPU_MODE_OFF,
            Mode::Tty => registers::GPU_MODE_TTY,
        }
    }
}

#[derive(Debug)]
pub struct Tty {
    buffer: Box<[u8; registers::TTY_CELLS as usize]>,
    cursor: (u8, u8),
}

impl Default for Tty {
    fn default() -> Self {
        Self {
            buffer: empty_memory(),
            cursor: (0, 0),
        }
    }
}

#[derive(Debug, Default)]
pub struct GPU {
    pub mode: Mode,
    pub tty: Tty,
}

impl Device for GPU {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            registers::GPU_MODE => self.mode.into(),
            _ => unimplemented!(),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            registers::GPU_MODE => self.mode = value.into(),
            registers::TTY_MODE if self.mode == Mode::Tty => {
                self.tty.buffer[self.tty.cursor.0 as usize * registers::TTY_COLS as usize
                    + self.tty.cursor.1 as usize] = value;
                self.tty.cursor.1 += 1;
                if self.tty.cursor.1 >= registers::TTY_COLS {
                    self.tty.cursor.1 = 0;
                    self.tty.cursor.0 += 1;
                    if self.tty.cursor.0 >= registers::TTY_ROWS {
                        self.tty.cursor.0 = 0;
                    }
                }
            }
            _ => unimplemented!(),
        }
    }
}
