extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type CanvasRenderingContext2D;

    #[wasm_bindgen(method, setter = fillStyle)]
    pub fn set_fill_style(this: &CanvasRenderingContext2D, fill_style: &str);

    #[wasm_bindgen(method, js_name = fillRect)]
    pub fn fill_rect(this: &CanvasRenderingContext2D, x: u32, y: u32, width: u32, height: u32);

    #[wasm_bindgen(method, js_name = clearRect)]
    pub fn clear_rect(this: &CanvasRenderingContext2D, x: u32, y: u32, width: u32, height: u32);

    #[wasm_bindgen(method, js_name = beginPath)]
    pub fn begin_path(this: &CanvasRenderingContext2D);

    #[wasm_bindgen(method, js_name = moveTo)]
    pub fn move_to(this: &CanvasRenderingContext2D, x: u32, y: u32);

    #[wasm_bindgen(method, js_name = lineTo)]
    pub fn line_to(this: &CanvasRenderingContext2D, x: u32, y: u32);

    #[wasm_bindgen(method)]
    pub fn fill(this: &CanvasRenderingContext2D);

    #[wasm_bindgen(method)]
    pub fn stroke(this: &CanvasRenderingContext2D);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

//pub fn alert(s: &str);
}
