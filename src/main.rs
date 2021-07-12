#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod widgets;
mod controllers;
mod save;

use druid::{
    AppLauncher, LocalizedString, Point, WindowDesc, WidgetExt
};
use druid::im::vector;

use widgets::{
    pinboard::PinBoard,
    todo::{todo, TodoItem},
    dot_grid::dot_grid
};
use controllers::*;
use save::read_or;

fn main() {
    let window = WindowDesc::new(
        || PinBoard::new(|position| TodoItem::new(position), || todo())
            .background(dot_grid())
            .draggable(true)
            .undo_root()
    ).title(LocalizedString::new("Pando"));
    AppLauncher::with_window(window)
        .launch(read_or((Point::ZERO, vector![])))
        .expect("Launch Failed");
}
