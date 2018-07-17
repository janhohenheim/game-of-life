extern crate wasm_bindgen;
use crate::canvas::constant;
use crate::canvas::presenter::CanvasPresenter;
use crate::canvas::view::js;
use crate::canvas::view::CanvasViewImpl;
use crate::generation_calculator::GenerationCalculatorImpl;
use crate::grid::GridImpl;
use crate::grid_info::GridInfo;
use crate::input_handler::{ClickableInputHandler, ClickableInputHandlerImpl};
use crate::interactive_game::InteractiveGameImpl;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct EntryPoint {
    input_handler: Box<ClickableInputHandler>,
}

#[wasm_bindgen]
impl EntryPoint {
    pub fn new(context: js::CanvasRenderingContext2D) -> Self {
        let grid_info = GridInfo {
            width: constant::CANVAS_WIDTH,
            height: constant::CANVAS_HEIGHT,
            rows: 100,
            columns: 100,
        };
        let view = Box::new(CanvasViewImpl::new(context));
        let presenter = Box::new(CanvasPresenter::new(view, grid_info.clone()));
        let generation_calculator = Box::new(GenerationCalculatorImpl::new());
        let grid = Box::new(GridImpl::new(grid_info.columns, grid_info.rows));
        let game = Box::new(InteractiveGameImpl::new(
            grid,
            generation_calculator,
            presenter,
        ));
        let input_handler = Box::new(ClickableInputHandlerImpl::new(game, grid_info));
        EntryPoint { input_handler }
    }

    #[wasm_bindgen]
    pub fn on_click(&mut self, x: u32, y: u32) {
        (self as &mut dyn ClickableInputHandler).on_click(x, y);
    }

    #[wasm_bindgen]
    pub fn on_timer(&mut self) {
        (self as &mut dyn ClickableInputHandler).on_timer();
    }
}

impl ClickableInputHandler for EntryPoint {
    fn on_click(&mut self, x: u32, y: u32) {
        self.input_handler.on_click(x, y);
    }

    fn on_timer(&mut self) {
        self.input_handler.on_timer();
    }
}
