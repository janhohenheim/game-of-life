use std::ops::Deref;

#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

#[cfg_attr(test, mocked)]
pub trait Grid {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn is_alive_at(&self, x: u32, y: u32) -> bool;
    fn set_alive_at(&mut self, x: u32, y: u32);
    fn set_dead_at(&mut self, x: u32, y: u32);
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GridImpl {
    grid: Vec<bool>,
    width: u32,
    height: u32,
}

impl GridImpl {
    pub fn new(width: u32, height: u32) -> Self {
        GridImpl {
            grid: vec![false; (width * height) as usize],
            width,
            height,
        }
    }
    fn translate_coordinates_to_index(&self, x: u32, y: u32) -> usize {
        (self.width() * y + x) as usize
    }
}

impl Grid for GridImpl {
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn is_alive_at(&self, x: u32, y: u32) -> bool {
        let index = self.translate_coordinates_to_index(x, y);
        self.grid[index]
    }
    fn set_alive_at(&mut self, x: u32, y: u32) {
        let index = self.translate_coordinates_to_index(x, y);
        self.grid[index] = true;
    }
    fn set_dead_at(&mut self, x: u32, y: u32) {
        let index = self.translate_coordinates_to_index(x, y);
        self.grid[index] = false;
    }
}

impl Deref for GridImpl {
    type Target = [bool];

    fn deref(&self) -> &Self::Target {
        self.grid.as_slice()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn grid_has_correct_width() {
        let grid = GridImpl::new(10, 5);
        assert_eq!(10, grid.width());
    }

    #[test]
    fn grid_has_correct_height() {
        let grid = GridImpl::new(10, 5);
        assert_eq!(5, grid.height());
    }

    #[test]
    fn grid_inits_dead() {
        let grid = GridImpl::new(10, 10);
        for cell_is_alive in grid.iter() {
            assert_eq!(false, *cell_is_alive)
        }
    }

    #[test]
    fn grid_sets_alive() {
        let mut grid = GridImpl::new(10, 10);
        grid.set_alive_at(2, 3);
        assert_eq!(true, grid.is_alive_at(2, 3));
    }

    #[test]
    fn grid_sets_dead() {
        let mut grid = GridImpl::new(10, 10);
        grid.set_alive_at(2, 3);
        grid.set_dead_at(2, 3);
        assert_eq!(false, grid.is_alive_at(2, 3));
    }
}
