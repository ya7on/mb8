use mb8_cli::vm;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    vm::run_wasm();
}
