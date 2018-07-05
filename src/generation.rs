use crate::grid::Grid;

#[derive(Debug, Eq, PartialEq)]
pub struct Change {
    x: usize,
    y: usize,
    is_alive: bool,
}

pub trait GenerationCalculator<T: Grid> {
    fn next_generation(&self, grid: &T) -> Vec<Change>;
}

#[derive(Debug)]
pub struct DeathFrameGenerationCalculator;

impl<T> GenerationCalculator<T> for DeathFrameGenerationCalculator
where
    T: Grid,
{
    fn next_generation(&self, grid: &T) -> Vec<Change> {
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

fn count_neighbours_at<T: Grid>(grid: &T, x: usize, y: usize) -> Option<usize> {
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
    use super::{Change, DeathFrameGenerationCalculator, GenerationCalculator};
    use crate::grid::{Grid, OneDimensionalBoolGrid};

    #[test]
    fn dead_grid_stays_dead() {
        let generation_calculator = DeathFrameGenerationCalculator {};
        let dead_grid = OneDimensionalBoolGrid::new(5, 4);
        let changes = generation_calculator.next_generation(&dead_grid);

        assert_eq!(0, changes.len());
    }

    #[test]
    fn lone_alive_cell_dies() {
        let generation_calculator = DeathFrameGenerationCalculator {};
        let mut grid = OneDimensionalBoolGrid::new(3, 3);
        grid.set_alive_at(1, 1);
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
        let generation_calculator = DeathFrameGenerationCalculator {};
        let mut grid = OneDimensionalBoolGrid::new(3, 3);
        grid.set_alive_at(0, 0);
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
        let generation_calculator = DeathFrameGenerationCalculator {};
        let mut grid = OneDimensionalBoolGrid::new(3, 3);
        grid.set_alive_at(0, 0);
        grid.set_alive_at(1, 1);
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
        let generation_calculator = DeathFrameGenerationCalculator {};
        let mut grid = OneDimensionalBoolGrid::new(3, 3);
        /*
         * O | . | .
         * O | O | .
         * . | . | .
         */
        grid.set_alive_at(0, 0);
        grid.set_alive_at(0, 1);
        grid.set_alive_at(1, 1);
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
        let generation_calculator = DeathFrameGenerationCalculator {};
        let mut grid = OneDimensionalBoolGrid::new(3, 2);
        /*
         * . | O | O
         * O | O | O
         */
        grid.set_alive_at(1, 0);
        grid.set_alive_at(2, 0);
        grid.set_alive_at(0, 1);
        grid.set_alive_at(1, 1);
        grid.set_alive_at(2, 1);
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
        let generation_calculator = DeathFrameGenerationCalculator {};
        let mut grid = OneDimensionalBoolGrid::new(3, 2);
        /*
         * O | O | O
         * O | . | O
         */
        grid.set_alive_at(0, 0);
        grid.set_alive_at(1, 0);
        grid.set_alive_at(2, 0);
        grid.set_alive_at(0, 1);
        grid.set_alive_at(2, 1);
        let changes = generation_calculator.next_generation(&grid);

        assert_eq!(1, changes.len());
    }

    #[test]
    fn block_stays_block() {
        let generation_calculator = DeathFrameGenerationCalculator {};
        let mut grid = OneDimensionalBoolGrid::new(4, 4);
        /*
         * . | . | . | .
         * . | O | O | .
         * . | O | O | .
         * . | . | . | .
         */
        grid.set_alive_at(1, 1);
        grid.set_alive_at(1, 2);
        grid.set_alive_at(2, 1);
        grid.set_alive_at(2, 2);
        let changes = generation_calculator.next_generation(&grid);
        assert_eq!(0, changes.len());
    }

    #[test]
    fn blinker_period_one_becomes_period_two() {
        let generation_calculator = DeathFrameGenerationCalculator {};
        let mut grid = OneDimensionalBoolGrid::new(3, 3);
        /*
         * . | . | .
         * O | O | O
         * . | . | .
         */
        grid.set_alive_at(0, 1);
        grid.set_alive_at(1, 1);
        grid.set_alive_at(2, 1);
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
        let generation_calculator = DeathFrameGenerationCalculator {};
        let mut grid = OneDimensionalBoolGrid::new(3, 3);
        /*
         * . | O | .
         * . | O | .
         * . | O | .
         */
        grid.set_alive_at(1, 0);
        grid.set_alive_at(1, 1);
        grid.set_alive_at(1, 2);
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
