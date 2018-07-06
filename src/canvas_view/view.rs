extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use crate::constant;
use super::js;

#[wasm_bindgen]
pub fn init_board(context: &js::CanvasRenderingContext2D) {
    context.set_fill_style("aquamarine");
    context.fill_rect(0, 0, constant::BOARD_WIDTH, constant::BOARD_HEIGHT);
    context.set_fill_style("white");
    context.begin_path();
    context.move_to(75, 50);
    context.line_to(100, 75);
    context.stroke();
    context.line_to(100, 25);
    context.line_to(200, 25);
    context.fill();
}
