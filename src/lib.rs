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
mod coordinate_translator;
pub mod entrypoint;
mod generation_calculator;
mod grid;
mod grid_info;
mod input_handler;
mod interactive_game;
