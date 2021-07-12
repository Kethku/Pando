use druid::im::Vector;

pub trait Pinnable {
    fn get_id(&self) -> String;

    fn get_dependencies(&self) -> Vector<String> {
        // For default, assume no dependencies
        Vector::new()
    }

    fn toggle_dependency(&mut self, _dependency_id: &String) {
        // For default, don't do anything
    }
}
