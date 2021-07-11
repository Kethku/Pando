mod dots;
mod canvas;
mod draggable;
mod pinboard;
mod todo;
mod utils;

use druid::{
    AppLauncher, LocalizedString, WindowDesc, Point, WidgetExt
};
use druid::im::vector;

use pinboard::PinBoard;
use draggable::DragController;
use todo::{todo, TodoItem};

fn main() {
    let window = WindowDesc::new(|| {
        PinBoard::new(|position| TodoItem::new(position), || todo()).controller(DragController::new(true))
    }).title(LocalizedString::new("Pando"));
    AppLauncher::with_window(window)
        .launch((Point::ZERO, vector![]))
        .expect("Launch Failed");
}
