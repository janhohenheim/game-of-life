use crate::generation_calculator::Change;
use crate::grid::{Grid, Position};

#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

#[cfg_attr(test, mocked)]
pub trait GenerationCalculator {
    fn next_generation(&self, grid: &Grid) -> Vec<Change>;
}

#[cfg_attr(test, mocked)]
pub trait Presenter {
    fn init_board(&mut self, width: u32, height: u32, alive_cells: &[Position]);
    fn present_changes(&mut self, changes: &[Change]);
}

#[cfg_attr(test, mocked)]
pub trait InteractiveGame {
    fn accept_changes(&mut self, changes: &[Change]);
    fn next_generation(&mut self);
}

pub struct InteractiveGameImpl {
    grid: Box<dyn Grid>,
    generation_calculator: Box<dyn GenerationCalculator>,
    presenter: Box<dyn Presenter>,
}
impl InteractiveGameImpl {
    pub fn new(
        grid: Box<dyn Grid>,
        generation_calculator: Box<dyn GenerationCalculator>,
        mut presenter: Box<dyn Presenter>,
    ) -> Self {
        presenter.init_board(grid.width(), grid.height(), &Vec::new());
        InteractiveGameImpl {
            grid,
            generation_calculator,
            presenter,
        }
    }
}

impl InteractiveGame for InteractiveGameImpl {
    fn accept_changes(&mut self, changes: &[Change]) {
        for change in changes {
            if change.is_alive {
                self.grid.set_alive_at(change.position);
            } else {
                self.grid.set_dead_at(change.position);
            }
        }
    }

    fn next_generation(&mut self) {
        let changes = self.generation_calculator.next_generation(&*self.grid);
        if !changes.is_empty() {
            self.presenter.present_changes(&changes);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::grid::{GridMock, Position};
    use mockers::matchers::ANY;
    use mockers::Scenario;
    const CHANGES: [Change; 3] = [
        Change {
            position: Position { x: 20, y: 30 },
            is_alive: false,
        },
        Change {
            position: Position { x: 123, y: 432 },
            is_alive: true,
        },
        Change {
            position: Position { x: 223, y: 42 },
            is_alive: true,
        },
    ];
    const ALIVE_INITIALIZED_CELLS: [Position; 2] =
        [Position { x: 300, y: 123 }, Position { x: 111, y: 222 }];

    fn create_mock() -> (Scenario, GridMock, GenerationCalculatorMock, PresenterMock) {
        let scenario = Scenario::new();
        let grid = scenario.create_mock_for::<Grid>();
        let generation_calculator = scenario.create_mock_for::<GenerationCalculator>();
        let presenter = scenario.create_mock_for::<Presenter>();
        const WIDTH: u32 = 1000;
        const HEIGHT: u32 = 800;
        scenario.expect(grid.width_call().and_return(WIDTH));
        scenario.expect(grid.height_call().and_return(HEIGHT));
        scenario.expect(grid.is_alive_at_call(ANY).and_return(false));
        for alive_pos in &ALIVE_INITIALIZED_CELLS {
            scenario.expect(grid.is_alive_at_call(*alive_pos).and_return(true));
        }

        scenario.expect(
            presenter
                .init_board_call(WIDTH, HEIGHT, ALIVE_INITIALIZED_CELLS.as_ref())
                .and_return(()),
        );

        (scenario, grid, generation_calculator, presenter)
    }

    #[test]
    fn inits_presenter_with_grid_data() {
        let (_scenario, grid, generation_calculator, presenter) = create_mock();
        let mut game = InteractiveGameImpl::new(
            Box::new(grid),
            Box::new(generation_calculator),
            Box::new(presenter),
        );
    }

    #[test]
    fn does_not_present_stable_generation() {
        let (scenario, grid, generation_calculator, presenter) = create_mock();
        scenario.expect(
            generation_calculator
                .next_generation_call(ANY)
                .and_return(Vec::new()),
        );

        let mut game = InteractiveGameImpl::new(
            Box::new(grid),
            Box::new(generation_calculator),
            Box::new(presenter),
        );
        game.next_generation();
    }

    #[test]
    fn applies_changes() {
        let (scenario, grid, generation_calculator, presenter) = create_mock();
        for change in &CHANGES {
            if change.is_alive {
                scenario.expect(grid.set_alive_at_call(change.position).and_return(()))
            } else {
                scenario.expect(grid.set_dead_at_call(change.position).and_return(()))
            }
        }

        let mut game = InteractiveGameImpl::new(
            Box::new(grid),
            Box::new(generation_calculator),
            Box::new(presenter),
        );
        game.accept_changes(CHANGES.as_ref());
    }

    #[test]
    fn presents_next_generation() {
        let (scenario, grid, generation_calculator, presenter) = create_mock();
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

        let mut game = InteractiveGameImpl::new(
            Box::new(grid),
            Box::new(generation_calculator),
            Box::new(presenter),
        );
        game.next_generation();
    }
}
