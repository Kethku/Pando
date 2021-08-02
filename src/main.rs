#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod widgets;
mod controllers;
mod persistence;

use druid::{
    AppLauncher, Data, LocalizedString, Point, WindowDesc, WidgetExt
};
use druid::im::{HashMap as ImHashMap, HashSet as ImHashSet};
use serde::{Serialize, Deserialize};

use widgets::{
    canvas::Positioned,
    flow::{Flow, FlowDependency},
    todo::{todo, TodoItem},
    dot_grid::dot_grid
};
use controllers::*;
use persistence::read_or;

#[derive(Clone, Data, Debug, Serialize, Deserialize)]
pub struct AppData {
    position: Point,
    dependencies: ImHashSet<FlowDependency>,
    todos: ImHashMap<u64, TodoItem>,
}

impl AppData {
    fn new() -> Self {
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

fn main() {
    let window = WindowDesc::new(|| Flow::new(|| todo())
            .background(dot_grid())
            .draggable(true)
            .undo_root()
    ).title(LocalizedString::new("Pando"));
    AppLauncher::with_window(window)
        .launch(read_or(AppData::new()))
        .expect("Launch Failed");
}
