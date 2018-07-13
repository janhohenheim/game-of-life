use std::ops::Deref;

#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[cfg_attr(test, mocked)]
pub trait Grid {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn is_alive_at(&self, position: Position) -> bool;
    fn set_alive_at(&mut self, position: Position);
    fn set_dead_at(&mut self, position: Position);
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
    fn translate_coordinates_to_index(&self, position: Position) -> usize {
        (self.width() * position.y + position.x) as usize
    }
}

impl Grid for GridImpl {
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn is_alive_at(&self, position: Position) -> bool {
        let index = self.translate_coordinates_to_index(position);
        self.grid[index]
    }
    fn set_alive_at(&mut self, position: Position) {
        let index = self.translate_coordinates_to_index(position);
        self.grid[index] = true;
    }
    fn set_dead_at(&mut self, position: Position) {
        let index = self.translate_coordinates_to_index(position);
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
        let position = Position { x: 2, y: 3 };
        grid.set_alive_at(position);
        assert_eq!(true, grid.is_alive_at(position));
    }

    #[test]
    fn grid_sets_dead() {
        let mut grid = GridImpl::new(10, 10);
        let position = Position { x: 2, y: 3 };
        grid.set_alive_at(position);
        grid.set_dead_at(position);
        assert_eq!(false, grid.is_alive_at(position));
    }
}
