#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

use crate::grid::{Grid, Position};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Change {
    pub position: Position,
    pub is_alive: bool,
}

#[cfg_attr(test, mocked)]
pub trait GenerationCalculator {
    fn next_generation(&self, grid: &Grid) -> Vec<Change>;
}

#[derive(Debug)]
pub struct GenerationCalculatorImpl;

impl GenerationCalculatorImpl {
    fn new() -> Self {
        GenerationCalculatorImpl
    }
}
impl GenerationCalculator for GenerationCalculatorImpl {
    fn next_generation(&self, grid: &dyn Grid) -> Vec<Change> {
        let mut changes = Vec::new();
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let position = Position { x, y };
                let neighbours = count_neighbours_at(grid, position).expect("x or y out of bounds");
                let is_alive = grid.is_alive_at(position);
                if is_alive && (neighbours < 2 || neighbours > 3) {
                    changes.push(Change {
                        position,
                        is_alive: false,
                    });
                } else if !is_alive && neighbours == 3 {
                    changes.push(Change {
                        position,
                        is_alive: true,
                    })
                }
            }
        }
        changes
    }
}

fn count_neighbours_at(grid: &dyn Grid, position: Position) -> Option<u32> {
    if position.x >= grid.width() || position.y >= grid.height() {
        return None;
    }

    let top = position.y == 0;
    let right = position.x == grid.width() - 1;
    let bottom = position.y == grid.height() - 1;
    let left = position.x == 0;

    let mut neighbours = 0;
    if !top && grid.is_alive_at(Position {
        x: position.x,
        y: position.y - 1,
    }) {
        neighbours += 1;
    }
    if !top && !right && grid.is_alive_at(Position {
        x: position.x + 1,
        y: position.y - 1,
    }) {
        neighbours += 1;
    }
    if !right && grid.is_alive_at(Position {
        x: position.x + 1,
        y: position.y,
    }) {
        neighbours += 1;
    }
    if !right && !bottom && grid.is_alive_at(Position {
        x: position.x + 1,
        y: position.y + 1,
    }) {
        neighbours += 1;
    }
    if !bottom && grid.is_alive_at(Position {
        x: position.x,
        y: position.y + 1,
    }) {
        neighbours += 1;
    }
    if !left && !bottom && grid.is_alive_at(Position {
        x: position.x - 1,
        y: position.y + 1,
    }) {
        neighbours += 1;
    }
    if !left && grid.is_alive_at(Position {
        x: position.x - 1,
        y: position.y,
    }) {
        neighbours += 1;
    }
    if !left && !top && grid.is_alive_at(Position {
        x: position.x - 1,
        y: position.y - 1,
    }) {
        neighbours += 1;
    }
    Some(neighbours)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::grid::{GridMock, Position};
    use mockers::matchers::*;
    use mockers::Scenario;

    fn create_mock_with_size(width: u32, height: u32) -> (Scenario, GridMock) {
        let scenario = Scenario::new();
        let grid = scenario.create_mock_for::<Grid>();
        scenario.expect(grid.width_call().and_return_clone(width).times(..));
        scenario.expect(grid.height_call().and_return_clone(height).times(..));
        scenario.expect(grid.is_alive_at_call(ANY).and_return_clone(false).times(..));
        (scenario, grid)
    }

    fn set_grid_alive_at(scenario: &Scenario, grid: &GridMock, positions: &[Position]) {
        for position in positions {
            scenario.expect(
                grid.is_alive_at_call(*position)
                    .and_return_clone(true)
                    .times(..),
            );
        }
    }

    #[test]
    fn dead_grid_stays_dead() {
        let (_, grid) = create_mock_with_size(5, 4);

        let generation_calculator = GenerationCalculatorImpl::new();
        let changes = generation_calculator.next_generation(&grid);

        assert_eq!(0, changes.len());
    }

    #[test]
    fn lone_alive_cell_dies() {
        let (scenario, grid) = create_mock_with_size(5, 4);
        set_grid_alive_at(&scenario, &grid, &[Position { x: 1, y: 1 }]);

        let generation_calculator = GenerationCalculatorImpl::new();
        let changes = generation_calculator.next_generation(&grid);

        assert_eq!(1, changes.len());
        let expected = Change {
            position: Position { x: 1, y: 1 },
            is_alive: false,
        };
        assert_eq!(expected, changes[0]);
    }

    #[test]
    fn alive_cell_in_corner_dies() {
        let (scenario, grid) = create_mock_with_size(5, 4);
        set_grid_alive_at(&scenario, &grid, &[Position { x: 0, y: 0 }]);

        let generation_calculator = GenerationCalculatorImpl::new();
        let changes = generation_calculator.next_generation(&grid);

        assert_eq!(1, changes.len());
        let expected = Change {
            position: Position { x: 0, y: 0 },
            is_alive: false,
        };
        assert_eq!(expected, changes[0]);
    }

    #[test]
    fn alive_cell_in_corner_with_single_neighbour_dies() {
        let (scenario, grid) = create_mock_with_size(3, 3);
        set_grid_alive_at(
            &scenario,
            &grid,
            &[Position { x: 0, y: 0 }, Position { x: 1, y: 1 }],
        );

        let generation_calculator = GenerationCalculatorImpl::new();
        let changes = generation_calculator.next_generation(&grid);

        assert_eq!(2, changes.len());
        let expected = Change {
            position: Position { x: 0, y: 0 },
            is_alive: false,
        };
        assert_eq!(expected, changes[0]);
        let expected = Change {
            position: Position { x: 1, y: 1 },
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
        set_grid_alive_at(
            &scenario,
            &grid,
            &[
                Position { x: 0, y: 0 },
                Position { x: 0, y: 1 },
                Position { x: 1, y: 1 },
            ],
        );

        let generation_calculator = GenerationCalculatorImpl {};
        let changes = generation_calculator.next_generation(&grid);

        assert_eq!(1, changes.len());
        let expected = Change {
            position: Position { x: 1, y: 0 },
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
        set_grid_alive_at(
            &scenario,
            &grid,
            &[
                Position { x: 1, y: 0 },
                Position { x: 2, y: 0 },
                Position { x: 0, y: 1 },
                Position { x: 1, y: 1 },
                Position { x: 2, y: 1 },
            ],
        );

        let generation_calculator = GenerationCalculatorImpl {};
        let changes = generation_calculator.next_generation(&grid);

        assert_eq!(3, changes.len());
        let expected = Change {
            position: Position { x: 0, y: 0 },
            is_alive: true,
        };
        assert_eq!(expected, changes[0]);
        let expected = Change {
            position: Position { x: 1, y: 0 },
            is_alive: false,
        };
        assert_eq!(expected, changes[1]);
        let expected = Change {
            position: Position { x: 1, y: 1 },
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
        set_grid_alive_at(
            &scenario,
            &grid,
            &[
                Position { x: 0, y: 0 },
                Position { x: 1, y: 0 },
                Position { x: 2, y: 0 },
                Position { x: 0, y: 1 },
                Position { x: 2, y: 1 },
            ],
        );

        let generation_calculator = GenerationCalculatorImpl {};
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
        set_grid_alive_at(
            &scenario,
            &grid,
            &[
                Position { x: 1, y: 1 },
                Position { x: 1, y: 2 },
                Position { x: 2, y: 1 },
                Position { x: 2, y: 2 },
            ],
        );

        let generation_calculator = GenerationCalculatorImpl {};
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
        set_grid_alive_at(
            &scenario,
            &grid,
            &[
                Position { x: 0, y: 1 },
                Position { x: 1, y: 1 },
                Position { x: 2, y: 1 },
            ],
        );

        let generation_calculator = GenerationCalculatorImpl {};
        let changes = generation_calculator.next_generation(&grid);

        /*
         * . | O | .
         * . | O | .
         * . | O | .
         */
        assert_eq!(4, changes.len());
        let expected = Change {
            position: Position { x: 1, y: 0 },
            is_alive: true,
        };
        assert_eq!(expected, changes[0]);
        let expected = Change {
            position: Position { x: 0, y: 1 },
            is_alive: false,
        };
        assert_eq!(expected, changes[1]);
        let expected = Change {
            position: Position { x: 2, y: 1 },
            is_alive: false,
        };
        assert_eq!(expected, changes[2]);
        let expected = Change {
            position: Position { x: 1, y: 2 },
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
        set_grid_alive_at(
            &scenario,
            &grid,
            &[
                Position { x: 1, y: 0 },
                Position { x: 1, y: 1 },
                Position { x: 1, y: 2 },
            ],
        );

        let generation_calculator = GenerationCalculatorImpl {};
        let changes = generation_calculator.next_generation(&grid);

        /*
         * . | . | .
         * O | O | O
         * . | . | .
         */
        assert_eq!(4, changes.len());
        let expected = Change {
            position: Position { x: 1, y: 0 },
            is_alive: false,
        };
        assert_eq!(expected, changes[0]);
        let expected = Change {
            position: Position { x: 0, y: 1 },
            is_alive: true,
        };
        assert_eq!(expected, changes[1]);
        let expected = Change {
            position: Position { x: 2, y: 1 },
            is_alive: true,
        };
        assert_eq!(expected, changes[2]);
        let expected = Change {
            position: Position { x: 1, y: 2 },
            is_alive: false,
        };
        assert_eq!(expected, changes[3]);
    }
}
