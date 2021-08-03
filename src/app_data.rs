use druid::{Data, Point};
use druid::im::{HashMap as ImHashMap, HashSet as ImHashSet};
use serde::{Serialize, Deserialize};

use crate::widgets::{
    canvas::{Positioned, CanvasData},
    pin_board::PinBoardData,
    flow::{FlowData, FlowDependency},
    todo::TodoItem,
};

#[derive(Clone, Data, Debug, Serialize, Deserialize)]
pub struct AppData {
    pub position: Point,
    pub dependencies: ImHashSet<FlowDependency>,
    pub todos: ImHashMap<u64, TodoItem>,
}

impl AppData {
    pub fn new() -> Self {
        AppData {
            position: Point::ZERO,
            dependencies: ImHashSet::new(),
            todos: ImHashMap::new()
        }
    }
}

impl Positioned for AppData {
    fn get_position(&self) -> Point {
        self.position
    }

    fn set_position(&mut self, new_position: Point) {
        self.position = new_position;
    }
}

impl CanvasData<TodoItem> for AppData {
    fn children(&self) -> Box<dyn Iterator<Item=(&u64, &TodoItem)> + '_> {
        Box::new(self.todos.iter())
    }

    fn children_mut(&mut self) -> Box<dyn Iterator<Item=(&u64, &mut TodoItem)> + '_> {
        Box::new(self.todos.iter_mut())
    }

    fn get_child(&self, id: &u64) -> Option<&TodoItem> {
        self.todos.get(id)
    }

    fn get_child_mut(&mut self, id: &u64) -> Option<&mut TodoItem> {
        self.todos.get_mut(id)
    }
}

impl PinBoardData<TodoItem> for AppData {
    fn add_pin(&mut self, new_pin: TodoItem) {
        self.todos.insert(new_pin.id, new_pin);
    }

    fn remove_pin(&mut self, pin_id: &u64) {
        self.todos.remove(pin_id);
    }

    fn pins(&self) -> Box<dyn Iterator<Item=&TodoItem> + '_> {
        Box::new(self.todos.values())
    }
}

impl FlowData for AppData {
    fn dependencies(&self) -> Box<dyn Iterator<Item=&FlowDependency> + '_> {
        Box::new(self.dependencies.iter())
    }

    fn contains_dependency(&self, dependency: &FlowDependency) -> bool {
        self.dependencies.contains(dependency)
    }

    fn add_dependency(&mut self, dependency: FlowDependency) {
        self.dependencies.insert(dependency);
    }

    fn remove_dependency(&mut self, dependency: &FlowDependency) {
        self.dependencies.remove(dependency);
    }
}
