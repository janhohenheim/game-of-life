use crate::canvas::constant;
use crate::generation_calculator::Change;
use crate::grid::Position;
use crate::grid_info::GridInfo;
use crate::interactive_game::Presenter;

#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

#[derive(Debug, Eq, PartialEq)]
pub struct CanvasViewModel {
    pub lines: Vec<Line>,
    pub squares: Vec<Square>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Line {
    pub from: Position,
    pub to: Position,
    pub colour: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Square {
    pub width: u32,
    pub height: u32,
    pub origin: Position,
    pub colour: String,
}

#[cfg_attr(test, mocked)]
pub trait CanvasView {
    fn init_board(&mut self, view_model: &CanvasViewModel);
    fn draw_view_model(&mut self, view_model: &CanvasViewModel);
}

pub struct CanvasPresenter {
    view: Box<CanvasView>,
    is_initialized: bool,
    grid_info: GridInfo,
}

impl CanvasPresenter {
    pub fn new(view: Box<CanvasView>, grid_info: GridInfo) -> Self {
        CanvasPresenter {
            view,
            is_initialized: false,
            grid_info,
        }
    }
}

impl Presenter for CanvasPresenter {
    fn init_board(&mut self, alive_cells: &[Position]) {
        if self.is_initialized {
            panic!(
                "Initialized board multiple times \
                 Did you accidentally call .init_board() multiple times?"
            );
        }
        self.is_initialized = true;
        let lines = get_lines(&self.grid_info);
        let cells_as_changes = alive_cells
            .iter()
            .map(|&position| Change {
                position,
                is_alive: true,
            })
            .collect::<Vec<_>>();
        let squares = get_squares(&self.grid_info, &cells_as_changes);
        let view_model = CanvasViewModel { lines, squares };
        self.view.init_board(&view_model);
    }

    fn present_changes(&mut self, changes: &[Change]) {
        if !self.is_initialized {
            panic!(
                "Presenting changes to board before initilizing. \
                 Did you forget to call .init_board()?"
            );
        }
        let squares = get_squares(&self.grid_info, changes);
        let view_model = CanvasViewModel {
            lines: Vec::new(),
            squares,
        };
        self.view.draw_view_model(&view_model);
    }
}

fn get_lines(grid_info: &GridInfo) -> Vec<Line> {
    let mut lines = Vec::new();
    for y in 1..grid_info.rows {
        lines.push(Line {
            from: Position {
                x: 0,
                y: y * (grid_info.height / grid_info.rows),
            },
            to: Position {
                x: grid_info.width,
                y: y * (grid_info.height / grid_info.rows),
            },
            colour: constant::LINE_COLOUR.into(),
        })
    }
    for x in 1..grid_info.columns {
        lines.push(Line {
            from: Position {
                x: x * (grid_info.width / grid_info.columns),
                y: 0,
            },
            to: Position {
                x: x * (grid_info.width / grid_info.columns),
                y: grid_info.height,
            },
            colour: constant::LINE_COLOUR.into(),
        })
    }
    lines
}

fn get_squares(grid_info: &GridInfo, changes: &[Change]) -> Vec<Square> {
    let mut squares = Vec::new();
    for change in changes {
        let cell_width = grid_info.width / grid_info.columns - 2;
        let cell_height = grid_info.height / grid_info.rows - 2;
        squares.push(Square {
            width: cell_width,
            height: cell_height,
            origin: Position {
                x: cell_width / 2 + change.position.x,
                y: cell_height / 2 + change.position.y,
            },
            colour: if change.is_alive {
                constant::ALIVE_CELL_COLOUR.into()
            } else {
                constant::DEAD_CELL_COLOUR.into()
            },
        });
    }
    squares
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::canvas::constant;
    use mockers::Scenario;

    const WIDTH: u32 = 10;
    const HEIGHT: u32 = 8;

    const GRID_INFO: GridInfo = GridInfo {
        width: constant::CANVAS_WIDTH,
        height: constant::CANVAS_HEIGHT,
        rows: 8,
        columns: 10,
    };

    fn create_mock() -> (Scenario, CanvasViewMock) {
        let scenario = Scenario::new();
        let view = scenario.create_mock_for::<CanvasView>();
        (scenario, view)
    }

    fn init_board(scenario: &Scenario, view: &CanvasViewMock) {
        lazy_static! {
            static ref EMPTY_INITIALIZED_VIEW_MODEL: CanvasViewModel = {
                let mut lines = Vec::new();
                for y in 1..HEIGHT {
                    lines.push(Line {
                        from: Position {
                            x: 0,
                            y: y * (constant::CANVAS_HEIGHT / HEIGHT),
                        },
                        to: Position {
                            x: constant::CANVAS_WIDTH,
                            y: y * (constant::CANVAS_HEIGHT / HEIGHT),
                        },
                        colour: constant::LINE_COLOUR.into(),
                    })
                }
                for x in 1..WIDTH {
                    lines.push(Line {
                        from: Position {
                            x: x * (constant::CANVAS_WIDTH / WIDTH),
                            y: 0,
                        },
                        to: Position {
                            x: x * (constant::CANVAS_WIDTH / WIDTH),
                            y: constant::CANVAS_HEIGHT,
                        },
                        colour: constant::LINE_COLOUR.into(),
                    })
                }
                CanvasViewModel {
                    lines,
                    squares: Vec::new(),
                }
            };
        }
        scenario.expect(
            view.init_board_call(&*EMPTY_INITIALIZED_VIEW_MODEL)
                .and_return(()),
        );
    }

    #[test]
    #[should_panic]
    fn panics_when_presenting_changes_and_not_initialized() {
        let (_scenario, view) = create_mock();
        let mut presenter = CanvasPresenter::new(Box::new(view), GRID_INFO.clone());
        presenter.present_changes(&Vec::new());
    }

    #[test]
    fn inits_empty_board() {
        let (scenario, view) = create_mock();
        init_board(&scenario, &view);

        let mut presenter = CanvasPresenter::new(Box::new(view), GRID_INFO.clone());
        presenter.init_board(&Vec::new());
    }

    #[test]
    #[should_panic]
    fn panics_initializing_multiple_times() {
        let (scenario, view) = create_mock();
        init_board(&scenario, &view);

        let mut presenter = CanvasPresenter::new(Box::new(view), GRID_INFO.clone());
        presenter.init_board(&Vec::new());
        presenter.init_board(&Vec::new());
    }

    #[test]
    fn present_changes() {
        let (scenario, view) = create_mock();
        init_board(&scenario, &view);
        lazy_static!{
            static ref EXPECTED_VIEW_MODEL: CanvasViewModel = {
                let mut squares = Vec::new();
                for (x, y) in &[(2, 3), (3, 4), (1, 1)] {
                    let cell_width = constant::CANVAS_WIDTH / WIDTH - 2;
                    let cell_height = constant::CANVAS_HEIGHT / HEIGHT - 2;
                    squares.push(Square {
                        width: cell_width,
                        height: cell_height,
                        origin: Position {
                            x: cell_width / 2 + x,
                            y: cell_height / 2 + y,
                        },
                        colour: constant::ALIVE_CELL_COLOUR.into(),
                    });
                }

                CanvasViewModel {
                    lines: Vec::new(), // We only send changes to the view, so this is empty
                    squares,
                }
            };
        }
        scenario.expect(
            view.draw_view_model_call(&*EXPECTED_VIEW_MODEL)
                .and_return(()),
        );
        let mut presenter = CanvasPresenter::new(Box::new(view), GRID_INFO.clone());
        presenter.init_board(&Vec::new());
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
