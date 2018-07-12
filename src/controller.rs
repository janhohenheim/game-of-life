use crate::constant;
use crate::generation_calculator::Change;
use crate::generation_calculator::Grid;
use crate::interactive_game::InteractiveGame;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

#[cfg_attr(test, mocked)]
pub trait Presenter {
    fn register_controller(&mut self, controller: Weak<RefCell<Controller>>);
    fn init_board(&mut self, width: u32, height: u32);
    fn present_changes(&mut self, changes: &[Change]);
}

#[cfg_attr(test, mocked)]
pub trait GenerationCalculator {
    fn next_generation(&self, grid: &Grid) -> Vec<Change>;
}

pub enum PresenterEvent {
    Changes(Vec<Change>),
    NextStep(),
}

pub struct Controller {
    pub presenter: Box<Presenter>,
    game: Box<InteractiveGame>,
}

impl Controller {
    pub fn new(presenter: Box<Presenter>, game: Box<InteractiveGame>) -> Rc<RefCell<Self>> {
        let controller = Rc::new(RefCell::new(Controller { presenter, game }));
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

    pub fn react_to_event(&mut self, event: PresenterEvent) {
        match event {
            PresenterEvent::NextStep() => {
                let changes = self.game.next_generation();
                if changes.is_empty() {
                    return;
                }
                self.presenter.present_changes(&changes);
            }
            PresenterEvent::Changes(changes) => {
                self.game.accept_changes(&changes);
                self.presenter.present_changes(&changes);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::interactive_game::InteractiveGameMock;
    use mockers::matchers::ANY;
    use mockers::Scenario;

    const CHANGES: [Change; 3] = [
        Change {
            x: 20,
            y: 30,
            is_alive: false,
        },
        Change {
            x: 123,
            y: 432,
            is_alive: true,
        },
        Change {
            x: 223,
            y: 42,
            is_alive: true,
        },
    ];

    fn create_mock() -> (Scenario, PresenterMock, InteractiveGameMock) {
        let scenario = Scenario::new();
        let presenter = scenario.create_mock_for::<Presenter>();
        let game = scenario.create_mock_for::<InteractiveGame>();

        scenario.expect(presenter.register_controller_call(ANY).and_return(()));
        scenario.expect(
            presenter
                .init_board_call(constant::BOARD_WIDTH, constant::BOARD_HEIGHT)
                .and_return(()),
        );

        (scenario, presenter, game)
    }

    fn expect_changes_on_grid(scenario: &Scenario, grid: &GridMock) {
        for change in &CHANGES {
            if change.is_alive {
                scenario.expect(grid.set_alive_at_call(change.x, change.y).and_return(()))
            } else {
                scenario.expect(grid.set_dead_at_call(change.x, change.y).and_return(()))
            }
        }
    }

    #[test]
    fn inits_presenter_with_constants() {
        let (_scenario, presenter, generation_calculator, grid) = create_mock();

        let controller = Controller::new(
            Box::new(presenter),
            Box::new(generation_calculator),
            Box::new(grid),
        );
        let mut controller = controller.borrow_mut();
        controller.start();
    }

    #[test]
    fn does_not_present_stable_generation() {
        let (scenario, presenter, generation_calculator, grid) = create_mock();

        scenario.expect(
            generation_calculator
                .next_generation_call(ANY)
                .and_return(Vec::new()),
        );

        let controller = Controller::new(
            Box::new(presenter),
            Box::new(generation_calculator),
            Box::new(grid),
        );
        let mut controller = controller.borrow_mut();

        controller.start();
        controller.react_to_event(PresenterEvent::NextStep());
    }

    #[test]
    fn presents_next_generation() {
        let (scenario, presenter, generation_calculator, grid) = create_mock();

        scenario.expect(
            generation_calculator
                .next_generation_call(ANY)
                .and_return(CHANGES.to_vec()),
        );
        scenario.expect(
            presenter
                .present_changes_call(CHANGES.as_ref())
                .and_return(()),
        );

        expect_changes_on_grid(&scenario, &grid);

        let controller = Controller::new(
            Box::new(presenter),
            Box::new(generation_calculator),
            Box::new(grid),
        );
        let mut controller = controller.borrow_mut();

        controller.start();
        controller.react_to_event(PresenterEvent::NextStep());
    }

    #[test]
    fn presents_changes() {
        let (scenario, presenter, generation_calculator, grid) = create_mock();

        scenario.expect(
            presenter
                .present_changes_call(CHANGES.as_ref())
                .and_return(()),
        );

        expect_changes_on_grid(&scenario, &grid);

        let controller = Controller::new(
            Box::new(presenter),
            Box::new(generation_calculator),
            Box::new(grid),
        );
        let mut controller = controller.borrow_mut();

        controller.start();
        controller.react_to_event(PresenterEvent::Changes(CHANGES.to_vec()));
    }
}
