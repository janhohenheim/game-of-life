use crate::generation_calculator::Change;
use crate::grid::Position;
use crate::interactive_game::Presenter;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

// This trait seems to break mockers
// #[cfg_attr(test, mocked)]
pub trait View {
    fn init_board(&mut self, width: u32, height: u32);
}

pub struct CanvasPresenter {
    view: Box<View>,
}
impl CanvasPresenter {
    pub fn new(view: Box<View>) -> Self {
        CanvasPresenter { view }
    }
}
impl Presenter for CanvasPresenter {
    fn init_board(&mut self, width: u32, height: u32, alive_cells: &[Position]) {}
    fn present_changes(&mut self, changes: &[Change]) {}
}

#[cfg(test)]
mod test {
    use super::*;
}
