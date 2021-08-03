use druid::Point;
use druid::im::{Vector as ImVector, HashSet};
use serde::{Serialize, Deserialize};

use super::v1::{V1AppData, V1FlowDependency, V1TodoItem};
use crate::widgets::{
    flow::FlowDependency,
    todo::TodoStatus,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct V0TodoItem {
    pub id: u64,
    pub position: Point,
    pub name: String,
    pub status: TodoStatus,
    pub dependencies: ImVector<u64>,
    #[serde(default)]
    pub highlighted: bool
}

impl V0TodoItem {
    fn upgrade(self) -> V1TodoItem {
        let mut dependencies = HashSet::new();
        for dependency in self.dependencies {
            dependencies.insert(V1FlowDependency {
                local_link_index: 0,
                dependency_id: dependency,
                dependency_link_index: 1
            });
        }

        V1TodoItem {
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
