use super::grid::Grid;

trait GenerationCalculator<T: Grid> {
    fn next_generation(&self, grid: T) -> T;
}

#[derive(Debug)]
struct ProceduralGenerationCalculator;

impl<T: Grid> GenerationCalculator<T> for ProceduralGenerationCalculator {
    fn next_generation(&self, grid: T) -> T {
        T::new(grid.width(), grid.height())
    }
}

#[cfg(test)]
mod procedural_generation_calculator_test {
    use crate::grid::{Grid, OneDimensionalBoolGrid};
    use super::{GenerationCalculator, ProceduralGenerationCalculator};

    #[test]
    fn dead_grid_stays_dead() {
        let generation_calculator = ProceduralGenerationCalculator {};
        let dead_grid = OneDimensionalBoolGrid::new(5, 4);
        let next_generation = generation_calculator.next_generation(dead_grid);

        let dead_grid = OneDimensionalBoolGrid::new(5, 4);
        assert_eq!(dead_grid, next_generation);
    }
    /*

        #[test]
        fn TestLoneAliveCellDies()
        {
            IGrid grid = new Grid(3, 3);
            grid[1, 1] = new Cell(true);
            var nextGen = _game.NextGeneration(grid);

            IGrid deadGrid = new Grid(3, 3);
            assert_eq!(deadGrid, nextGen);
        }


        #[test]
        fn TestAliveCellInCornerDies()
        {
            IGrid grid = new Grid(3, 3);
            grid[0, 0] = new Cell(true);
            var nextGen = _game.NextGeneration(grid);

            IGrid deadGrid = new Grid(3, 3);
            assert_eq!(deadGrid, nextGen);
        }

        #[test]
        fn TestAliveCellInCornerWithNeighbourDies()
        {
            IGrid grid = new Grid(3, 3);
            grid[0, 0] = new Cell(true);
            grid[1, 1] = new Cell(true);
            var nextGen = _game.NextGeneration(grid);

            IGrid deadGrid = new Grid(3, 3);
            assert_eq!(deadGrid, nextGen);
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
            var nextGen = _game.NextGeneration(grid);

            Assert.IsTrue(nextGen[0, 0].IsAlive);
            Assert.IsTrue(nextGen[0, 1].IsAlive);
            Assert.IsTrue(nextGen[1, 1].IsAlive);
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
            var nextGen = _game.NextGeneration(grid);

            Assert.IsFalse(nextGen[1, 1].IsAlive);
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
            var nextGen = _game.NextGeneration(grid);

            Assert.IsTrue(nextGen[1, 2].IsAlive);
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
            var nextGen = _game.NextGeneration(grid);

            Assert.IsFalse(nextGen[1, 0].IsAlive);
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
            var nextGen = _game.NextGeneration(grid);

            assert_eq!(grid, nextGen);
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
            var nextGen = _game.NextGeneration(grid);

            IGrid expected = new Grid(3, 3);
            /*
             * . | O | .
             * . | O | .
             * . | O | .
             */
            expected[1, 0] = new Cell(true);
            expected[1, 1] = new Cell(true);
            expected[1, 2] = new Cell(true);
            assert_eq!(expected, nextGen);
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
            var nextGen = _game.NextGeneration(grid);

            IGrid expected = new Grid(3, 3);
            /*
             * . | . | .
             * O | O | O
             * . | . | .
             */
            expected[0, 1] = new Cell(true);
            expected[1, 1] = new Cell(true);
            expected[2, 1] = new Cell(true);
            assert_eq!(expected, nextGen);
        }
    */
}
