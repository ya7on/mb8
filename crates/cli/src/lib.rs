use std::{cell::RefCell, rc::Rc};

use mb8::vm::VirtualMachine;
use minifb::{Scale, ScaleMode, Window, WindowOptions};
use tty::render_tty;
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast,
};

mod tty;

#[wasm_bindgen(start)]
pub fn start_wasm() {
    let rom = include_bytes!("../../../kernel/main.bin");
    let user = vec![
        ("sh", include_bytes!("../../../user/sh.bin")),
        ("help", include_bytes!("../../../user/help.bin")),
    ];

    let mut vm = VirtualMachine::default();
    vm.load_rom(rom);

    // makefs
    let mut fs = vec![0u8; 65536];
    let mut blocks = 1;
    let mut files = 0;
    for (filename, data) in user {
        let size = (data.len() / 256) + 1;

        // Add to zero block
        let zero_block_start = files * 16;
        fs[zero_block_start] = 1;
        fs[zero_block_start + 1] = blocks;
        fs[zero_block_start + 2] = size as u8;

        let chars = filename.as_bytes();
        if chars.len() > 8 {
            eprintln!(
                "Error: File name {} is too long. Max 8 characters allowed.",
                filename
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

    vm.devices.rand().number = 1;

    let buf = vec![0u32; 320 * 200];

    let mut options = WindowOptions::default();
    options.scale = Scale::X2;
    options.scale_mode = ScaleMode::Center;
    let window = Window::new("minifb-container", 320, 200, options).unwrap();

    let state = Rc::new(RefCell::new(State {
        vm,
        window,
        buf,
        i: 0,
    }));

    {
        let mut st = state.borrow_mut();
        step_once(&mut *st);
    }

    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();
    let state_rc = state.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        {
            let mut st = state_rc.borrow_mut();

            if st.vm.halted || !st.window.is_open() || st.i >= 100_000 {
                return;
            }

            step_once(&mut *st);
        }

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

#[derive(Debug)]
struct State {
    vm: VirtualMachine,
    window: Window,
    buf: Vec<u32>,
    i: u32,
}

fn step_once(st: &mut State) {
    for _ in 0..10 {
        if st.vm.halted {
            break;
        }
        let pc = st.vm.step();

        web_sys::console::log_1(&format!("Step {}", pc).into());
    }

    let gpu = st.vm.devices.gpu();
    let tty = gpu.tty_buffer();
    render_tty(tty, st.buf.as_mut_slice());

    let _ = st.window.update_with_buffer(&st.buf, 320, 200);
    st.i += 1;
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .expect("no global `window`")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
