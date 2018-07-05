use grid::{Grid, OneDimensionalBoolGrid};
use generation::{Change, DeathFrameGenerationCalculator, GenerationCalculator};

pub trait Presenter {
    fn set_present_dead_at_fn(&mut self, fun: Fn(usize, usize));
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
