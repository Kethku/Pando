use std::{cell::RefCell, rc::Rc};

use glamour::prelude::*;
use vide::*;

use crate::{
    framework::{
        context::{Context, DrawContext},
        mouse_region::MouseRegion,
        token::Token,
    },
    util::*,
};

pub struct Background {
    token: Token,
    offset: Rc<RefCell<Point2>>,
    moved: Rc<RefCell<bool>>,
}

impl Background {
    pub fn new(offset: Point2) -> Self {
        Self {
            token: Token::new(),
            offset: Rc::new(RefCell::new(offset)),
            moved: Rc::new(RefCell::new(false)),
        }
    }

    pub fn offset(&self) -> Point2 {
        *self.offset.borrow()
    }

    pub fn update(&mut self, _cx: &Context) -> bool {
        self.moved.take()
    }

    pub fn draw(&self, cx: &mut DrawContext) {
        let mut layer = Layer::new();
        layer.add_clear(*BACKGROUND0);

        let size = cx.window_size();
        cx.add_mouse_region(
            MouseRegion::new(self.token, Rect::new(point2!(0., 0.), size)).on_drag({
                let offset = self.offset.clone();
                let moved = self.moved.clone();
                move |_down, cx| {
                    *offset.borrow_mut() += cx.mouse_delta();
                    *moved.borrow_mut() = true;
                }
            }),
        );

        let offset = self.offset.borrow_mut();
        let mut x = offset.x % 50.;
        loop {
            let mut y = offset.y % 50.;
            loop {
                layer.add_quad(
                    Quad::new(Rect::new(point2!(x, y), size2!(2., 2.)), *BACKGROUND5)
                        .with_corner_radius(1.),
                );
                y += 50.;
                if y > size.height {
                    break;
                }
            }
            x += 50.;
            if x > size.width {
                break;
            }
        }

        cx.add_layer(layer);
    }
}
