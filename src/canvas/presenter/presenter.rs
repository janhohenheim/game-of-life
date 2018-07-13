use crate::generation_calculator::Change;
use crate::grid::Position;
use crate::interactive_game::Presenter;

#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

#[derive(Debug)]
pub struct Line {
    from: Position,
    to: Position,
    colour: String,
}

#[derive(Debug)]
pub struct Square {
    width: u32,
    height: u32,
    origin: Position,
    colour: String,
}

#[cfg_attr(test, mocked)]
pub trait View {
    fn init_board(&mut self, width: u32, height: u32, lines: &[Line], squares: &[Square]);
    fn draw_squares(&mut self, squares: &[Square]);
}

pub struct CanvasPresenter {
    view: Box<View>,
}
impl CanvasPresenter {
    pub fn new(view: Box<View>) -> Self {
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

    const WIDTH: u32 = 1000;
    const HEIGHT: u32 = 800;

    fn create_mock() -> (Scenario, ViewMock) {
        let scenario = Scenario::new();
        let view = scenario.create_mock_for::<View>();
        (scenario, view)
    }

    #[test]
    fn inits_empty_board() {
        let (scenario, view) = create_mock();
        let mut presenter = CanvasPresenter::new(Box::new(view));
        presenter.init_board(WIDTH, HEIGHT, &Vec::new());
    }

}
