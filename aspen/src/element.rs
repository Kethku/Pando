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
        let mut child_cx = cx.child(self.token, self.children());
        self.element.update(&mut child_cx);
    }

    #[must_use]
    pub fn layout<'a>(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> LayoutResult {
        let mut child_cx = cx.child(self.token, self.children());
        let size = self.element.layout(min, max, &mut child_cx).clamp(min, max);
        LayoutResult {
            size,
            token: self.token,
        }
    }

    pub fn draw<'a>(&self, cx: &mut DrawContext) {
        let mut child_cx = cx.child(self.token, self.children());

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
        let cx = cx.child(self.token, self.children());
        callback(&cx)
    }

    pub fn insert_state<State: Any>(self, state: State, cx: &Context) -> ElementPointer<E> {
        self.with_context(cx, |cx| cx.insert_state(state));
        self
    }

    pub fn with_state<'a, State: Any, Result>(
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
    use std::{collections::HashMap, sync::Arc};

    use vello::Scene;

    use super::*;
    use crate::{
        context_stack::{
            Context, DrawContext, EventState, LayoutContext, MockContextEventLoop,
            MockContextWindow,
        },
        mouse_region::MouseRegionManager,
    };

    struct TestApp {
        component: ElementPointer<TestComponent>,
    }
    impl TestApp {
        fn new() -> ElementPointer<Self> {
            ElementPointer::new(Self {
                component: TestComponent::new(),
            })
        }
    }
    impl Element for TestApp {
        fn layout(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> Size {
            self.component
                .layout(min, max, cx)
                .position(Point::new(10., 10.), cx);

            Size::new(70., 70.)
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
            Size::new(50., 50.)
        }
    }

    #[test]
    fn layout_result_position_records_rect() {
        let event_state = EventState::new();
        let event_loop = MockContextEventLoop::new();
        let window = Arc::new(MockContextWindow::new());

        struct TestComponent {}
        let token = Token::new::<TestComponent>();

        let cx = {
            let event_state: &'a EventState = &event_state;
            let event_loop: &'a dyn ContextEventLoop = &event_loop;
            let window: Arc<dyn ContextWindow> = window.clone();
            Context {
                event_state,
                event_loop,
                window,
                shaper: token,
                default_text_styles: Vec::new(),
                element_token,
            }
        };

        let mut regions = HashMap::new();
        let mut children = HashMap::new();
        let mut layout_cx = LayoutContext::new(cx, &mut regions, &mut children);

        let result = LayoutResult {
            size: Size::new(10., 10.),
            token,
        };
        result.position(Point::new(5., 5.), &mut layout_cx);

        assert_eq!(
            regions[&token],
            Rect::from_origin_size(Point::new(5., 5.), Size::new(10., 10.))
        );
    }

    #[test]
    fn nested_components_adjusts_regions() {
        let event_state = EventState::new();
        let event_loop = MockContextEventLoop::new();
        let window = Arc::new(MockContextWindow::new());

        let mut app = TestApp::new();

        let cx = {
            let event_state: &'a EventState = &event_state;
            let event_loop: &'a dyn ContextEventLoop = &event_loop;
            let window: Arc<dyn ContextWindow> = window.clone();
            let shaper = app.token();
            Context {
                event_state,
                event_loop,
                window,
                shaper,
                default_text_styles: Vec::new(),
                element_token,
            }
        };

        let mut regions = HashMap::new();
        let mut children = HashMap::new();
        let mut layout_cx = LayoutContext::new(cx, &mut regions, &mut children);

        app.layout(Size::new(0., 0.), Size::new(100., 100.), &mut layout_cx)
            .position(Point::new(10., 10.), &mut layout_cx);

        assert_eq!(
            regions[&app.token()],
            Rect::from_origin_size(Point::new(10., 10.), Size::new(70., 70.))
        );
        assert_eq!(
            regions[&app.component.token()],
            Rect::from_origin_size(Point::new(20., 20.), Size::new(50., 50.))
        );
    }

    #[test]
    fn drawn_rect_matches_positioned_layout() {
        let event_state = EventState::new();
        let event_loop = MockContextEventLoop::new();
        let window = Arc::new(MockContextWindow::new());

        let mut app = TestApp::new();
        let mut regions = HashMap::new();
        let mut children = HashMap::new();

        {
            let cx = {
                let event_state: &'a EventState = &event_state;
                let event_loop: &'a dyn ContextEventLoop = &event_loop;
                let window: Arc<dyn ContextWindow> = window.clone();
                let shaper = app.token();
                Context {
                    event_state,
                    event_loop,
                    window,
                    shaper,
                    default_text_styles: Vec::new(),
                    element_token,
                }
            };
            let mut layout_cx = LayoutContext::new(cx, &mut regions, &mut children);

            app.layout(Size::new(0., 0.), Size::new(100., 100.), &mut layout_cx)
                .position(Point::new(10., 10.), &mut layout_cx);
        }

        let cx = {
            let event_state: &'a EventState = &event_state;
            let event_loop: &'a dyn ContextEventLoop = &event_loop;
            let window: Arc<dyn ContextWindow> = window.clone();
            let shaper = app.token();
            Context {
                event_state,
                event_loop,
                window,
                shaper,
                default_text_styles: Vec::new(),
                element_token,
            }
        };
        let mut mouse_region_manager = MouseRegionManager::new();
        let mut scene = Scene::new();
        let mut draw_cx = DrawContext::new(cx, &mut mouse_region_manager, &regions, &mut scene);

        assert_eq!(
            draw_cx.region(),
            Rect::from_origin_size(Point::new(10., 10.), Size::new(70., 70.))
        );
        let child_draw_cx = draw_cx.child(app.component.token());
        assert_eq!(
            child_draw_cx.region(),
            Rect::from_origin_size(Point::new(20., 20.), Size::new(50., 50.))
        );
    }
}
