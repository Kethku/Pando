use druid::Point;
use druid::im::{Vector as ImVector, HashMap as ImHashMap};

use crate::AppData;
use crate::widgets::{
    canvas::Identifiable,
    todo::TodoItem,
};

pub type V2AppData = (Point, ImHashMap<u64, TodoItem>);
