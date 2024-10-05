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
            token: Token::new::<E>(),
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
        let mut child_cx = cx.child(self.token);
        let size = self.element.layout(min, max, &mut child_cx).clamp(min, max);
        LayoutResult {
            size,
            token: self.token,
        }
    }

    pub fn draw(&self, cx: &mut DrawContext) {
        let mut child_cx = cx.child(self.token);
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

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::Arc};

    use glamour::prelude::*;
    use vide::Scene;

    use super::*;
    use crate::framework::{
        context::{
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
        fn layout(&mut self, min: Size2, max: Size2, cx: &mut LayoutContext) -> Size2 {
            self.component
                .layout(min, max, cx)
                .position(point2!(10., 10.), cx);

            size2!(70., 70.)
        }
    }

    struct TestComponent {}
    impl TestComponent {
        fn new() -> ElementPointer<Self> {
            ElementPointer::new(Self {})
        }
    }
    impl Element for TestComponent {
        fn layout(&mut self, _min: Size2, _max: Size2, _cx: &mut LayoutContext) -> Size2 {
            size2!(50., 50.)
        }
    }

    #[test]
    fn layout_result_position_records_rect() {
        let event_state = EventState::new();
        let event_loop = MockContextEventLoop::new();
        let window = Arc::new(MockContextWindow::new());

        struct TestComponent {}
        let token = Token::new::<TestComponent>();

        let cx = Context::new(&event_state, &event_loop, window.clone(), token);

        let mut regions = HashMap::new();
        let mut children = HashMap::new();
        let mut layout_cx = LayoutContext::new(cx, &mut regions, &mut children);

        let result = LayoutResult {
            size: size2!(10., 10.),
            token,
        };
        result.position(point2!(5., 5.), &mut layout_cx);

        assert_eq!(
            regions[&token],
            Rect::new(point2!(5., 5.), size2!(10., 10.))
        );
    }

    #[test]
    fn nested_components_adjusts_regions() {
        let event_state = EventState::new();
        let event_loop = MockContextEventLoop::new();
        let window = Arc::new(MockContextWindow::new());

        let mut app = TestApp::new();

        let cx = Context::new(&event_state, &event_loop, window.clone(), app.token());

        let mut regions = HashMap::new();
        let mut children = HashMap::new();
        let mut layout_cx = LayoutContext::new(cx, &mut regions, &mut children);

        app.layout(size2!(0., 0.), size2!(100., 100.), &mut layout_cx)
            .position(point2!(10., 10.), &mut layout_cx);

        assert_eq!(
            regions[&app.token()],
            Rect::new(point2!(10., 10.), size2!(70., 70.))
        );
        assert_eq!(
            regions[&app.component.token()],
            Rect::new(point2!(20., 20.), size2!(50., 50.))
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
            let cx = Context::new(&event_state, &event_loop, window.clone(), app.token());
            let mut layout_cx = LayoutContext::new(cx, &mut regions, &mut children);

            app.layout(size2!(0., 0.), size2!(100., 100.), &mut layout_cx)
                .position(point2!(10., 10.), &mut layout_cx);
        }

        let cx = Context::new(&event_state, &event_loop, window.clone(), app.token());
        let mut mouse_region_manager = MouseRegionManager::new();
        let mut scene = Scene::new();
        let mut draw_cx = DrawContext::new(cx, &mut mouse_region_manager, &regions, &mut scene);

        assert_eq!(
            draw_cx.region(),
            Rect::new(point2!(10., 10.), size2!(70., 70.))
        );
        let child_draw_cx = draw_cx.child(app.component.token());
        assert_eq!(
            child_draw_cx.region(),
            Rect::new(point2!(20., 20.), size2!(50., 50.))
        );
    }
}
