#[cfg(feature = "desktop")]
pub mod config;

pub mod filesystem;
pub mod keyboard;
pub mod tty;
pub mod vmrun;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "wasm")]
use wasm_bindgen::{closure::Closure, JsCast};
#[cfg(feature = "wasm")]
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement, ImageData, KeyboardEvent};

#[cfg(feature = "wasm")]
use std::cell::RefCell;
#[cfg(feature = "wasm")]
use std::rc::Rc;

#[cfg(feature = "wasm")]
use crate::tty::Tty;
#[cfg(feature = "wasm")]
use mb8::dev::gpu::registers::{TTY_COLS, TTY_ROWS};
#[cfg(feature = "wasm")]
use mb8::vm::VirtualMachine;

#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    match run_wasm() {
        Ok(_) => console::log_1(&"MB8 WASM started".into()),
        Err(e) => console::log_1(&format!("WASM Error: {:?}", e).into()),
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn run_wasm() -> Result<(), JsValue> {
    const OPS_PER_FRAME: u32 = 1024;
    const WIDTH: usize = 320;
    const HEIGHT: usize = 200;

    let vm = Rc::new(RefCell::new(VirtualMachine::default()));
    let tty = Rc::new(RefCell::new(Tty::new(
        TTY_COLS as usize,
        TTY_ROWS as usize,
        1024,
    )));
    let framebuffer = Rc::new(RefCell::new(vec![0u32; WIDTH * HEIGHT]));

    // Load kernel
    static KERNEL: &[u8] = include_bytes!("../../../kernel/main.bin");
    {
        let mut vm = vm.borrow_mut();
        vm.load_rom(KERNEL);
        vm.devices.rand().number = 1;

        crate::filesystem::makefs_wasm(&mut vm);
    }

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas: HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .expect("canvas missing")
        .dyn_into()?;
    canvas.set_width(WIDTH as u32);
    canvas.set_height(HEIGHT as u32);
    canvas.style().set_property("width", "960px")?;
    canvas.style().set_property("height", "600px")?;

    canvas
        .style()
        .set_property("image-rendering", "pixelated")?;
    canvas
        .style()
        .set_property("image-rendering", "crisp-edges")?;

    let ctx: CanvasRenderingContext2d = canvas.get_context("2d")?.unwrap().dyn_into()?;

    // Keyboard
    {
        let vm = vm.clone();
        let keydown = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e| {
            if let Some(b) = map_dom_key(&e) {
                vm.borrow_mut().devices.keyboard().key_pressed(b);
            }
        });
        window.add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref())?;
        keydown.forget();
    }

    // --- Main
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();
    let window_loop = window.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        {
            let mut vm = vm.borrow_mut();
            for _ in 0..OPS_PER_FRAME {
                if vm.halted {
                    break;
                }
                vm.step();
            }
        }

        let tty_read = Rc::new(RefCell::new(0usize));
        // GPU → TTY
        {
            let mut vm = vm.borrow_mut();
            let gpu = vm.devices.gpu();
            let mut tty = tty.borrow_mut();

            let buf = gpu.tty_buffer();
            let mut read = tty_read.borrow_mut();

            while *read < buf.len() {
                tty.write_byte(buf[*read]);
                *read += 1;
            }
        }

        // Render framebuffer to canvas
        {
            let mut fb = framebuffer.borrow_mut();
            tty.borrow_mut().render(&mut fb, WIDTH);

            let mut bytes = vec![0u8; fb.len() * 4];
            for (i, px) in fb.iter().enumerate() {
                let r = ((px >> 16) & 0xFF) as u8;
                let g = ((px >> 8) & 0xFF) as u8;
                let b = (px & 0xFF) as u8;

                bytes[i * 4 + 0] = r;
                bytes[i * 4 + 1] = g;
                bytes[i * 4 + 2] = b;
                bytes[i * 4 + 3] = 0xFF;
            }

            let img = ImageData::new_with_u8_clamped_array_and_sh(
                wasm_bindgen::Clamped(&bytes),
                WIDTH as u32,
                HEIGHT as u32,
            )
            .unwrap();

            ctx.put_image_data(&img, 0.0, 0.0).unwrap();
        }

        window_loop
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }));

    window.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;

    Ok(())
}

#[cfg(feature = "wasm")]
fn map_dom_key(e: &KeyboardEvent) -> Option<u8> {
    match e.key().as_str() {
        "Enter" => Some(b'\n'),
        "Backspace" => Some(0x08),
        "Tab" => Some(0x09),
        "Escape" => Some(0x1B),
        " " => Some(b' '),
        k if k.len() == 1 => Some(k.as_bytes()[0]),
        _ => None,
    }
}
