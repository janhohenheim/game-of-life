pub trait Grid {
    fn new(width: usize, height: usize) -> Self;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn is_alive_at(&self, x: usize, y: usize) -> bool;
    fn set_alive_at(&mut self, x: usize, y: usize);
    fn set_dead_at(&mut self, x: usize, y: usize);
}

pub struct OneDimensionalBoolGrid {
    grid: Vec<bool>,
    width: usize,
    height: usize,
}

impl Grid for OneDimensionalBoolGrid {
    fn new(width: usize, height: usize) -> Self {
        OneDimensionalBoolGrid {
            grid: Vec::new(),
            width,
            height,
        }
    }
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn is_alive_at(&self, x: usize, y: usize) -> bool {
        unimplemented!()
    }
    fn set_alive_at(&mut self, x: usize, y: usize) {
        unimplemented!()
    }
    fn set_dead_at(&mut self, x: usize, y: usize) {
        unimplemented!()
    }
}

#[cfg(test)]
mod one_dimensional_bool_grid_test {
    use grid::{Grid, OneDimensionalBoolGrid};

    #[test]
    fn grid_has_correct_width() {
        let grid = OneDimensionalBoolGrid::new(10, 5);
        assert_eq!(10, grid.width());
    }

    #[test]
    fn grid_has_correct_height() {
        let grid = OneDimensionalBoolGrid::new(10, 5);
        assert_eq!(5, grid.height());
    }

    #[test]
    fn grid_inits_dead() {
        let grid = OneDimensionalBoolGrid::new(10, 10);
        for x in 0..grid.width() {
            for y in 0..grid.height() {
                assert_eq!(false, grid.is_alive_at(x, y))
            }
        }
    }

    #[test]
    fn grid_sets_alive() {
        let mut grid = OneDimensionalBoolGrid::new(10, 10);
        grid.set_alive_at(2, 3);
        assert_eq!(true, grid.is_alive_at(2, 3));
    }

    #[test]
    fn grid_sets_dead() {
        let mut grid = OneDimensionalBoolGrid::new(10, 10);
        grid.set_alive_at(2, 3);
        grid.set_dead_at(2, 3);
        assert_eq!(false, grid.is_alive_at(2, 3));
    }
}
