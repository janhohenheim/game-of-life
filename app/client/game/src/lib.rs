#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

use wasm_bindgen::prelude::*;
extern crate wasm_bindgen;

mod grid;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
