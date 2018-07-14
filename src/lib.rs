#![feature(
    proc_macro,
    wasm_custom_section,
    wasm_import_module,
    crate_in_paths,
    rust_2018_preview,
    const_vec_new
)]

#[cfg(test)]
extern crate mockers;

#[cfg(test)]
extern crate mockers_derive;
extern crate wasm_bindgen;

#[macro_use]
#[cfg(test)]
extern crate lazy_static;

pub mod canvas;
mod controller;
mod generation_calculator;
mod grid;
mod interactive_game;
