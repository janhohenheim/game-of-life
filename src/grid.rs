use std::ops::Deref;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grid {
    grid: Vec<bool>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            grid: vec![false; width * height],
            width,
            height,
        }
    }

    fn translate_coordinates_to_index(&self, x: usize, y: usize) -> usize {
        self.width() * y + x
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

impl Deref for Grid {
    type Target = [bool];

    fn deref(&self) -> &Self::Target {
        self.grid.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_has_correct_width() {
        let grid = Grid::new(10, 5);
        assert_eq!(10, grid.width());
    }

    #[test]
    fn grid_has_correct_height() {
        let grid = Grid::new(10, 5);
        assert_eq!(5, grid.height());
    }

    #[test]
    fn grid_inits_dead() {
        let grid = Grid::new(10, 10);
        for cell_is_alive in grid.iter() {
            assert_eq!(false, *cell_is_alive)
        }
    }

    #[test]
    fn grid_sets_alive() {
        let mut grid = Grid::new(10, 10);
        grid.set_alive_at(2, 3);
        assert_eq!(true, grid.is_alive_at(2, 3));
    }

    #[test]
    fn grid_sets_dead() {
        let mut grid = Grid::new(10, 10);
        grid.set_alive_at(2, 3);
        grid.set_dead_at(2, 3);
        assert_eq!(false, grid.is_alive_at(2, 3));
    }
}
