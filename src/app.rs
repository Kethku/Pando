use std::collections::HashSet;

use aspen::prelude::*;
use ordered_float::OrderedFloat;

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

            if bounds.is_zero_area() {
                return;
            }

            let mut spacing = 2048.;
            let mut filled_spaces = HashSet::new();
            loop {
                spacing = spacing / 2.;
                let radius = spacing / 75.;
                let actual_radius = (cx.current_transform().unskewed_scale() * radius).length();
                if actual_radius < 0.75 {
                    break;
                } else if actual_radius > 4. {
                    continue;
                }

                let mut x = bounds.min_x() - bounds.min_x().rem_euclid(spacing);
                loop {
                    let mut y = bounds.min_y() - bounds.min_y().rem_euclid(spacing);
                    loop {
                        cx.set_fill_brush(Brush::Solid(
                            BACKGROUND0.mix(&BACKGROUND5, (actual_radius - 0.75) * 4.),
                        ));
                        let point = (OrderedFloat(x), OrderedFloat(y));
                        if !filled_spaces.contains(&point) {
                            cx.fill(&Circle::new(Point::new(x, y).snap(), radius));
                            filled_spaces.insert(point);
                        }

                        y += spacing;
                        if y > bounds.max_y() {
                            break;
                        }
                    }
                    x += spacing;
                    if x > bounds.max_x() {
                        break;
                    }
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
