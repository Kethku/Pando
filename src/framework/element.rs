use std::ops::{Deref, DerefMut};
use vide::glamour::prelude::*;

use crate::framework::{
    context::{DrawContext, LayoutContext, UpdateContext},
    token::Token,
};

pub trait Element {
    fn update(&mut self, _cx: &mut UpdateContext) {}
    fn layout(&mut self, min: Size2, max: Size2, cx: &mut LayoutContext) -> Size2;
    fn draw(&self, _cx: &mut DrawContext) {}
}

pub struct ElementPointer<E: Element> {
    token: Token,
    element: E,
}

impl<E: Element> ElementPointer<E> {
    pub fn new(element: E) -> Self {
        Self {
            token: Token::new(),
            element,
        }
    }

    pub fn token(&self) -> Token {
        self.token
    }

    pub fn update(&mut self, cx: &mut UpdateContext) {
        let mut child_cx: UpdateContext = cx.child(self.token);
        self.element.update(&mut child_cx);
    }

    #[must_use]
    pub fn layout(&mut self, min: Size2, max: Size2, cx: &mut LayoutContext) -> LayoutResult {
        let mut child_cx: LayoutContext = cx.child(self.token);
        let size = self.element.layout(min, max, &mut child_cx);
        if size.width < min.width
            || size.height < min.height
            || size.width > max.width
            || size.height > max.height
        {
            panic!("Element returned {size:?} which is out of min {min:?} to max {max:?}");
        }
        LayoutResult {
            size,
            token: self.token,
        }
    }

    pub fn draw(&self, cx: &mut DrawContext) {
        let mut child_cx: DrawContext = cx.child(self.token);
        self.element.draw(&mut child_cx);
    }
}

impl<E: Element> Deref for ElementPointer<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl<E: Element> DerefMut for ElementPointer<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.element
    }
}

impl<E: Element> From<E> for ElementPointer<E> {
    fn from(element: E) -> Self {
        Self::new(element)
    }
}

pub struct LayoutResult {
    size: Size2,
    token: Token,
}

impl LayoutResult {
    pub fn size(&self) -> Size2 {
        self.size
    }

    pub fn position(self, position: Point2, cx: &mut LayoutContext) {
        cx.translate_descendants(self.token, position.to_vector());
        cx.add_region(self.token, Rect::new(position, self.size));
    }
}

impl Deref for LayoutResult {
    type Target = Size2;

    fn deref(&self) -> &Self::Target {
        &self.size
    }
}
