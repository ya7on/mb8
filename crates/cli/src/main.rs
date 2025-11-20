use std::path::PathBuf;

use clap::Parser;
use mb8::vm;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use tty::render_tty;

mod config;
mod tty;

#[allow(clippy::too_many_lines)]
fn map_key_to_char(key: Key, shift: bool) -> Option<u8> {
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

        _ => return None,
    };

    Some(ch)
}

fn run_vm(file: PathBuf) {
    let Ok(source) = std::fs::read(file) else {
        return;
    };
    let mut vm = vm::VirtualMachine::default();
    vm.load_rom(&source);

    let Ok(mut window) = Window::new("MB8", 640, 480, WindowOptions::default()) else {
        return;
    };

    let mut buf = vec![0u32; 320 * 200];

    while !vm.halted && window.is_open() {
        for key in window.get_keys_pressed(KeyRepeat::No) {
            let Some(char) =
                map_key_to_char(key, window.is_key_pressed(Key::LeftShift, KeyRepeat::Yes))
            else {
                continue;
            };
            vm.devices.keyboard().key_pressed(char);
        }

        vm.step();

        let tty = vm.devices.gpu().tty_buffer();
        render_tty(tty, buf.as_mut_slice());

        if window.update_with_buffer(&buf, 320, 200).is_err() {
            return;
        }
    }
}

fn main() {
    let cli = config::Cli::parse();

    match cli.command {
        config::Commands::Run { file } => {
            run_vm(file);
        }
    }
}
