use crate::grid::Position;
#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

#[cfg_attr(test, mocked)]
pub trait CoordinateTranslator {
    fn to_local(&self, position: &Position) -> Position;
    fn to_absolute(&self, position: &Position) -> Position;
}
