use generation::{Change, GenerationCalculator};
use grid::Grid;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub trait Presenter {
    fn register_controller(&mut self, Weak<RefCell<Controller>>);
    fn present_change(&mut self, change: Change);
}

pub enum PresenterEvent {
    Change(Change),
}

pub trait Controller {
    fn notify(&mut self, event: PresenterEvent);
}

pub struct ControllerImpl<G: Grid> {
    presenter: Box<Presenter>,
    generation_calculator: Box<GenerationCalculator<G>>,
}

impl<G: Grid> ControllerImpl<G> {
    fn new(
        presenter: Box<Presenter>,
        generation_calculator: Box<GenerationCalculator<G>>,
    ) -> Rc<RefCell<Self>> {
        let controller = Rc::new(RefCell::new(ControllerImpl {
            presenter,
            generation_calculator,
        }));
        let second = Rc::downgrade(&controller);
        controller
            .borrow_mut()
            .presenter
            .register_controller(second);
        controller
    }
}

impl<G: Grid> Controller for ControllerImpl<G> {
    fn notify(&mut self, event: PresenterEvent) {}
}
