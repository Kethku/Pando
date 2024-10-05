use aspen::{
    components::{board::Board, resize_handles::ResizeHandles, window_buttons::WindowButtons},
    context::{DrawContext, LayoutContext, UpdateContext},
    element::{Element, ElementPointer},
    vide::prelude::*,
};

use crate::{todo::Todo, util::*};

pub struct App {
    pub board: ElementPointer<Board<Todo>>,

    pub window_buttons: ElementPointer<WindowButtons>,
    pub resize_handles: ElementPointer<ResizeHandles>,
}

impl App {
    pub fn new() -> App {
        let mut board = Board::new(point2!(0., 0.), |offset, region, cx| {
            cx.update_layer(|_, layer| {
                layer.add_quad(
                    Quad::new(Rect::new(point2!(0., 0.), region.size), *BACKGROUND0)
                        .with_corner_radius(1.),
                )
            });
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
        });
        board.add_child(Todo::new(point2!(100., 100.)));

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

    fn layout(&mut self, min: Size2, max: Size2, cx: &mut LayoutContext) -> Size2 {
        self.board.layout(min, max, cx).position(Point2::ZERO, cx);
        self.window_buttons
            .layout(size2!(0., 0.), max, cx)
            .position(Point2::ZERO, cx);
        self.resize_handles
            .layout(min, max, cx)
            .position(Point2::ZERO, cx);

        max
    }

    fn draw(&self, cx: &mut DrawContext) {
        self.board.draw(cx);
        self.window_buttons.draw(cx);
        self.resize_handles.draw(cx);
    }
}
