use druid::Point;
use druid::im::{HashMap as ImHashMap, HashSet as ImHashSet};

use super::v1::{V1AppData, V1FlowDependency, V1TodoItem};
use crate::AppData;
use crate::widgets::{
    flow::FlowDependency,
    canvas::Identifiable,
    todo::TodoItem,
};

pub type V2AppData = (Point, ImHashMap<u64, V1TodoItem>);

pub fn upgrade_v2_to_current(v2_state: V2AppData) -> AppData {
    let (position, v2_todos) = v2_state;

    let mut dependencies = ImHashSet::new();
    let mut todos = ImHashMap::new();

    for v2_todo in v2_todos.values() {
        for dependency in v2_todo.dependencies.iter() {
            dependencies.insert(
                FlowDependency::try_new(
                    (v2_todo.id, dependency.local_link_index),
                    (dependency.dependency_id, dependency.dependency_link_index)).unwrap());
        }

        todos.insert(v2_todo.id, TodoItem {
            id: v2_todo.id,
            position: v2_todo.position,
            name: v2_todo.name,
            status: v2_todo.status,
            highlighted: v2_todo.highlighted
        });
    }

    AppData {
        position,
        dependencies,
        todos
    }
}
