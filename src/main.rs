#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod widgets;
mod controllers;
mod persistence;
mod app_data;

use druid::{
    AppLauncher, LocalizedString, WindowDesc, WidgetExt
};

use app_data::AppData;
use controllers::*;
use persistence::read_or;
use widgets::{
    flow::Flow,
    todo::todo,
    dot_grid::dot_grid
};

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
