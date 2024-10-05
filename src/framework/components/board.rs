use std::{cell::RefCell, rc::Rc};

use crate::framework::{
    context::{DrawContext, LayoutContext, UpdateContext},
    element::{Element, ElementPointer},
    mouse_region::MouseRegion,
};
use glamour::prelude::*;

pub trait Pinnable: Element {
    fn center(&self) -> Point2;
}

pub struct Board<Child: Pinnable> {
    draw_background: Box<dyn Fn(Point2, Rect, &mut DrawContext)>,
    children: Vec<ElementPointer<Child>>,

    offset: Rc<RefCell<Point2>>,
}

impl<Child: Pinnable> Board<Child> {
    pub fn new(
        offset: Point2,
        draw_background: impl Fn(Point2, Rect, &mut DrawContext) + 'static,
    ) -> ElementPointer<Self> {
        ElementPointer::new(Self {
            draw_background: Box::new(draw_background),
            children: Vec::new(),

            offset: Rc::new(RefCell::new(offset)),
        })
    }

    pub fn add_child(&mut self, child: ElementPointer<Child>) {
        self.children.push(child);
    }

    pub fn offset(&self) -> Point2 {
        *self.offset.borrow()
    }
}

impl<Child: Pinnable> Element for Board<Child> {
    fn update(&mut self, cx: &mut UpdateContext) {
        for child in self.children.iter_mut() {
            child.update(cx);
        }
    }

    fn layout(&mut self, _min: Size2, max: Size2, cx: &mut LayoutContext) -> Size2 {
        let offset = self.offset();
        for child in self.children.iter_mut() {
            let result = child.layout(Size2::ZERO, Size2::INFINITY, cx);
            let position = offset + child.center().to_vector() - result.size().to_vector() / 2.;
            result.position(position, cx);
        }

        max
    }

    fn draw(&self, cx: &mut DrawContext) {
        cx.add_mouse_region(MouseRegion::new(cx.token(), cx.region()).on_drag({
            let offset = self.offset.clone();
            move |_down, cx| {
                let mut offset = offset.borrow_mut();
                *offset += cx.mouse_delta();
                cx.request_redraw();
            }
        }));

        (self.draw_background)(self.offset(), cx.window_rect(), cx);

        for child in self.children.iter() {
            child.draw(cx);
        }
    }
}
