use std::{cell::RefCell, rc::Rc, time::Instant};

use glamour::prelude::*;
use palette::Srgba;
use vide::prelude::*;

use crate::{
    framework::{
        context::{DrawContext, EventContext, LayoutContext, UpdateContext},
        element::{Element, ElementPointer},
        mouse_region::MouseRegion,
    },
    util::Mixable,
};

const ANIM_LENGTH: f32 = 0.1;

pub struct Button {
    size: Size2,
    idle_background: Srgba,
    hover_background: Srgba,
    draw_contents: Box<dyn Fn(Rect, &mut DrawContext)>,

    state: Rc<RefCell<ButtonState>>,
}

struct ButtonState {
    on_clicked: Box<dyn Fn(&mut EventContext)>,
    hover_start: Instant,
    hover_t: f32,
    hovered: bool,
}

impl Button {
    pub fn new<D: Fn(Rect, &mut DrawContext) + 'static, C: Fn(&mut EventContext) + 'static>(
        size: Size2,
        idle_background: Srgba,
        hover_background: Srgba,
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
        state.hover_t = state.hover_start.elapsed().as_secs_f32() / ANIM_LENGTH;
        if was_drawing || state.hover_t < 1.5 {
            cx.request_redraw();
        }
    }

    fn layout(&mut self, min: Size2, max: Size2, _cx: &mut LayoutContext) -> Size2 {
        self.size.clamp(min, max)
    }

    fn draw(&self, cx: &mut DrawContext) {
        let region = cx.region();
        cx.add_mouse_region(
            MouseRegion::new(cx.token(), region)
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
                .on_clicked({
                    let state = self.state.clone();
                    move |cx| {
                        let state = state.borrow();
                        (state.on_clicked)(cx);
                    }
                }),
        );

        let state = self.state.borrow();
        if region.contains(&cx.mouse_position()) {
            cx.update_layer(|_, layer| {
                layer.add_quad(Quad::new(
                    region,
                    self.idle_background
                        .mix(&self.hover_background, state.hover_t),
                ));
            });
        } else {
            cx.update_layer(|_, layer| {
                layer.add_quad(Quad::new(
                    region,
                    self.hover_background
                        .mix(&self.idle_background, state.hover_t),
                ));
            });
        }

        (self.draw_contents)(region, cx);
    }
}
