use crate::grid::Position;
#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

#[cfg_attr(test, mocked)]
pub trait CoordinateTranslator {
    fn to_local(&self, position: &Position) -> Option<Position>;
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub origin: Position,
    pub width: u32,
    pub height: u32,
}

#[cfg_attr(test, mocked)]
pub trait ViewInfo {
    fn view_rect(&self) -> Rect;
    fn client_rect(&self) -> Rect;
}

#[allow(dead_code)]
pub struct ScalingCoordinateTranslator {
    view_info: Box<ViewInfo>,
}

impl ScalingCoordinateTranslator {
    #[allow(dead_code)]
    pub fn new(view_info: Box<dyn ViewInfo>) -> Self {
        ScalingCoordinateTranslator { view_info }
    }
}

impl CoordinateTranslator for ScalingCoordinateTranslator {
    fn to_local(&self, position: &Position) -> Option<Position> {
        let view_rect = self.view_info.view_rect();
        let client_rect = self.view_info.client_rect();
        let x_scale = client_rect.width / view_rect.width;
        let y_scale = client_rect.height / view_rect.height;
        let local_x: i32 = (position.x as i32 - view_rect.origin.x as i32) * x_scale as i32;
        let local_y: i32 = (position.y as i32 - view_rect.origin.y as i32) * y_scale as i32;
        if local_x < 0
            || local_y < 0
            || local_x > client_rect.width as i32
            || local_y > client_rect.height as i32
        {
            None
        } else {
            Some(Position {
                x: local_x as u32,
                y: local_y as u32,
            })
        }
    }
}

pub struct IdentityCoordinateTranslator;

impl CoordinateTranslator for IdentityCoordinateTranslator {
    fn to_local(&self, position: &Position) -> Option<Position> {
        Some(*position)
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
        scenario.expect(view_info.view_rect_call().and_return(Rect {
            width: 1000,
            height: 1000,
            origin: Position { x: 771, y: 0 },
        }));
        scenario.expect(view_info.client_rect_call().and_return(Rect {
            width: 1000,
            height: 1000,
            origin: Position { x: 0, y: 0 },
        }));
        (scenario, view_info)
    }

    #[test]
    fn converts_to_local() {
        let (_scenario, view_info) = create_mock();
        let coordinate_translator = ScalingCoordinateTranslator::new(Box::new(view_info));
        let global = Position { x: 772, y: 7 };
        let local = coordinate_translator.to_local(&global);
        let expected = Some(Position { x: 1, y: 7 });
        assert_eq!(expected, local);
    }

    #[test]
    fn converts_to_out_of_bounds_local() {
        let (_scenario, view_info) = create_mock();
        let coordinate_translator = ScalingCoordinateTranslator::new(Box::new(view_info));
        let global = Position { x: 3, y: 3 };
        let local = coordinate_translator.to_local(&global);
        assert_eq!(None, local);
    }

    #[test]
    fn entity_returns_itself() {
        let coordinate_translator = IdentityCoordinateTranslator;
        let expected = Position { x: 10, y: 20 };
        let actual = coordinate_translator.to_local(&expected);
        assert_eq!(Some(expected), actual);
    }
}
