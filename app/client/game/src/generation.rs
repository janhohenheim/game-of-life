use super::grid::Grid;

trait GenerationCalculator<T: Grid> {
    fn next_generation(&self, grid: T) -> T;
}

#[derive(Debug)]
struct DeathFrameGenerationCalculator;

impl<T> GenerationCalculator<T> for DeathFrameGenerationCalculator
where
    T: Grid + Clone,
{
    fn next_generation(&self, grid: T) -> T {
        let mut next_generation = grid.clone();
        for x in 0..grid.width() {
            for y in 0..grid.height() {
                let neighbours = count_neighbours_at(&grid, x, y).expect("x or y out of bounds");
                match neighbours {
                    n if n < 2 || n > 3 => next_generation.set_dead_at(x, y),
                    n if n == 3 => next_generation.set_alive_at(x, y),
                    _ => {}
                };
            }
        }
        next_generation
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
    use crate::grid::{Grid, OneDimensionalBoolGrid};
    use super::{DeathFrameGenerationCalculator, GenerationCalculator};

    #[test]
    fn dead_grid_stays_dead() {
        let generation_calculator = DeathFrameGenerationCalculator {};
        let dead_grid = OneDimensionalBoolGrid::new(5, 4);
        let next_generation = generation_calculator.next_generation(dead_grid);

        let dead_grid = OneDimensionalBoolGrid::new(5, 4);
        assert_eq!(dead_grid, next_generation);
    }

    #[test]
    fn lone_alive_cell_dies() {
        let generation_calculator = DeathFrameGenerationCalculator {};
        let mut grid = OneDimensionalBoolGrid::new(3, 3);
        grid.set_alive_at(1, 1);
        let next_generation = generation_calculator.next_generation(grid);

        let dead_grid = OneDimensionalBoolGrid::new(3, 3);
        assert_eq!(dead_grid, next_generation);
    }

    #[test]
    fn alive_cell_in_corner_dies() {
        let generation_calculator = DeathFrameGenerationCalculator {};
        let mut grid = OneDimensionalBoolGrid::new(3, 3);
        grid.set_dead_at(0, 0);
        let next_generation = generation_calculator.next_generation(grid);

        let dead_grid = OneDimensionalBoolGrid::new(3, 3);
        assert_eq!(dead_grid, next_generation);
    }

    #[test]
    fn alive_cell_in_corner_with_single_neighbour_dies() {
        let generation_calculator = DeathFrameGenerationCalculator {};
        let mut grid = OneDimensionalBoolGrid::new(3, 3);
        grid.set_alive_at(0, 0);
        grid.set_alive_at(1, 1);
        let next_generation = generation_calculator.next_generation(grid);

        let dead_grid = OneDimensionalBoolGrid::new(3, 3);
        assert_eq!(dead_grid, next_generation);
    }

    #[test]
    fn alive_cell_with_two_neighbours_survives() {
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
        let next_generation = generation_calculator.next_generation(grid);

        assert!(next_generation.is_alive_at(0, 0));
        assert!(next_generation.is_alive_at(0, 1));
        assert!(next_generation.is_alive_at(1, 1));
    }

    #[test]
    fn alive_cell_with_four_neighbours_dies() {
        let generation_calculator = DeathFrameGenerationCalculator {};
        let mut grid = OneDimensionalBoolGrid::new(3, 3);
        /*
         * O | . | O
         * O | O | .
         * O | . | .
         */
        grid.set_alive_at(0, 0);
        grid.set_alive_at(0, 1);
        grid.set_alive_at(0, 2);
        grid.set_alive_at(2, 0);
        grid.set_alive_at(1, 1);
        let next_generation = generation_calculator.next_generation(grid);

        assert_eq!(false, next_generation.is_alive_at(1, 1));
    }

    #[test]
    fn dead_cell_with_three_neighbours_resurrects() {
        let generation_calculator = DeathFrameGenerationCalculator {};
        let mut grid = OneDimensionalBoolGrid::new(3, 3);
        /*
         * O | . | O
         * O | O | .
         * O | . | .
         */
        grid.set_alive_at(0, 0);
        grid.set_alive_at(0, 1);
        grid.set_alive_at(0, 2);
        grid.set_alive_at(2, 0);
        grid.set_alive_at(1, 1);
        let next_generation = generation_calculator.next_generation(grid);

        assert!(next_generation.is_alive_at(1, 2));
    }

    #[test]
    fn dead_cell_with_four_neighbours_stays_dead() {
        let generation_calculator = DeathFrameGenerationCalculator {};
        let mut grid = OneDimensionalBoolGrid::new(3, 3);
        /*
         * O | . | O
         * O | O | .
         * O | . | .
         */
        grid.set_alive_at(0, 0);
        grid.set_alive_at(0, 1);
        grid.set_alive_at(0, 2);
        grid.set_alive_at(2, 0);
        grid.set_alive_at(1, 1);
        let next_generation = generation_calculator.next_generation(grid);

        assert_eq!(false, next_generation.is_alive_at(1, 0));
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
        let original = grid.clone();
        let next_generation = generation_calculator.next_generation(grid);
        assert_eq!(original, next_generation);
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
        let next_generation = generation_calculator.next_generation(grid);

        let mut expected = OneDimensionalBoolGrid::new(3, 3);
        /*
         * . | O | .
         * . | O | .
         * . | O | .
         */
        expected.set_alive_at(1, 0);
        expected.set_alive_at(1, 1);
        expected.set_alive_at(1, 2);
        assert_eq!(expected, next_generation);
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
        let next_generation = generation_calculator.next_generation(grid);

        let mut expected = OneDimensionalBoolGrid::new(3, 3);
        /*
         * . | . | .
         * O | O | O
         * . | . | .
         */
        expected.set_alive_at(0, 1);
        expected.set_alive_at(1, 1);
        expected.set_alive_at(2, 1);
        assert_eq!(expected, next_generation);
    }
}
