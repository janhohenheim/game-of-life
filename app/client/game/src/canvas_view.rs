extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use crate::constant;

#[wasm_bindgen]
extern "C" {
    pub type CanvasRenderingContext2D;

    #[wasm_bindgen(method, setter = fillStyle)]
    pub fn set_fill_style(this: &CanvasRenderingContext2D, fill_style: &str);

    #[wasm_bindgen(method, js_name = fillRect)]
    pub fn fill_rect(this: &CanvasRenderingContext2D, x: u32, y: u32, width: u32, height: u32);

    #[wasm_bindgen(method, js_name = clearRect)]
    pub fn clear_rect(this: &CanvasRenderingContext2D, x: u32, y: u32, width: u32, height: u32);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = Date)]
    fn now() -> f64;

    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn init_board(context: CanvasRenderingContext2D) {
    context.set_fill_style("aquamarine");
    context.fill_rect(0, 0, constant::BOARD_WIDTH, constant::BOARD_HEIGHT);
}
