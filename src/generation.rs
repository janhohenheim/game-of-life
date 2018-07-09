#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Change {
    pub x: usize,
    pub y: usize,
    pub is_alive: bool,
}

#[cfg_attr(test, mocked)]
pub trait Grid {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn is_alive_at(&self, x: usize, y: usize) -> bool;
    fn set_alive_at(&mut self, x: usize, y: usize);
    fn set_dead_at(&mut self, x: usize, y: usize);
}

#[derive(Debug)]
pub struct DeathFrameGenerationCalculator;

impl DeathFrameGenerationCalculator {
    fn new() -> Self {
        DeathFrameGenerationCalculator
    }

    fn next_generation(&self, grid: &dyn Grid) -> Vec<Change> {
        let mut changes = Vec::new();
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let neighbours = count_neighbours_at(grid, x, y).expect("x or y out of bounds");
                let is_alive = grid.is_alive_at(x, y);
                if is_alive && (neighbours < 2 || neighbours > 3) {
                    changes.push(Change {
                        x,
                        y,
                        is_alive: false,
                    });
                } else if !is_alive && neighbours == 3 {
                    changes.push(Change {
                        x,
                        y,
                        is_alive: true,
                    })
                }
            }
        }
        changes
    }
}

fn count_neighbours_at(grid: &dyn Grid, x: usize, y: usize) -> Option<usize> {
    if x >= grid.width() || y >= grid.height() {
        return None;
    }

    let top = y == 0;
    let right = x == grid.width() - 1;
    let bottom = y == grid.height() - 1;
    let left = x == 0;

    let mut neighbours = 0;
    if !top && grid.is_alive_at(x, y - 1) {
        neighbours += 1;
    }
    if !top && !right && grid.is_alive_at(x + 1, y - 1) {
        neighbours += 1;
    }
    if !right && grid.is_alive_at(x + 1, y) {
        neighbours += 1;
    }
    if !right && !bottom && grid.is_alive_at(x + 1, y + 1) {
        neighbours += 1;
    }
    if !bottom && grid.is_alive_at(x, y + 1) {
        neighbours += 1;
    }
    if !left && !bottom && grid.is_alive_at(x - 1, y + 1) {
        neighbours += 1;
    }
    if !left && grid.is_alive_at(x - 1, y) {
        neighbours += 1;
    }
    if !left && !top && grid.is_alive_at(x - 1, y - 1) {
        neighbours += 1;
    }
    Some(neighbours)
}

#[cfg(test)]
mod death_framed_generation_calculator_test {
    use super::*;
    use mockers::matchers::*;
    use mockers::Scenario;

    fn create_mock_with_size(width: usize, height: usize) -> (Scenario, GridMock) {
        let scenario = Scenario::new();
        let grid = scenario.create_mock_for::<Grid>();
        scenario.expect(grid.width_call().and_return_clone(width).times(..));
        scenario.expect(grid.height_call().and_return_clone(height).times(..));
        scenario.expect(
            grid.is_alive_at_call(ANY, ANY)
                .and_return_clone(false)
                .times(..),
        );
        (scenario, grid)
    }

    fn set_grid_alive_at(scenario: &Scenario, grid: &GridMock, alive_cells: &[(usize, usize)]) {
        for alive_cell in alive_cells {
            scenario.expect(
                grid.is_alive_at_call(alive_cell.0, alive_cell.1)
                    .and_return_clone(true)
                    .times(..),
            );
        }
    }

    #[test]
    fn dead_grid_stays_dead() {
        let (_, grid) = create_mock_with_size(5, 4);

        let generation_calculator = DeathFrameGenerationCalculator::new();
        let changes = generation_calculator.next_generation(&grid);

        assert_eq!(0, changes.len());
    }

    #[test]
    fn lone_alive_cell_dies() {
        let (scenario, grid) = create_mock_with_size(5, 4);
        set_grid_alive_at(&scenario, &grid, &[(1, 1)]);

        let generation_calculator = DeathFrameGenerationCalculator::new();
        let changes = generation_calculator.next_generation(&grid);

        assert_eq!(1, changes.len());
        let expected = Change {
            x: 1,
            y: 1,
            is_alive: false,
        };
        assert_eq!(expected, changes[0]);
    }

    #[test]
    fn alive_cell_in_corner_dies() {
        let (scenario, grid) = create_mock_with_size(5, 4);
        set_grid_alive_at(&scenario, &grid, &[(0, 0)]);

        let generation_calculator = DeathFrameGenerationCalculator::new();
        let changes = generation_calculator.next_generation(&grid);

        assert_eq!(1, changes.len());
        let expected = Change {
            x: 0,
            y: 0,
            is_alive: false,
        };
        assert_eq!(expected, changes[0]);
    }

    #[test]
    fn alive_cell_in_corner_with_single_neighbour_dies() {
        let (scenario, grid) = create_mock_with_size(3, 3);
        set_grid_alive_at(&scenario, &grid, &[(0, 0), (1, 1)]);

        let generation_calculator = DeathFrameGenerationCalculator::new();
        let changes = generation_calculator.next_generation(&grid);

        assert_eq!(2, changes.len());
        let expected = Change {
            x: 0,
            y: 0,
            is_alive: false,
        };
        assert_eq!(expected, changes[0]);
        let expected = Change {
            x: 1,
            y: 1,
            is_alive: false,
        };
        assert_eq!(expected, changes[1]);
    }

