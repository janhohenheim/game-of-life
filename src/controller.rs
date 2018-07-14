use crate::grid_info::GridInfo;
use crate::interactive_game::InteractiveGame;

#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

pub struct ClickableController {
    game: Box<InteractiveGame>,
    grid_info: GridInfo,
}

impl ClickableController {
    pub fn new(game: Box<InteractiveGame>, grid_info: GridInfo) -> Self {
        ClickableController { game, grid_info }
    }

    pub fn on_click(&mut self, x: u32, y: u32) {
        unimplemented!()
    }

    pub fn on_timer(&mut self) {
        unimplemented!()
    }
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
