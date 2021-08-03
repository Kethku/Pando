use druid::Point;
use druid::im::{Vector as ImVector, HashMap as ImHashMap, HashSet as ImHashSet};
use serde::{Serialize, Deserialize};

use super::v2::V2AppData;
use crate::AppData;
use crate::widgets::{
    canvas::Identifiable,
    todo::TodoItem,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct V1FlowDependency {
    pub local_link_index: usize,
    pub dependency_id: u64,
    pub dependency_link_index: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct V1TodoItem {
    pub id: u64,
    pub position: Point,
    pub name: String,
    pub status: TodoStatus,
    pub dependencies: ImHashSet<V1FlowDependency>,
    #[serde(default)]
    pub highlighted: bool
}

pub type V1AppData = (Point, ImVector<V1TodoItem>);

pub fn upgrade_v1_to_v2(v1_state: V1AppData) -> V2AppData {
    let (offset, todo_data_vector) = v1_state;

    let mut todo_data_map = ImHashMap::new();
    for todo_item in todo_data_vector {
        todo_data_map.insert(todo_item.id, todo_item);
    }

    (offset, todo_data_map)
}
