use super::js::HTMLCanvasElement;
use crate::coordinate_translator::ViewInfo;

impl ViewInfo for HTMLCanvasElement {
    fn x_offset(&self) -> u32 {
        self.get_bounding_client_rect().left() as u32
    }
    fn y_offset(&self) -> u32 {
        self.get_bounding_client_rect().top() as u32
    }
}
