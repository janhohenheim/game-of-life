use generation::{Change, GenerationCalculator};
use grid::Grid;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub trait Presenter {
    fn register_controller(&mut self, controller: Weak<RefCell<Controller>>);
    fn init_board(&mut self, width: u32, height: u32);
    fn present_change(&mut self, change: Change);
}

pub enum PresenterEvent {
    Change(Change),
    NextStep(),
}

pub trait Controller {
    fn start(&mut self);
    fn react_to_event(&mut self, event: PresenterEvent);
}

pub struct ControllerImpl {
    pub presenter: Box<Presenter>,
    pub generation_calculator: Box<GenerationCalculator>,
}

impl ControllerImpl {
    fn new(
        presenter: Box<Presenter>,
        generation_calculator: Box<GenerationCalculator>,
    ) -> Rc<RefCell<Self>> {
        let controller = Rc::new(RefCell::new(ControllerImpl {
            presenter,
            generation_calculator,
        }));
        let second = Rc::downgrade(&controller);
        controller
            .borrow_mut()
            .presenter
            .register_controller(second);
        controller
    }
}

impl Controller for ControllerImpl {
    fn start(&mut self) {}
    fn react_to_event(&mut self, event: PresenterEvent) {}
}

#[cfg(test)]
mod controller_impl_test {
    use super::*;

    #[derive(Default)]
    struct MockPresenter {
        controller: Option<Weak<RefCell<Controller>>>,
        width: u32,
        height: u32,
        changes: Vec<Change>,
    }

    impl MockPresenter {
        fn new() -> Self {
            Default::default()
        }

        fn mock_change(&mut self, change: Change) {
            if let Some(ref controller) = self.controller {
                if let Some(ref controller) = controller.upgrade() {
                    let event = PresenterEvent::Change(change);
                    let mut controller = controller.borrow_mut();
                    (*controller).react_to_event(event);
                }
            }
        }

        fn mock_next_step(&mut self) {
            if let Some(ref controller) = self.controller {
                if let Some(ref controller) = controller.upgrade() {
                    let event = PresenterEvent::NextStep();
                    let mut controller = controller.borrow_mut();
                    (*controller).react_to_event(event);
                }
            }
        }
    }

    impl Presenter for MockPresenter {
        fn register_controller(&mut self, controller: Weak<RefCell<Controller>>) {
            self.controller = Some(controller);
        }

        fn init_board(&mut self, width: u32, height: u32) {
            self.width = width;
            self.height = height;
        }

        fn present_change(&mut self, change: Change) {
            self.changes.push(change);
        }
    }

    struct MockGenerationCalculator;
    impl MockGenerationCalculator {
        fn new() -> Self {
            MockGenerationCalculator {}
        }
    }
    impl GenerationCalculator for MockGenerationCalculator {
        fn next_generation(&self, grid: &Box<Grid>) -> Vec<Change> {
            vec![
                Change {
                    x: 2,
                    y: 2,
                    is_alive: true,
                },
            ]
        }
    }

    fn create_controller() -> Rc<RefCell<Controller>> {
        let presenter = Box::new(MockPresenter::new());
        let generation_calculator = Box::new(MockGenerationCalculator::new());
        ControllerImpl::new(presenter, generation_calculator)
    }

    fn test_inits_board() {
        let controller = create_controller();
        controller.borrow_mut().start();
    }
}
