use crate::grid::Position;
#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

#[cfg_attr(test, mocked)]
pub trait CoordinateTranslator {
    fn to_local(&self, position: &Position) -> Option<Position>;
    fn to_global(&self, position: &Position) -> Position;
}

#[cfg_attr(test, mocked)]
pub trait ViewInfo {
    fn x_offset(&self) -> u32;
    fn y_offset(&self) -> u32;
}

struct CoordinateTranslatorImpl {
    view_info: Box<ViewInfo>,
}

impl CoordinateTranslatorImpl {
    pub fn new(view_info: Box<dyn ViewInfo>) -> Self {
        CoordinateTranslatorImpl { view_info }
    }
}

impl CoordinateTranslator for CoordinateTranslatorImpl {
    fn to_local(&self, position: &Position) -> Option<Position> {
        let x_offset = self.view_info.x_offset();
        let y_offset = self.view_info.y_offset();
        if x_offset > position.x || y_offset > position.y {
            None
        } else {
            Some(Position {
                x: position.x - x_offset,
                y: position.y - y_offset,
            })
        }
    }
    fn to_global(&self, position: &Position) -> Position {
        Position {
            x: position.x + self.view_info.x_offset(),
            y: position.y + self.view_info.y_offset(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::grid::Position;
    use mockers::Scenario;

    fn create_mock() -> (Scenario, ViewInfoMock) {
        let scenario = Scenario::new();
        let view_info = scenario.create_mock_for::<ViewInfo>();
        scenario.expect(view_info.x_offset_call().and_return(5));
        scenario.expect(view_info.y_offset_call().and_return(3));
        (scenario, view_info)
    }

    #[test]
    fn converts_to_local() {
        let (_scenario, view_info) = create_mock();
        let coordinate_translator = CoordinateTranslatorImpl::new(Box::new(view_info));
        let global = Position { x: 10, y: 7 };
        let local = coordinate_translator.to_local(&global);
        let expected = Some(Position { x: 5, y: 4 });
        assert_eq!(expected, local);
    }

    #[test]
    fn converts_to_out_of_bounds_local() {
        let (_scenario, view_info) = create_mock();
        let coordinate_translator = CoordinateTranslatorImpl::new(Box::new(view_info));
        let global = Position { x: 3, y: 3 };
        let local = coordinate_translator.to_local(&global);
        assert_eq!(None, local);
    }

    #[test]
    fn converts_to_global() {
        let (_scenario, view_info) = create_mock();
        let coordinate_translator = CoordinateTranslatorImpl::new(Box::new(view_info));
        let local = Position { x: 10, y: 7 };
        let global = coordinate_translator.to_global(&local);
        let expected = Position { x: 15, y: 10 };
        assert_eq!(expected, global);
    }
}
