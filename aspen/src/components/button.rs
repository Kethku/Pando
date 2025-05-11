use std::{cell::RefCell, rc::Rc, time::Instant};

use vello::{
    kurbo::{Rect, Size},
    peniko::{Brush, Color},
};

use crate::{
    context::{DrawContext, EventContext, LayoutContext, UpdateContext},
    element::{Element, ElementPointer},
    util::Mixable,
};

const ANIM_LENGTH: f64 = 0.1;

pub struct Button {
    size: Size,
    idle_background: Color,
    hover_background: Color,
    draw_contents: Box<dyn Fn(Rect, &mut DrawContext)>,

    state: Rc<RefCell<ButtonState>>,
}

struct ButtonState {
    on_clicked: Box<dyn Fn(&mut EventContext)>,
    hover_start: Instant,
    hover_t: f64,
    hovered: bool,
}

impl Button {
    pub fn new<D: Fn(Rect, &mut DrawContext) + 'static, C: Fn(&mut EventContext) + 'static>(
        size: Size,
        idle_background: Color,
        hover_background: Color,
        draw_contents: D,
        on_clicked: C,
    ) -> ElementPointer<Self> {
        ElementPointer::new(Self {
            size,
            idle_background,
            hover_background,
            draw_contents: Box::new(draw_contents),

            state: Rc::new(RefCell::new(ButtonState {
                on_clicked: Box::new(on_clicked),
                hover_start: Instant::now(),
                hover_t: 1.5,
                hovered: false,
            })),
        })
    }
}

impl Element for Button {
    fn update(&mut self, cx: &mut UpdateContext) {
        let mut state = self.state.borrow_mut();
        let was_drawing = state.hover_t < 1.5;
        state.hover_t = state.hover_start.elapsed().as_secs_f64() / ANIM_LENGTH;
        if was_drawing || state.hover_t < 1.5 {
            cx.request_redraw();
        }
    }

    fn layout(&mut self, _min: Size, _max: Size, _cx: &mut LayoutContext) -> Size {
        self.size
    }

    fn draw(&self, cx: &mut DrawContext) {
        let region = cx.region();
        cx.mouse_region(region)
            .on_hover({
                let state = self.state.clone();
                move |_cx| {
                    let mut state = state.borrow_mut();
                    if !state.hovered {
                        state.hovered = true;
                        state.hover_start = Instant::now();
                    }
                }
            })
            .on_leave({
                let state = self.state.clone();
                move |_cx| {
                    let mut state = state.borrow_mut();
                    if state.hovered {
                        state.hovered = false;
                        state.hover_start = Instant::now();
                    }
                }
            })
            .on_click({
                let state = self.state.clone();
                move |cx| {
                    let state = state.borrow();
                    (state.on_clicked)(cx);
                }
            });

        let state = self.state.borrow();
        if region.contains(cx.mouse_position()) {
            cx.set_fill_brush(Brush::Solid(
                self.idle_background
                    .mix(&self.hover_background, state.hover_t),
            ));

            cx.fill(&region);
        } else {
            cx.set_fill_brush(Brush::Solid(
                self.hover_background
                    .mix(&self.idle_background, state.hover_t),
            ));
            cx.fill(&region);
        }

        (self.draw_contents)(region, cx);
    }
}
