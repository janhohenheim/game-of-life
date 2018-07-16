use super::js;
use crate::canvas::presenter::{CanvasView, CanvasViewModel, Line, Square};

pub struct CanvasViewImpl {
    context: js::CanvasRenderingContext2D,
}

impl CanvasViewImpl {
    pub fn new(context: js::CanvasRenderingContext2D) -> Self {
        CanvasViewImpl { context }
    }
    fn draw_line(&mut self, line: &Line) {
        self.context.set_fill_style(&line.colour);
        self.context.begin_path();
        self.context.move_to(line.from.x, line.from.y);
        self.context.line_to(line.to.x, line.to.y);
        self.context.stroke();
    }
    fn draw_square(&mut self, square: &Square) {
        self.context.set_fill_style(&square.colour);
        self.context.fill_rect(
            square.origin.x,
            square.origin.y,
            square.width,
            square.height,
        );
    }
}

impl CanvasView for CanvasViewImpl {
    fn init_board(&mut self, view_model: &CanvasViewModel) {
        self.draw_view_model(view_model);
    }
    fn draw_view_model(&mut self, view_model: &CanvasViewModel) {
        for line in &view_model.lines {
            self.draw_line(line);
        }
        for square in &view_model.squares {
            self.draw_square(square);
        }
    }
}

#[cfg(test)]
mod test {}
