use super::js::HTMLCanvasElement;
use crate::coordinate_translator::{Rect, ViewInfo};
use crate::grid::Position;

impl ViewInfo for HTMLCanvasElement {
    fn view_rect(&self) -> Rect {
        let bounding_rect = self.get_bounding_client_rect();
        Rect {
            width: bounding_rect.width() as u32,
            height: bounding_rect.height() as u32,
            origin: Position {
                x: bounding_rect.left() as u32,
                y: bounding_rect.top() as u32,
            },
        }
    }
    fn client_rect(&self) -> Rect {
        Rect {
            width: self.width() as u32,
            height: self.height() as u32,
            origin: Position { x: 0, y: 0 },
        }
    }
}
