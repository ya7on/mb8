use std::{
    collections::HashMap,
    path::PathBuf,
    time::{Duration, Instant},
};

use clap::Parser;
use mb8::{
    dev::gpu::registers::{TTY_COLS, TTY_ROWS},
    vm,
};
use mb8c::compile;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use tty::Tty;

mod config;
mod tty;

const OPS_PER_FRAME: u32 = 1024;
const RENDER_INTERVAL: u32 = 1000;

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
        Key::Comma => b',',
        Key::Period => b'.',
        Key::Slash => b'/',

        _ => return None,
    };

    Some(ch)
}
#[allow(clippy::too_many_lines)]
fn run_vm(kernel: PathBuf, user: Vec<PathBuf>, seed: Option<u16>) {
    let Ok(rom) = std::fs::read(kernel) else {
        return;
    };
    let mut vm = vm::VirtualMachine::default();
    vm.load_rom(&rom);

    let Ok(mut window) = Window::new("MB8", 640, 480, WindowOptions::default()) else {
        return;
    };

    let seed = seed.unwrap_or(1);

    let mut tty = Tty::new(TTY_COLS as usize, TTY_ROWS as usize, 1024);
    vm.devices.rand().number = (seed as u8).max(1);

    // MakeFS
    let mut fs = vec![0u8; 65536];
    let mut blocks = 1;
    let mut files = 0;
    for path in user {
        let Ok(data) = std::fs::read(&path) else {
            continue;
        };
        let Ok(name) = path.file_stem().ok_or("Failed to get file name") else {
            continue;
        };

        let size = (data.len() / 256) + 1;

        // Add to zero block
        let zero_block_start = files * 16;
        fs[zero_block_start] = 1;
        fs[zero_block_start + 1] = blocks;
        fs[zero_block_start + 2] = size as u8;

        let chars = name.as_encoded_bytes();
        if chars.len() > 8 {
            eprintln!(
                "Error: File name {} is too long. Max 8 characters allowed.",
                name.to_string_lossy()
            );
            return;
        }
        for (i, c) in chars.iter().enumerate() {
            fs[zero_block_start + 3 + i] = *c;
        }

        let block_start = blocks as usize * 256;
        for (i, d) in data.iter().enumerate() {
            fs[block_start + i] = *d;
        }

        blocks += size as u8;
        files += 1;
    }

    let Ok(fs) = fs.try_into() else {
        eprintln!("Failed to convert file system");
        return;
    };
    vm.devices.disk().set(fs);

    let mut buf = vec![0u32; 320 * 200];

    let mut ticks = RENDER_INTERVAL - 1;
    let mut last_render = Instant::now();
    let mut left_shift = false;
    let mut right_shift = false;

    let mut key_last_pressed = HashMap::new();

    while !vm.halted && window.is_open() {
        ticks = ticks.wrapping_add(1);

        for key in window.get_keys_pressed(KeyRepeat::No) {
            let current_time = Instant::now();

            if let Some(last_time) = key_last_pressed.get(&key) {
                if current_time.duration_since(*last_time) < Duration::from_millis(100) {
                    continue;
                }
            }
            if key == Key::LeftShift {
                left_shift = true;
                continue;
            }
            if key == Key::RightShift {
                right_shift = true;
                continue;
            }

            if let Some(mapped_char) = map_key_to_char(key, left_shift || right_shift) {
                vm.devices.keyboard().key_pressed(mapped_char);
            }

            key_last_pressed.insert(key, current_time);
        }
        for key in window.get_keys_released() {
            if key == Key::LeftShift {
                left_shift = false;
            }
            if key == Key::RightShift {
                right_shift = false;
            }
        }

        for _ in 0..OPS_PER_FRAME {
            if vm.halted {
                break;
            }
            vm.step();
        }

        if last_render.elapsed() >= Duration::from_millis(16) {
            let gpu = vm.devices.gpu();
            for &byte in gpu.tty_buffer() {
                tty.write_byte(byte);
            }

            tty.render(buf.as_mut_slice(), 320);

            if window.update_with_buffer(&buf, 320, 200).is_err() {
                return;
            }
            last_render = Instant::now();
        }
    }
}

fn main() {
    let cli = config::Cli::parse();

    match cli.command {
        config::Commands::Run { kernel, user } => {
            run_vm(kernel, user, cli.seed);
        }
        config::Commands::Compile { source } => {
            let code = match std::fs::read_to_string(source) {
                Ok(code) => code,
                Err(err) => {
                    eprintln!("Failed to read source file: {err}");
                    return;
                }
            };
            match compile(&code) {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("Compilation error: {err:?}");
                }
            }
        }
    }
}
