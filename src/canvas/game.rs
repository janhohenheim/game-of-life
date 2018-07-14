extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game;

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        Game
    }
    pub fn start(&mut self) {}
}
