extern crate wasm_bindgen;
use crate::canvas::presenter::CanvasPresenter;
use crate::canvas::view::js;
use crate::canvas::view::CanvasViewImpl;
use crate::controller::{ClickableController, ClickableControllerImpl};
use crate::generation_calculator::GenerationCalculatorImpl;
use crate::grid::GridImpl;
use crate::grid_info::GridInfo;
use crate::interactive_game::InteractiveGameImpl;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    controller: Box<ClickableController>,
}

#[wasm_bindgen]
impl Game {
    pub fn new(context: js::CanvasRenderingContext2D) -> Self {
        let grid_info = GridInfo {
            width: 1000,
            height: 1000,
            rows: 100,
            columns: 100,
        };
        let view = Box::new(CanvasViewImpl::new(context, grid_info.clone()));
        let presenter = Box::new(CanvasPresenter::new(view, grid_info.clone()));
        let generation_calculator = Box::new(GenerationCalculatorImpl::new());
        let grid = Box::new(GridImpl::new(grid_info.columns, grid_info.rows));
        let game = Box::new(InteractiveGameImpl::new(
            grid,
            generation_calculator,
            presenter,
        ));
        let controller = Box::new(ClickableControllerImpl::new(game, grid_info));
        Game { controller }
    }

    #[wasm_bindgen]
    pub fn on_click(&mut self, x: u32, y: u32) {
        (self as &mut dyn ClickableController).on_click(x, y);
    }

    #[wasm_bindgen]
    pub fn on_timer(&mut self) {
        (self as &mut dyn ClickableController).on_timer();
    }
}

impl ClickableController for Game {
    fn on_click(&mut self, x: u32, y: u32) {
        self.controller.on_click(x, y);
    }

    fn on_timer(&mut self) {
        self.controller.on_timer();
    }
}
