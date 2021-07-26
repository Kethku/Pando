use druid::Point;
use druid::im::{Vector as ImVector, HashMap as ImHashMap};

use crate::AppData;
use crate::widgets::{
    canvas::Identifiable,
    todo::TodoItem,
};

pub type V1AppData = (Point, ImVector<TodoItem>);


pub fn upgrade_v1_to_current(v1_state: V1AppData) -> AppData {
    let (offset, todo_data_vector) = v1_state;

    let mut todo_data_map = ImHashMap::new();
    for todo_item in todo_data_vector {
        todo_data_map.insert(todo_item.get_id(), todo_item);
    }

    (offset, todo_data_map)
}
