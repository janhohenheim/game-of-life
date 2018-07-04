extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use super::constant;
use super::js;

#[wasm_bindgen]
pub fn init_board(context: js::CanvasRenderingContext2D) {
    context.set_fill_style("aquamarine");
    context.fill_rect(0, 0, constant::BOARD_WIDTH, constant::BOARD_HEIGHT);
}
