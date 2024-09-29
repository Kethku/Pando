use glamour::prelude::*;

use vide::prelude::*;

use crate::{
    framework::{
        components::{board::Board, resize_handles::ResizeHandles, window_buttons::WindowButtons},
        context::{DrawContext, LayoutContext, UpdateContext},
        element::{Element, ElementPointer},
    },
    todo::Todo,
    util::*,
};

pub struct App {
    pub board: ElementPointer<Board<Todo>>,

    pub window_buttons: ElementPointer<WindowButtons>,
    pub resize_handles: ElementPointer<ResizeHandles>,
}

impl App {
    pub fn new() -> App {
        App {
            window_buttons: WindowButtons::new(),
            resize_handles: ResizeHandles::new(),
            board: Board::new(point2!(0., 0.), |offset, region, cx| {
                let mut x = offset.x % 50.;
                loop {
                    let mut y = offset.y % 50.;
                    loop {
                        cx.update_layer(|_, layer| {
                            layer.add_quad(
                                Quad::new(Rect::new(point2!(x, y), size2!(2., 2.)), *BACKGROUND5)
                                    .with_corner_radius(1.),
                            )
                        });
                        y += 50.;
                        if y > region.size.height {
                            break;
                        }
                    }
                    x += 50.;
                    if x > region.size.width {
                        break;
                    }
                }
            }),
        }
    }
}

impl Element for App {
    fn update(&mut self, cx: &mut UpdateContext) {
        self.board.update(cx);
        self.window_buttons.update(cx);
        self.resize_handles.update(cx);
    }

    fn layout(&mut self, min: Size2, max: Size2, cx: &mut LayoutContext) -> Size2 {
        self.window_buttons
            .layout(min, max, cx)
            .position(Point2::ZERO, cx);
        self.resize_handles
            .layout(min, max, cx)
            .position(Point2::ZERO, cx);
        self.board.layout(min, max, cx).position(Point2::ZERO, cx);

        max
    }

    fn draw(&self, cx: &mut DrawContext) {
        self.board.draw(cx);
        self.window_buttons.draw(cx);
        self.resize_handles.draw(cx);
    }
}
