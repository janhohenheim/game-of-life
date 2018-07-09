#![feature(proc_macro, wasm_custom_section, wasm_import_module, crate_in_paths)]

#[cfg(test)]
extern crate mockers;

#[cfg(test)]
extern crate mockers_derive;
extern crate wasm_bindgen;

pub mod canvas_view;
mod controller;
mod generation;
mod grid;
mod constant;
