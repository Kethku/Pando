use aspen::prelude::*;

use crate::{todo::Todo, util::*};

pub struct App {
    pub board: ElementPointer<Board<Todo>>,

    pub window_buttons: ElementPointer<WindowButtons>,
    pub resize_handles: ElementPointer<ResizeHandles>,
}

impl App {
    pub fn new() -> App {
        let mut board = Board::new(Affine::IDENTITY, |bounds, cx| {
            cx.set_fill_brush(Brush::Solid(*BACKGROUND0));
            cx.fill(&bounds);

            let mut x = bounds.min_x() - bounds.min_x().rem_euclid(50.);
            loop {
                let mut y = bounds.min_y() - bounds.min_y().rem_euclid(50.);
                loop {
                    cx.set_fill_brush(Brush::Solid(*BACKGROUND5));
                    cx.fill(&Circle::new(Point::new(x, y), 1.));
                    y += 50.;
                    if y > bounds.max_y() {
                        break;
                    }
                }
                x += 50.;
                if x > bounds.max_x() {
                    break;
                }
            }
        });
        board.add_child(Todo::new(Point::new(100., 100.)));

        App {
            window_buttons: WindowButtons::new(*BACKGROUND3, *CLOSE, *FOREGROUND),
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
