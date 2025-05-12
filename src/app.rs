use aspen::prelude::*;

use crate::{todo::Todo, util::*};

pub struct App {
    pub board: ElementPointer<Board<Todo>>,

    pub window_buttons: ElementPointer<WindowButtons>,
    pub resize_handles: ElementPointer<ResizeHandles>,
}

impl App {
    pub fn new() -> App {
        let mut board = Board::new_dotgrid(Affine::IDENTITY, *BACKGROUND0, *BACKGROUND3);
        board.add_child(Todo::new(Point::new(100., 100.)));

        App {
            window_buttons: WindowButtons::new(*BACKGROUND3, *CLOSE, *BACKGROUND4, *FOREGROUND),
            resize_handles: ResizeHandles::new(),
            board,
        }
    }
}

impl Element for App {
    fn update(&mut self, cx: &mut UpdateContext) {
        self.board.update(cx);
        self.window_buttons.update(cx);
        self.resize_handles.update(cx);
    }

    fn layout(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> Size {
        self.board.layout(min, max, cx).position(Point::ZERO, cx);
        self.window_buttons
            .layout(Size::new(0., 0.), max, cx)
            .position(Point::ZERO, cx);
        self.resize_handles
            .layout(min, max, cx)
            .position(Point::ZERO, cx);

        max
    }

    fn draw(&self, cx: &mut DrawContext) {
        self.board.draw(cx);
        self.window_buttons.draw(cx);
        self.resize_handles.draw(cx);
    }
}
