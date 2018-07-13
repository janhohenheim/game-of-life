use crate::generation_calculator::Change;
use crate::grid::Position;
use crate::interactive_game::Presenter;

#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

#[derive(Debug, Eq, PartialEq)]
pub struct CanvasViewModel {
    lines: Vec<Line>,
    squares: Vec<Square>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Line {
    from: Position,
    to: Position,
    colour: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Square {
    width: u32,
    height: u32,
    origin: Position,
    colour: String,
}

#[cfg_attr(test, mocked)]
pub trait CanvasView {
    fn init_board(&mut self, width: u32, height: u32, view_model: &CanvasViewModel);
    fn draw_view_model(&mut self, view_model: &CanvasViewModel);
}

pub struct CanvasPresenter {
    view: Box<CanvasView>,
}
impl CanvasPresenter {
    pub fn new(view: Box<CanvasView>) -> Self {
        CanvasPresenter { view }
    }
}
impl Presenter for CanvasPresenter {
    fn init_board(&mut self, width: u32, height: u32, alive_cells: &[Position]) {}
    fn present_changes(&mut self, changes: &[Change]) {}
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::interactive_game::InteractiveGameMock;
    use mockers::matchers::ANY;
    use mockers::Scenario;

    const WIDTH: u32 = 10;
    const HEIGHT: u32 = 8;

    fn create_mock() -> (Scenario, CanvasViewMock) {
        let scenario = Scenario::new();
        let view = scenario.create_mock_for::<CanvasView>();
        (scenario, view)
    }

    fn init_board(scenario: &Scenario, view: &CanvasViewMock) {
        static EMPTY_INITIALIZED_VIEW_MODEL: CanvasViewModel = CanvasViewModel {
            lines: Vec::new(), // To do: Add actual values
            squares: Vec::new(),
        };
        scenario.expect(
            view.init_board_call(WIDTH, HEIGHT, &EMPTY_INITIALIZED_VIEW_MODEL)
                .and_return(()),
        );
    }

    #[test]
    #[should_panic]
    fn panics_when_presenting_changes_and_not_initialized() {
        let (_scenario, view) = create_mock();
        let mut presenter = CanvasPresenter::new(Box::new(view));
        presenter.present_changes(&Vec::new());
    }

    #[test]
    fn inits_empty_board() {
        let (scenario, view) = create_mock();
        init_board(&scenario, &view);

        let mut presenter = CanvasPresenter::new(Box::new(view));
        presenter.init_board(WIDTH, HEIGHT, &Vec::new());
    }

    #[test]
    fn present_changes() {
        let (scenario, view) = create_mock();
        init_board(&scenario, &view);
        static EXPECTED_VIEW_MODEL: CanvasViewModel = CanvasViewModel {
            lines: Vec::new(), // To do: Add actual values
            squares: Vec::new(),
        };
        scenario.expect(
            view.draw_view_model_call(&EXPECTED_VIEW_MODEL)
                .and_return(()),
        );
        let mut presenter = CanvasPresenter::new(Box::new(view));
        presenter.init_board(WIDTH, HEIGHT, &Vec::new());
        let changes = vec![
            Change {
                position: Position { x: 2, y: 3 },
                is_alive: true,
            },
            Change {
                position: Position { x: 3, y: 4 },
                is_alive: true,
            },
            Change {
                position: Position { x: 1, y: 1 },
                is_alive: true,
            },
        ];
        presenter.present_changes(&changes);
    }
}
