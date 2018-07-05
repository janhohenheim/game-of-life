#![feature(proc_macro, wasm_custom_section, wasm_import_module, crate_in_paths)]

extern crate wasm_bindgen;

mod grid;
mod generation;
mod controller;
pub mod canvas_view;
