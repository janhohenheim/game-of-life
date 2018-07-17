use crate::canvas::view::js;
use crate::coordinate_translator::CoordinateTranslator;
use crate::grid::Position;

#[derive(Debug, Clone)]
struct CanvasCoordinateTranslator {
    x_offset: u32,
    y_offset: u32,
}
impl CanvasCoordinateTranslator {
    fn new(canvas: &js::HTMLCanvasElement) -> Self {
        unimplemented!()
    }
}

impl CoordinateTranslator for CanvasCoordinateTranslator {
    fn to_local(&self, position: &Position) -> Position {
        unimplemented!()
    }
    fn to_absolute(&self, position: &Position) -> Position {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn foo() {}
}
