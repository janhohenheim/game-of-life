use super::grid::Grid;

trait GenerationCalculator<T: Grid> {
    fn next_generation(&self, grid: T) -> T;
}

#[derive(Debug)]
struct DeathFrameGenerationCalculator;

impl<T: Grid> GenerationCalculator<T> for DeathFrameGenerationCalculator {
    fn next_generation(&self, grid: T) -> T {
        T::new(grid.width(), grid.height())
    }
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
