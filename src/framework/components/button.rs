use std::{cell::RefCell, rc::Rc, time::Instant};

use glamour::prelude::*;
use palette::Srgba;
use vide::prelude::*;

use crate::{
    framework::{
        context::{DrawContext, EventContext, UpdateContext},
        mouse_region::MouseRegion,
        token::Token,
    },
    util::Mixable,
};

const ANIM_LENGTH: f32 = 0.2;

pub struct Button {
    token: Token,
    pub rect: Rect,
    idle_background: Srgba,
    hover_background: Srgba,
    draw_contents: Box<dyn Fn(Rect, &mut Layer, &mut DrawContext)>,

    state: Rc<RefCell<ButtonState>>,
}

struct ButtonState {
    on_clicked: Box<dyn Fn(&mut EventContext)>,
    hover_start: Instant,
    hover_t: f32,
    hovered: bool,
}

impl Button {
    pub fn new<
        D: Fn(Rect, &mut Layer, &mut DrawContext) + 'static,
        C: Fn(&mut EventContext) + 'static,
    >(
        rect: Rect,
        idle_background: Srgba,
        hover_background: Srgba,
        draw_contents: D,
        on_clicked: C,
    ) -> Self {
        Self {
            token: Token::new(),
            rect,
            idle_background,
            hover_background,
            draw_contents: Box::new(draw_contents),

            state: Rc::new(RefCell::new(ButtonState {
                on_clicked: Box::new(on_clicked),
                hover_start: Instant::now(),
                hover_t: 1.5,
                hovered: false,
            })),
        }
    }

    pub fn update(&mut self, cx: &mut UpdateContext) {
        let mut state = self.state.borrow_mut();
        let was_drawing = state.hover_t < 1.5;
        state.hover_t = state.hover_start.elapsed().as_secs_f32() / ANIM_LENGTH;
        if was_drawing || state.hover_t < 1.5 {
            cx.request_redraw();
        }
    }

    pub fn draw(&self, layer: &mut Layer, cx: &mut DrawContext) {
        cx.add_mouse_region(
            MouseRegion::new(self.token, self.rect)
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

        {
            let state = self.state.borrow();
            if self.rect.contains(&cx.mouse_position()) {
                layer.add_quad(Quad::new(
                    self.rect,
                    self.idle_background
                        .mix(&self.hover_background, state.hover_t),
                ));
            } else {
                layer.add_quad(Quad::new(
                    self.rect,
                    self.hover_background
                        .mix(&self.idle_background, state.hover_t),
                ));
            }
        }

        (self.draw_contents)(self.rect, layer, cx);
    }
}
