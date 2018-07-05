use grid::Grid;
use generation::{Change, GenerationCalculator};

pub trait Presenter {
    fn set_present_change_fn(&mut self, fun: Fn(Change));
}

pub trait Controller {}

pub struct ControllerImpl<G: Grid> {
    presenter: Box<Presenter>,
    generation_calculator: Box<GenerationCalculator<G>>,
}

impl<G: Grid> ControllerImpl<G> {
    fn new(presenter: Box<Presenter>, generation_calculator: Box<GenerationCalculator<G>>) -> Self {
        ControllerImpl {
            presenter,
            generation_calculator,
        }
    }
}

impl<G: Grid> Controller for ControllerImpl<G> {}