    #[test]
    fn dead_cell_with_three_neighbours_resurrects() {
        let (scenario, grid) = create_mock_with_size(3, 3);
        /*
         * O | . | .
         * O | O | .
         * . | . | .
         */
        set_grid_alive_at(&scenario, &grid, &[(0, 0), (0, 1), (1, 1)]);

        let generation_calculator = DeathFrameGenerationCalculator {};
        let changes = generation_calculator.next_generation(&grid);

        assert_eq!(1, changes.len());
        let expected = Change {
            x: 1,
            y: 0,
            is_alive: true,
        };
        assert_eq!(expected, changes[0]);
    }

    #[test]
    fn alive_cell_with_four_neighbours_dies() {
        let (scenario, grid) = create_mock_with_size(3, 2);
        /*
         * . | O | O
         * O | O | O
         */
        set_grid_alive_at(&scenario, &grid, &[(1, 0), (2, 0), (0, 1), (1, 1), (2, 1)]);

        let generation_calculator = DeathFrameGenerationCalculator {};
        let changes = generation_calculator.next_generation(&grid);

        assert_eq!(3, changes.len());
        let expected = Change {
            x: 0,
            y: 0,
            is_alive: true,
        };
        assert_eq!(expected, changes[0]);
        let expected = Change {
            x: 1,
            y: 0,
            is_alive: false,
        };
        assert_eq!(expected, changes[1]);
        let expected = Change {
            x: 1,
            y: 1,
            is_alive: false,
        };
        assert_eq!(expected, changes[2]);
    }

    #[test]
    fn dead_cell_with_four_neighbours_stays_dead() {
        let (scenario, grid) = create_mock_with_size(3, 2);
        /*
         * O | O | O
         * O | . | O
         */
        set_grid_alive_at(&scenario, &grid, &[(0, 0), (1, 0), (2, 0), (0, 1), (2, 1)]);

        let generation_calculator = DeathFrameGenerationCalculator {};
        let changes = generation_calculator.next_generation(&grid);

        assert_eq!(1, changes.len());
    }

    #[test]
    fn block_stays_block() {
        let (scenario, grid) = create_mock_with_size(4, 4);
        /*
         * . | . | . | .
         * . | O | O | .
         * . | O | O | .
         * . | . | . | .
         */
        set_grid_alive_at(&scenario, &grid, &[(1, 1), (1, 2), (2, 1), (2, 2)]);

        let generation_calculator = DeathFrameGenerationCalculator {};
        let changes = generation_calculator.next_generation(&grid);

        assert_eq!(0, changes.len());
    }

    #[test]
    fn blinker_period_one_becomes_period_two() {
        let (scenario, grid) = create_mock_with_size(3, 3);
        /*
         * . | . | .
         * O | O | O
         * . | . | .
         */
        set_grid_alive_at(&scenario, &grid, &[(0, 1), (1, 1), (2, 1)]);

        let generation_calculator = DeathFrameGenerationCalculator {};
        let changes = generation_calculator.next_generation(&grid);

        /*
         * . | O | .
         * . | O | .
         * . | O | .
         */
        assert_eq!(4, changes.len());
        let expected = Change {
            x: 1,
            y: 0,
            is_alive: true,
        };
        assert_eq!(expected, changes[0]);
        let expected = Change {
            x: 0,
            y: 1,
            is_alive: false,
        };
        assert_eq!(expected, changes[1]);
        let expected = Change {
            x: 2,
            y: 1,
            is_alive: false,
        };
        assert_eq!(expected, changes[2]);
        let expected = Change {
            x: 1,
            y: 2,
            is_alive: true,
        };
        assert_eq!(expected, changes[3]);
    }

    #[test]
    fn blinker_period_two_becomes_period_one() {
        let (scenario, grid) = create_mock_with_size(3, 3);
        /*
         * . | O | .
         * . | O | .
         * . | O | .
         */
        set_grid_alive_at(&scenario, &grid, &[(1, 0), (1, 1), (1, 2)]);

        let generation_calculator = DeathFrameGenerationCalculator {};
        let changes = generation_calculator.next_generation(&grid);

        /*
         * . | . | .
         * O | O | O
         * . | . | .
         */
        assert_eq!(4, changes.len());
        let expected = Change {
            x: 1,
            y: 0,
            is_alive: false,
        };
        assert_eq!(expected, changes[0]);
        let expected = Change {
            x: 0,
            y: 1,
            is_alive: true,
        };
        assert_eq!(expected, changes[1]);
        let expected = Change {
            x: 2,
            y: 1,
            is_alive: true,
        };
        assert_eq!(expected, changes[2]);
        let expected = Change {
            x: 1,
            y: 2,
            is_alive: false,
        };
        assert_eq!(expected, changes[3]);
    }
}
