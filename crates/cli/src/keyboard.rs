use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use mb8::vm;
use minifb::{Key, KeyRepeat, Window};

#[derive(Debug)]
pub struct Keyboard {
    pub left_shift: bool,
    pub right_shift: bool,
    key_last_pressed: HashMap<Key, Instant>,
}

impl Keyboard {
    #[must_use]
    pub fn new(l_shift: bool, r_shift: bool) -> Self {
        Self {
            left_shift: l_shift,
            right_shift: r_shift,
            key_last_pressed: HashMap::new(),
        }
    }

    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn map_key_to_char(key: Key, shift: bool) -> Option<u8> {
        let ch = match key {
            Key::Key0 => b'0',
            Key::Key1 => b'1',
            Key::Key2 => b'2',
            Key::Key3 => b'3',
            Key::Key4 => b'4',
            Key::Key5 => b'5',
            Key::Key6 => b'6',
            Key::Key7 => b'7',
            Key::Key8 => b'8',
            Key::Key9 => b'9',

            Key::A => {
                if shift {
                    b'A'
                } else {
                    b'a'
                }
            }
            Key::B => {
                if shift {
                    b'B'
                } else {
                    b'b'
                }
            }
            Key::C => {
                if shift {
                    b'C'
                } else {
                    b'c'
                }
            }
            Key::D => {
                if shift {
                    b'D'
                } else {
                    b'd'
                }
            }
            Key::E => {
                if shift {
                    b'E'
                } else {
                    b'e'
                }
            }
            Key::F => {
                if shift {
                    b'F'
                } else {
                    b'f'
                }
            }
            Key::G => {
                if shift {
                    b'G'
                } else {
                    b'g'
                }
            }
            Key::H => {
                if shift {
                    b'H'
                } else {
                    b'h'
                }
            }
            Key::I => {
                if shift {
                    b'I'
                } else {
                    b'i'
                }
            }
            Key::J => {
                if shift {
                    b'J'
                } else {
                    b'j'
                }
            }
            Key::K => {
                if shift {
                    b'K'
                } else {
                    b'k'
                }
            }
            Key::L => {
                if shift {
                    b'L'
                } else {
                    b'l'
                }
            }
            Key::M => {
                if shift {
                    b'M'
                } else {
                    b'm'
                }
            }
            Key::N => {
                if shift {
                    b'N'
                } else {
                    b'n'
                }
            }
            Key::O => {
                if shift {
                    b'O'
                } else {
                    b'o'
                }
            }
            Key::P => {
                if shift {
                    b'P'
                } else {
                    b'p'
                }
            }
            Key::Q => {
                if shift {
                    b'Q'
                } else {
                    b'q'
                }
            }
            Key::R => {
                if shift {
                    b'R'
                } else {
                    b'r'
                }
            }
            Key::S => {
                if shift {
                    b'S'
                } else {
                    b's'
                }
            }
            Key::T => {
                if shift {
                    b'T'
                } else {
                    b't'
                }
            }
            Key::U => {
                if shift {
                    b'U'
                } else {
                    b'u'
                }
            }
            Key::V => {
                if shift {
                    b'V'
                } else {
                    b'v'
                }
            }
            Key::W => {
                if shift {
                    b'W'
                } else {
                    b'w'
                }
            }
            Key::X => {
                if shift {
                    b'X'
                } else {
                    b'x'
                }
            }
            Key::Y => {
                if shift {
                    b'Y'
                } else {
                    b'y'
                }
            }
            Key::Z => {
                if shift {
                    b'Z'
                } else {
                    b'z'
                }
            }

            Key::Space => b' ',
            Key::Enter => b'\n',
            Key::Backspace => 0x08,
            Key::Tab => 0x09,
            Key::Escape => 0x1B,
            Key::Comma => b',',
            Key::Period => b'.',
            Key::Slash => b'/',

            _ => return None,
        };

        Some(ch)
    }

    pub fn key_pressed(&mut self, window: &Window, vm: &mut vm::VirtualMachine) {
        for key in window.get_keys_pressed(KeyRepeat::No) {
            let current_time = Instant::now();

            if let Some(last_time) = self.key_last_pressed.get(&key) {
                if current_time.duration_since(*last_time) < Duration::from_millis(100) {
                    continue;
                }
            }
            if key == Key::LeftShift {
                self.left_shift = true;
                continue;
            }
            if key == Key::RightShift {
                self.right_shift = true;
                continue;
            }

            if let Some(mapped_char) =
                Keyboard::map_key_to_char(key, self.left_shift || self.right_shift)
            {
                vm.devices.keyboard().key_pressed(mapped_char);
            }

            self.key_last_pressed.insert(key, current_time);
        }
    }

    pub fn key_released(&mut self, window: &Window) {
        for key in window.get_keys_released() {
            if key == Key::LeftShift {
                self.left_shift = false;
            }
            if key == Key::RightShift {
                self.right_shift = false;
            }
        }
    }
}
