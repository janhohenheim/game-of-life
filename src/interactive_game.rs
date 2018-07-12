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

#[cfg(test)]
mod test {
    use super::*;
    use crate::generation_calculator::GridMock;
    use mockers::Scenario;
    const CHANGES: [Change; 3] = [
        Change {
            x: 20,
            y: 30,
            is_alive: false,
        },
        Change {
            x: 123,
            y: 432,
            is_alive: true,
        },
        Change {
            x: 223,
            y: 42,
            is_alive: true,
        },
    ];

    fn expect_changes_on_grid(scenario: &Scenario, grid: &GridMock) {
        for change in &CHANGES {
            if change.is_alive {
                scenario.expect(grid.set_alive_at_call(change.x, change.y).and_return(()))
            } else {
                scenario.expect(grid.set_dead_at_call(change.x, change.y).and_return(()))
            }
        }
    }
}
