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

type ObservablePresenter = Rc<RefCell<Presenter>>;
pub struct Controller {
    presenter: ObservablePresenter,
    game: Box<InteractiveGame>,
}

impl Controller {
    pub fn new(presenter: ObservablePresenter, game: Box<InteractiveGame>) -> Rc<RefCell<Self>> {
        let controller = Rc::new(RefCell::new(Controller { presenter, game }));
        let second = Rc::downgrade(&controller);
        controller
            .borrow_mut()
            .presenter
            .borrow_mut()
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

    #[test]
    fn inits_presenter_with_constants() {
        let (_scenario, presenter, game) = create_mock();

        let controller = Controller::new(Box::new(presenter), Box::new(game));
        let mut controller = controller.borrow_mut();
        controller.start();
    }

    #[test]
    fn does_not_present_stable_generation() {
        let (scenario, presenter, game) = create_mock();

        scenario.expect(game.next_generation_call().and_return(Vec::new()));

        let controller = Controller::new(Box::new(presenter), Box::new(game));
        let mut controller = controller.borrow_mut();

        controller.start();
        controller.react_to_event(PresenterEvent::NextStep());
    }

    #[test]
    fn presents_next_generation() {
        let (scenario, presenter, game) = create_mock();

        scenario.expect(game.next_generation_call().and_return(CHANGES.to_vec()));

        scenario.expect(
            presenter
                .present_changes_call(CHANGES.as_ref())
                .and_return(()),
        );

        let controller = Controller::new(Box::new(presenter), Box::new(game));
        let mut controller = controller.borrow_mut();

        controller.start();
        controller.react_to_event(PresenterEvent::NextStep());
    }

    #[test]
    fn presents_changes() {
        let (scenario, presenter, game) = create_mock();

        scenario.expect(game.accept_changes_call(CHANGES.as_ref()).and_return(()));

        scenario.expect(
            presenter
                .present_changes_call(CHANGES.as_ref())
                .and_return(()),
        );

        let controller = Controller::new(Box::new(presenter), Box::new(game));
        let mut controller = controller.borrow_mut();

        controller.start();
        controller.react_to_event(PresenterEvent::Changes(CHANGES.to_vec()));
    }
}
