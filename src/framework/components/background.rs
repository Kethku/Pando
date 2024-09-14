use std::{cell::RefCell, rc::Rc};

use glamour::prelude::*;
use vide::*;

use crate::{
    framework::{context::DrawContext, mouse_region::MouseRegion, token::Token},
    util::*,
};

pub struct Background {
    token: Token,
    offset: Rc<RefCell<Point2>>,
}

impl Background {
    pub fn new(offset: Point2) -> Self {
        Self {
            token: Token::new(),
            offset: Rc::new(RefCell::new(offset)),
        }
    }

    pub fn offset(&self) -> Point2 {
        *self.offset.borrow()
    }

    pub fn draw(&self, cx: &mut DrawContext) {
        let mut layer = Layer::new();
        layer.add_clear(*BACKGROUND0);

        let size = cx.window_size();
        cx.add_mouse_region(
            MouseRegion::new(self.token, Rect::new(point2!(0., 0.), size)).on_drag({
                let offset = self.offset.clone();
                move |_down, cx| {
                    let mut offset = offset.borrow_mut();
                    *offset += cx.mouse_delta();
                    cx.request_redraw();
                }
            }),
        );

        let offset = self.offset();
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
