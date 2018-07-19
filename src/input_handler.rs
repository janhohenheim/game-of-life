use crate::coordinate_translator::CoordinateTranslator;
use crate::grid::Position;
use crate::grid_info::GridInfo;
use crate::interactive_game::InteractiveGame;

#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

#[cfg_attr(test, mocked)]
pub trait ClickableInputHandler {
    fn on_click(&mut self, x: u32, y: u32);
    fn on_timer(&mut self);
}

pub struct ClickableInputHandlerImpl {
    game: Box<InteractiveGame>,
    coordinate_translator: Box<CoordinateTranslator>,
    grid_info: GridInfo,
}

impl ClickableInputHandlerImpl {
    pub fn new(
        game: Box<InteractiveGame>,
        coordinate_translator: Box<CoordinateTranslator>,
        grid_info: GridInfo,
    ) -> Self {
        ClickableInputHandlerImpl {
            game,
            coordinate_translator,
            grid_info,
        }
    }

    fn get_cell_location_from_coordinates(&self, x: u32, y: u32) -> Option<(u32, u32)> {
        let global_position = Position { x, y };
        if let Some(position) = self.coordinate_translator.to_local(&global_position) {
            use crate::canvas::view::js;
            let msg = format!("{:#?}", position);
            js::console::log(&msg);
            if position.x > self.grid_info.width || position.y > self.grid_info.height {
                None
            } else {
                let cell_width = self.grid_info.width / self.grid_info.columns;
                let cell_height = self.grid_info.height / self.grid_info.rows;
                let cell_x = position.x / cell_width;
                let cell_y = position.y / cell_height;
                Some((cell_x, cell_y))
            }
        } else {
            None
        }
    }
}

impl ClickableInputHandler for ClickableInputHandlerImpl {
    fn on_click(&mut self, x: u32, y: u32) {
        let cell_position = self.get_cell_location_from_coordinates(x, y);
        if let Some((x, y)) = cell_position {
            let position = Position { x, y };
            self.game.toggle_cell(&position);
        }
    }

    fn on_timer(&mut self) {
        self.game.next_generation();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::coordinate_translator::CoordinateTranslatorMock;
    use crate::grid::Position;
    use crate::grid_info::GridInfo;
    use crate::interactive_game::InteractiveGameMock;

    use mockers::matchers::ANY;
    use mockers::Scenario;

    fn create_mock() -> (
        Scenario,
        InteractiveGameMock,
        CoordinateTranslatorMock,
        GridInfo,
    ) {
        let scenario = Scenario::new();
        let game = scenario.create_mock_for::<InteractiveGame>();
        let coordinate_translator = scenario.create_mock_for::<CoordinateTranslator>();
        let grid_info = GridInfo {
            width: 10,
            height: 8,
            rows: 4,
            columns: 5,
        };
        (scenario, game, coordinate_translator, grid_info)
    }

    #[test]
    fn calls_next_gen_on_timer() {
        let (scenario, game, coordinate_translator, grid_info) = create_mock();
        scenario.expect(game.next_generation_call().and_return(()));
        let mut input_handler = ClickableInputHandlerImpl::new(
            Box::new(game),
            Box::new(coordinate_translator),
            grid_info,
        );
        input_handler.on_timer();
    }

    #[test]
    fn ignores_out_of_bounds_clicks() {
        let (scenario, game, coordinate_translator, grid_info) = create_mock();
        scenario.expect(
            coordinate_translator
                .to_local_call(ANY)
                .and_call_clone(|&pos| Some(pos))
                .times(3),
        );

        let mut input_handler = ClickableInputHandlerImpl::new(
            Box::new(game),
            Box::new(coordinate_translator),
            grid_info,
        );
        input_handler.on_click(11, 9);
        input_handler.on_click(10, 9);
        input_handler.on_click(11, 8);
    }

    #[test]
    fn ignores_untranslated_clicks() {
        let (scenario, game, coordinate_translator, grid_info) = create_mock();
        const POSITION: Position = Position { x: 3, y: 2 };
        scenario.expect(
            coordinate_translator
                .to_local_call(&POSITION)
                .and_return(None),
        );

        let mut input_handler = ClickableInputHandlerImpl::new(
            Box::new(game),
            Box::new(coordinate_translator),
            grid_info,
        );
        input_handler.on_click(POSITION.x, POSITION.y);
    }

    #[test]
    fn toggles_cell_on_click() {
        let (scenario, game, coordinate_translator, grid_info) = create_mock();
        const POSITION: Position = Position { x: 1, y: 2 };
        scenario.expect(game.toggle_cell_call(&POSITION).and_return(()));
        scenario.expect(
            coordinate_translator
                .to_local_call(ANY)
                .and_call(|&pos| Some(pos)),
        );
        let mut input_handler = ClickableInputHandlerImpl::new(
            Box::new(game),
            Box::new(coordinate_translator),
            grid_info,
        );
        input_handler.on_click(2, 5);
    }
}
