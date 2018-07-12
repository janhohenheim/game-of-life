use crate::generation_calculator::{Change, Grid};

#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

#[cfg_attr(test, mocked)]
pub trait GenerationCalculator {
    fn next_generation(&self, grid: &Grid) -> Vec<Change>;
}

#[cfg_attr(test, mocked)]
pub trait InteractiveGame {
    fn accept_changes(&mut self, changes: &[Change]);
    fn next_generation(&mut self) -> Vec<Change>;
}

pub struct InteractiveGameImpl {
    grid: Box<dyn Grid>,
    generation_calculator: Box<dyn GenerationCalculator>,
}
impl InteractiveGameImpl {
    pub fn new(grid: Box<dyn Grid>, generation_calculator: Box<dyn GenerationCalculator>) -> Self {
        InteractiveGameImpl {
            grid,
            generation_calculator,
        }
    }
}

impl InteractiveGame for InteractiveGameImpl {
    fn accept_changes(&mut self, changes: &[Change]) {}

    fn next_generation(&mut self) -> Vec<Change> {
        Vec::new()
    }
}
