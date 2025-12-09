use crate::dev::gpu::registers::VRAM_TTY_END;

use super::{utils::empty_memory, Device};

pub mod registers {
    pub const TTY_ROWS: u8 = 25;
    pub const TTY_COLS: u8 = 40;
    pub const TTY_CELLS: usize = TTY_ROWS as usize * TTY_COLS as usize;

    pub const GPU_MODE_OFF: u8 = 0x00;
    pub const GPU_MODE_TTY: u8 = 0x01;

    pub const GPU_REG_MODE: u16 = 0x0000;
    /// TTY mode registers
    pub const GPU_REG_TTY: u16 = 0x0001;

    pub const VRAM_CURSOR_X: usize = 0x0000;
    pub const VRAM_CURSOR_Y: usize = 0x0001;
    pub const VRAM_TTY_START: usize = 0x0002;
    pub const VRAM_TTY_END: usize = VRAM_TTY_START + TTY_CELLS;
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum Mode {
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
pub struct GPU {
    mode: Mode,
    vram: Box<[u8; registers::TTY_CELLS + 2]>,
    redraw: bool,
}

impl Default for GPU {
    fn default() -> Self {
        Self {
            mode: Mode::Off,
            vram: empty_memory(),
            redraw: false,
        }
    }
}

impl GPU {
    #[must_use]
    pub fn tty_buffer(&self) -> &[u8] {
        &self.vram[registers::VRAM_TTY_START..registers::VRAM_TTY_END]
    }

    pub fn redraw(&mut self) -> bool {
        if self.redraw {
            self.redraw = false;
            true
        } else {
            false
        }
    }
}

impl Device for GPU {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            registers::GPU_REG_MODE => self.mode.into(),
            _ => unimplemented!(),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            registers::GPU_REG_MODE => self.mode = value.into(),
            registers::GPU_REG_TTY if self.mode == Mode::Tty => {
                self.redraw = true;
                let (mut cursor_x, mut cursor_y) = (
                    self.vram[registers::VRAM_CURSOR_X],
                    self.vram[registers::VRAM_CURSOR_Y],
                );

                let tty_buf = &mut self.vram[registers::VRAM_TTY_START..VRAM_TTY_END];
                let cols = registers::TTY_COLS as usize;

                match value {
                    b'\n' => {
                        cursor_x = 0;
                        cursor_y += 1;
                    }

                    b'\x08' => {
                        if cursor_x > 0 {
                            cursor_x -= 1;
                        } else if cursor_y > 0 {
                            cursor_y -= 1;
                            cursor_x = registers::TTY_COLS - 1;
                        } else {
                            cursor_x = 0;
                            cursor_y = 0;
                        }

                        let index = cursor_y as usize * cols + cursor_x as usize;
                        tty_buf[index] = b' ';
                    }

                    _ => {
                        let index = cursor_y as usize * cols + cursor_x as usize;
                        tty_buf[index] = value;

                        cursor_x += 1;
                        if cursor_x >= registers::TTY_COLS {
                            cursor_x = 0;
                            cursor_y += 1;
                        }
                    }
                }

                if cursor_y >= registers::TTY_ROWS {
                    for y in 1..registers::TTY_ROWS {
                        for x in 0..registers::TTY_COLS {
                            let src_index = (y as usize * cols) + x as usize;
                            let dest_index = ((y - 1) as usize * cols) + x as usize;
                            tty_buf[dest_index] = tty_buf[src_index];
                        }
                    }

                    let last_line_start = (registers::TTY_ROWS - 1) as usize * cols;
                    for x in 0..registers::TTY_COLS {
                        tty_buf[last_line_start + x as usize] = b' ';
                    }

                    cursor_y = registers::TTY_ROWS - 1;
                }

                self.vram[registers::VRAM_CURSOR_X] = cursor_x;
                self.vram[registers::VRAM_CURSOR_Y] = cursor_y;
            }
            _ => unimplemented!(),
        }
    }
}
