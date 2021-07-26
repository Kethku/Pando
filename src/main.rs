#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod widgets;
mod controllers;
mod persistence;

use druid::{
    AppLauncher, LocalizedString, Point, WindowDesc, WidgetExt
};
use druid::im::HashMap as ImHashMap;

use widgets::{
    flow::Flow,
    todo::{todo, TodoItem},
    dot_grid::dot_grid
};
use controllers::*;
use persistence::read_or;

pub type AppData = (Point, ImHashMap<u64, TodoItem>);

fn main() {
    let window = WindowDesc::new(|| Flow::new(|| todo())
            .background(dot_grid())
            .draggable(true)
            .undo_root()
    ).title(LocalizedString::new("Pando"));
    AppLauncher::with_window(window)
        .launch(read_or((Point::ZERO, ImHashMap::new())))
        .expect("Launch Failed");
}
