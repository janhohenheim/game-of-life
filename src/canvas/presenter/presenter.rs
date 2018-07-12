#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

use crate::controller::{Controller, Presenter};
use crate::generation_calculator::Change;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type ObservablePresenter = Weak<RefCell<Presenter>>;

#[cfg_attr(test, mocked)]
trait View {
    fn register_presenter(&mut self, presenter: ObservablePresenter);
}

struct CanvasPresenter {
    view: Box<View>,
}
impl CanvasPresenter {
    pub fn new(view: Box<View>) -> Rc<RefCell<Self>> {
        let presenter = Rc::new(RefCell::new(CanvasPresenter { view }));
        let second = Rc::downgrade(&presenter);
        {
            let presenter_guard = presenter.borrow_mut();
            presenter_guard.view.borrow_mut().register_presenter(second);
        }
        presenter
    }
}
impl Presenter for CanvasPresenter {
    fn register_controller(&mut self, controller: Weak<RefCell<Controller>>) {}
    fn init_board(&mut self, width: u32, height: u32) {}
    fn present_changes(&mut self, changes: &[Change]) {}
}

#[cfg(test)]
mod test {
    use super::*;
}
