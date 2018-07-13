use crate::constant;
use crate::generation_calculator::Change;
use crate::grid::Grid;
use crate::interactive_game::InteractiveGame;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

pub struct Controller {
    game: Box<InteractiveGame>,
}

impl Controller {
    pub fn new(game: Box<InteractiveGame>) -> Self {
        Controller { game }
    }

    pub fn on_click(&mut self, x: u32, y: u32) {}
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::interactive_game::InteractiveGameMock;
    use mockers::matchers::ANY;
    use mockers::Scenario;

    fn create_mock() -> (Scenario, InteractiveGameMock) {
        let scenario = Scenario::new();
        let game = scenario.create_mock_for::<InteractiveGame>();

        (scenario, game)
    }
}
