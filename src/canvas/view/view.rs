use super::js;
use crate::canvas::constant;
use crate::canvas::presenter::{CanvasView, CanvasViewModel};
use crate::grid_info::GridInfo;

struct CanvasViewImpl {
    context: js::CanvasRenderingContext2D,
}

impl CanvasView for CanvasViewImpl {
    fn init_board(&mut self, grid_info: &GridInfo, view_model: &CanvasViewModel) {
        self.context.set_fill_style("aquamarine");
        self.context
            .fill_rect(0, 0, constant::CANVAS_WIDTH, constant::CANVAS_HEIGHT);
        self.context.set_fill_style("white");
        self.context.begin_path();
        self.context.move_to(75, 50);
        self.context.line_to(100, 75);
        self.context.stroke();
        self.context.line_to(100, 25);
        self.context.line_to(200, 25);
        self.context.fill();
    }
    fn draw_view_model(&mut self, view_model: &CanvasViewModel) {}
}

#[cfg(test)]
mod test {}
