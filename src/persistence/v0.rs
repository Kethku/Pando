use druid::Point;
use druid::im::{Vector as ImVector, HashSet};
use serde::{Serialize, Deserialize};

use super::v1::V1AppData;
use crate::widgets::{
    flow::FlowDependency,
    todo::{TodoItem, TodoStatus},
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V0TodoItem {
    id: u64,
    position: Point,
    name: String,
    status: TodoStatus,
    dependencies: ImVector<u64>,
    #[serde(default)]
    highlighted: bool
}

impl V0TodoItem {
    fn upgrade(self) -> TodoItem {
        let mut dependencies = HashSet::new();
        for dependency in self.dependencies {
            dependencies.insert(FlowDependency {
                local_link_index: 0,
                dependency_id: dependency,
                dependency_link_index: 1
            });
        }

        TodoItem {
            id: self.id,
            position: self.position,
            name: self.name,
            status: self.status,
            dependencies,
            highlighted: self.highlighted
        }
    }
}

pub type V0AppData = (Point, ImVector<V0TodoItem>);
pub fn upgrade_v0_to_v1(v0_state: V0AppData) -> V1AppData {
    let (offset, todo_data) = v0_state;
    (offset, todo_data.into_iter().map(V0TodoItem::upgrade).collect())
}
