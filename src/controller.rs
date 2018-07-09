use crate::constant;
use crate::generation::Change;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
#[cfg(test)]
extern crate mockers;
use crate::generation::Grid;
#[cfg(test)]
use mockers_derive::mocked;

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

pub struct Controller {
    pub presenter: Box<Presenter>,
    pub generation_calculator: Box<GenerationCalculator>,
}

impl Controller {
    pub fn new(
        presenter: Box<Presenter>,
        generation_calculator: Box<GenerationCalculator>,
    ) -> Rc<RefCell<Self>> {
        let controller = Rc::new(RefCell::new(Controller {
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

    pub fn start(&mut self) {
        self.presenter
            .init_board(constant::BOARD_WIDTH, constant::BOARD_HEIGHT)
    }

    pub fn react_to_event(&mut self, event: PresenterEvent) {}
}

#[cfg(test)]
mod controller_impl_test {
    use super::*;
    use mockers::matchers::ANY;
    use mockers::Scenario;

    fn create_mock() -> (Scenario, PresenterMock, GenerationCalculatorMock) {
        let scenario = Scenario::new();
        let presenter = scenario.create_mock_for::<Presenter>();
        let generation_calculator = scenario.create_mock_for::<GenerationCalculator>();

        scenario.expect(presenter.register_controller_call(ANY).and_return(()));

        (scenario, presenter, generation_calculator)
    }

    #[test]
    fn inits_presenter_with_constants() {
        let (scenario, presenter, generation_calculator) = create_mock();

        scenario.expect(
            presenter
                .init_board_call(constant::BOARD_WIDTH, constant::BOARD_HEIGHT)
                .and_return(()),
        );

        let controller = Controller::new(Box::new(presenter), Box::new(generation_calculator));
        let mut controller = controller.borrow_mut();
        controller.start();
    }

}
