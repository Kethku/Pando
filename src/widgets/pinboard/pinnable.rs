use druid::Point;
use druid::im::Vector;

use crate::controllers::draggable::Positioned;

pub trait Pinnable : Positioned {
    fn new(position: Point, id: u64) -> Self;

    fn get_id(&self) -> u64;

    fn get_dependencies(&self) -> Vector<u64> {
        // For default, assume no dependencies
        Vector::new()
    }

    fn toggle_dependency(&mut self, _dependency_id: &u64) {
        // For default, don't do anything
    }
}
