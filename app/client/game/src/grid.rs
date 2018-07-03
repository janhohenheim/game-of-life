pub trait Grid {
    fn new(width: usize, height: usize) -> Self;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn is_alive_at(&self, x: usize, y: usize) -> bool;
    fn set_alive_at(&mut self, x: usize, y: usize);
    fn set_dead_at(&mut self, x: usize, y: usize);
}

#[derive(Debug, Eq, PartialEq)]
pub struct OneDimensionalBoolGrid {
    grid: Vec<bool>,
    width: usize,
    height: usize,
}

impl OneDimensionalBoolGrid {
    fn translate_coordinates_to_index(&self, x: usize, y: usize) -> usize {
        self.width() * y + x
    }
}

impl Grid for OneDimensionalBoolGrid {
    fn new(width: usize, height: usize) -> Self {
        OneDimensionalBoolGrid {
            grid: vec![false; width * height],
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
        let index = self.translate_coordinates_to_index(x, y);
        self.grid[index]
    }
    fn set_alive_at(&mut self, x: usize, y: usize) {
        let index = self.translate_coordinates_to_index(x, y);
        self.grid[index] = true;
    }
    fn set_dead_at(&mut self, x: usize, y: usize) {
        let index = self.translate_coordinates_to_index(x, y);
        self.grid[index] = false;
    }
}

#[cfg(test)]
mod one_dimensional_bool_grid_test {
    use super::{Grid, OneDimensionalBoolGrid};

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
