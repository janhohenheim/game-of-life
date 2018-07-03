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
    /*

        #[test]
        fn TestAliveCellInCornerWithNeighbourDies()
        {
            IGrid grid = new Grid(3, 3);
            grid[0, 0] = new Cell(true);
            grid[1, 1] = new Cell(true);
            let next_generation = generation_calculator.next_generation(grid);

            IGrid dead_grid = new Grid(3, 3);
            assert_eq!(dead_grid, next_generation);
        }

        #[test]
        fn TestAliveCellWithTwoNeighboursSurvives()
        {
            IGrid grid = new Grid(3, 3);
            /*
             * O | . | .
             * O | O | .
             * . | . | .
             */
            grid[0, 0] = new Cell(true);
            grid[0, 1] = new Cell(true);
            grid[1, 1] = new Cell(true);
            let next_generation = generation_calculator.next_generation(grid);

            Assert.IsTrue(next_generation[0, 0].IsAlive);
            Assert.IsTrue(next_generation[0, 1].IsAlive);
            Assert.IsTrue(next_generation[1, 1].IsAlive);
        }

        #[test]
        fn TestAliveCellWithFourNeighboursDies()
        {
            IGrid grid = new Grid(3, 3);
            /*
             * O | . | O
             * O | O | .
             * O | . | .
             */
            grid[0, 0] = new Cell(true);
            grid[0, 1] = new Cell(true);
            grid[0, 2] = new Cell(true);
            grid[2, 0] = new Cell(true);
            grid[1, 1] = new Cell(true);
            let next_generation = generation_calculator.next_generation(grid);

            Assert.IsFalse(next_generation[1, 1].IsAlive);
        }

        #[test]
        fn TestDeadCellWithThreeNeighboursResurects()
        {
            IGrid grid = new Grid(3, 3);
            /*
             * O | . | O
             * O | O | .
             * O | . | .
             */
            grid[0, 0] = new Cell(true);
            grid[0, 1] = new Cell(true);
            grid[0, 2] = new Cell(true);
            grid[2, 0] = new Cell(true);
            grid[1, 1] = new Cell(true);
            let next_generation = generation_calculator.next_generation(grid);

            Assert.IsTrue(next_generation[1, 2].IsAlive);
        }

        #[test]
        fn TestDeadCellWithFourNeighboursStaysdead()
        {
            IGrid grid = new Grid(3, 3);
            /*
             * O | . | O
             * O | O | .
             * O | . | .
             */
            grid[0, 0] = new Cell(true);
            grid[0, 1] = new Cell(true);
            grid[0, 2] = new Cell(true);
            grid[2, 0] = new Cell(true);
            grid[1, 1] = new Cell(true);
            let next_generation = generation_calculator.next_generation(grid);

            Assert.IsFalse(next_generation[1, 0].IsAlive);
        }

        #[test]
        fn TestBlockStaysBlock()
        {
            IGrid grid = new Grid(4, 4);
            /*
             * . | . | . | .
             * . | O | O | .
             * . | O | O | .
             * . | . | . | .
             */
            grid[1, 1] = new Cell(true);
            grid[1, 2] = new Cell(true);
            grid[2, 1] = new Cell(true);
            grid[2, 2] = new Cell(true);
            let next_generation = generation_calculator.next_generation(grid);

            assert_eq!(grid, next_generation);
        }

        #[test]
        fn TestBlinkerPeriodOne()
        {
            IGrid grid = new Grid(3, 3);
            /*
             * . | . | .
             * O | O | O
             * . | . | .
             */
            grid[0, 1] = new Cell(true);
            grid[1, 1] = new Cell(true);
            grid[2, 1] = new Cell(true);
            let next_generation = generation_calculator.next_generation(grid);

            IGrid expected = new Grid(3, 3);
            /*
             * . | O | .
             * . | O | .
             * . | O | .
             */
            expected[1, 0] = new Cell(true);
            expected[1, 1] = new Cell(true);
            expected[1, 2] = new Cell(true);
            assert_eq!(expected, next_generation);
        }

        #[test]
        fn TestBlinkerPeriodTwo()
        {
            IGrid grid = new Grid(3, 3);
            /*
            * . | O | .
            * . | O | .
            * . | O | .
            */
            grid[1, 0] = new Cell(true);
            grid[1, 1] = new Cell(true);
            grid[1, 2] = new Cell(true);
            let next_generation = generation_calculator.next_generation(grid);

            IGrid expected = new Grid(3, 3);
            /*
             * . | . | .
             * O | O | O
             * . | . | .
             */
            expected[0, 1] = new Cell(true);
            expected[1, 1] = new Cell(true);
            expected[2, 1] = new Cell(true);
            assert_eq!(expected, next_generation);
        }
    */
}
