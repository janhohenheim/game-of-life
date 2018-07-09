use generation::Change;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::constant;
#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;
use generation::Grid;

#[cfg_attr(test, mocked)]
pub trait Presenter {
    fn register_controller(&mut self, controller: Weak<RefCell<Controller>>);
    fn init_board(&mut self, width: u32, height: u32);
    fn present_change(&mut self, change: Change);
}

#[cfg_attr(test, mocked)]
pub trait GenerationCalculator {
    fn next_generation(&self, grid: &Grid) -> Vec<Change>;
}

pub enum PresenterEvent {
    Change(Change),
    NextStep(),
}

pub trait Controller {
    fn start(&mut self);
    fn react_to_event(&mut self, event: PresenterEvent);
}

pub struct ControllerImpl {
    pub presenter: Box<Presenter>,
    pub generation_calculator: Box<GenerationCalculator>,
}

impl ControllerImpl {
    fn new(
        presenter: Box<Presenter>,
        generation_calculator: Box<GenerationCalculator>,
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

impl Controller for ControllerImpl {
    fn start(&mut self) {
        self.presenter
            .init_board(constant::BOARD_WIDTH, constant::BOARD_HEIGHT)
    }
    fn react_to_event(&mut self, event: PresenterEvent) {}
}

#[cfg(test)]
mod controller_impl_test {
    use super::*;
    use mockers::Scenario;
    use mockers::matchers::ANY;

    #[test]
    fn inits_presenter_with_constants() {
        let scenario = Scenario::new();
        let presenter = scenario.create_mock_for::<Presenter>();
        let generation_calculator = scenario.create_mock_for::<GenerationCalculator>();

        scenario.expect(presenter.register_controller_call(ANY).and_return(()));
        scenario.expect(
            presenter
                .init_board_call(constant::BOARD_WIDTH, constant::BOARD_HEIGHT)
                .and_return(()),
        );

        let controller = ControllerImpl::new(Box::new(presenter), Box::new(generation_calculator));
        let mut controller = controller.borrow_mut();
        controller.start();
        //let presenter = &controller.presenter.downcast::<MockPresenter>().unwrap();
        //assert_eq!(constant::BOARD_WIDTH, presenter.width);
        //assert_eq!(constant::BOARD_HEIGHT, presenter.height);
    }
}
