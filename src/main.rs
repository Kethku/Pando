mod widgets;
mod controllers;
mod save;

use druid::{
    AppLauncher, LocalizedString, Point, WindowDesc
};
use druid::im::vector;

use widgets::{
    pinboard::PinBoard,
    todo::{todo, TodoItem}
};
use controllers::*;

fn main() {
    let window = WindowDesc::new(
        PinBoard::new(|position| TodoItem::new(position), || todo()).draggable(true).undo_root()
    ).title(LocalizedString::new("Pando"));
    AppLauncher::with_window(window)
        .launch((Point::ZERO, vector![]))
        .expect("Launch Failed");
}
