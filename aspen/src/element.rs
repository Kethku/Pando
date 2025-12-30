use std::{
    any::Any,
    ops::{Deref, DerefMut},
};

use vello::kurbo::{Affine, Point, Rect, Size};

use crate::{
    context_stack::{Context, DrawContext, LayoutContext, UpdateContext},
    token::Token,
};

pub trait Element {
    fn update(&mut self, _cx: &mut UpdateContext) {}
    fn layout(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> Size;
    fn draw(&self, _cx: &mut DrawContext) {}
    fn children(&self) -> Vec<Token> {
        Vec::new()
    }
}

impl<E: Element + ?Sized> Element for Box<E> {
    fn update(&mut self, cx: &mut UpdateContext) {
        self.as_mut().update(cx)
    }

    fn layout(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> Size {
        self.as_mut().layout(min, max, cx)
    }

    fn draw(&self, cx: &mut DrawContext) {
        self.as_ref().draw(cx)
    }

    fn children(&self) -> Vec<Token> {
        self.as_ref().children()
    }
}

pub struct ElementPointer<E: Element> {
    token: Token,
    element: E,
}

impl<E: Element> ElementPointer<E> {
    pub fn new(element: E) -> Self {
        Self {
            token: Token::new::<E>(),
            element,
        }
    }

    pub fn map<R: Element>(self, map: impl FnOnce(E) -> R) -> ElementPointer<R> {
        ElementPointer {
            token: self.token,
            element: map(self.element),
        }
    }

    pub fn token(&self) -> Token {
        self.token
    }

    pub fn update<'a>(&mut self, cx: &mut UpdateContext) {
        let children = self.children();
        let mut child_cx = cx.child(self.token, &children);
        self.element.update(&mut child_cx);
    }

    #[must_use]
    pub fn layout<'a>(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> LayoutResult {
        let children = self.children();
        let mut child_cx = cx.child(self.token, &children);
        let size = self.element.layout(min, max, &mut child_cx).clamp(min, max);
        LayoutResult {
            size,
            token: self.token,
        }
    }

    pub fn draw<'a>(&self, cx: &mut DrawContext) {
        let children = self.children();
        let mut child_cx = cx.child(self.token, &children);

        if !child_cx.any_in_progress_mouse_regions() {
            let window_region = Rect::from_origin_size(Point::ZERO, child_cx.window_size);
            let child_region = child_cx
                .current_transform()
                .transform_rect_bbox(child_cx.region());
            if !window_region.overlaps(child_region) {
                return;
            }

            if child_region.area() < 1. {
                return;
            }
        }

        self.element.draw(&mut child_cx);
    }

    // Returns a list of tokens associated with this element. Includes the element's token and all
    // of it's children's tokens.
    pub fn tokens(&self) -> Vec<Token> {
        let mut tokens = vec![self.token];
        tokens.extend(self.element.children());
        tokens
    }

    // Returns the tokens of all the children of this element but not this element's token.
    pub fn children(&self) -> Vec<Token> {
        self.element.children()
    }

    pub fn with_context<'a, Result>(
        &self,
        cx: &Context,
        callback: impl FnOnce(&Context) -> Result,
    ) -> Result {
        let children = self.children();
        let cx = cx.child(self.token, &children);
        callback(&cx)
    }

    pub fn insert_state<State: Any>(self, state: State, cx: &Context) -> ElementPointer<E> {
        self.with_context(cx, |cx| cx.insert_state(state));
        self
    }

    pub fn with_initialized_state<'a, State: Any, Result>(
        &self,
        cx: &Context<'a>,
        callback: impl FnOnce(&mut State) -> Result,
    ) -> Result {
        self.with_context(cx, |cx| cx.with_initialized_state(callback))
    }

    pub fn with_state<'a, State: Any + Default, Result>(
        &self,
        cx: &Context<'a>,
        callback: impl FnOnce(&mut State) -> Result,
    ) -> Result {
        self.with_context(cx, |cx| cx.with_state(callback))
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

#[derive(Debug)]
pub struct LayoutResult {
    size: Size,
    token: Token,
}

impl LayoutResult {
    pub fn size(&self) -> Size {
        self.size
    }

    pub fn position(self, transform: Affine, cx: &mut LayoutContext) -> Size {
        cx.add_region(self.token, transform, self.size);
        self.size
    }
}

impl Deref for LayoutResult {
    type Target = Size;

    fn deref(&self) -> &Self::Target {
        &self.size
    }
}

#[cfg(test)]
mod tests {

    use vello::kurbo::Vec2;

    use super::*;
    use crate::{
        context_stack::LayoutContext,
        test_runner::TestRunner,
    };

    struct TestApp {
        component: ElementPointer<TestNestingComponent>,
    }
    impl TestApp {
        fn new() -> ElementPointer<Self> {
            ElementPointer::new(Self {
                component: TestNestingComponent::new(),
            })
        }
    }
    impl Element for TestApp {
        fn layout(&mut self, _: Size, max: Size, cx: &mut LayoutContext) -> Size {
            self.component
                .layout(Size::new(0., 0.), max, cx)
                .position(Affine::translate(Vec2::new(10., 10.)), cx);

            Size::new(70., 70.)
        }
    }

    struct TestNestingComponent {
        child: ElementPointer<TestComponent>,
    }
    impl TestNestingComponent {
        fn new() -> ElementPointer<Self> {
            ElementPointer::new(Self {
                child: TestComponent::new(),
            })
        }
    }
    impl Element for TestNestingComponent {
        fn layout(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> Size {
            self.child.layout(min, max, cx)
                .position(Affine::translate(Vec2::new(10., 10.)), cx);

            Size::new(50., 50.)
        }
    }

    struct TestComponent {}
    impl TestComponent {
        fn new() -> ElementPointer<Self> {
            ElementPointer::new(Self {})
        }
    }
    impl Element for TestComponent {
        fn layout(&mut self, _min: Size, _max: Size, _cx: &mut LayoutContext) -> Size {
            Size::new(30., 30.)
        }
    }

    #[test]
    fn nested_components_adjusts_regions() {
        let test_runner = TestRunner::new(Size::new(100., 100.), |_| TestApp::new());
        test_runner.layout();
        let regions = test_runner.regions.borrow();
        let root = test_runner.root.borrow();

        println!("Regions: {:?}", *regions);

        assert_eq!(
            regions[&root.component.token()],
            (Affine::translate(Vec2::new(10., 10.)), Size::new(50., 50.))
        );
        assert_eq!(
            regions[&root.component.child.token()],
            (Affine::translate(Vec2::new(20., 20.)), Size::new(30., 30.))
        );
    }
}
