#[cfg(test)]
extern crate mockers;
#[cfg(test)]
use mockers_derive::mocked;

#[cfg_attr(test, mocked)]
trait View {}

struct Presenter {
    view: Box<View>,
}

#[cfg(test)]
mod test {
    use super::*;
}
