use std::{cell::RefCell, rc::Rc, time::Instant};

use glamour::prelude::*;
use palette::Srgba;
use vide::prelude::*;

use crate::{
    framework::{
        context::{Context, DrawContext},
        mouse_region::MouseRegion,
        token::Token,
    },
    util::Mixable,
};

const DEFAULT_ANIM_LENGTH: f32 = 0.2;

pub struct Button {
    token: Token,
    pub rect: Rect,
    idle_background: Srgba,
    hover_background: Srgba,
    draw_contents: Box<dyn Fn(Rect, &mut Layer, &Context)>,
    anim_length: f32,

    on_clicked: Rc<RefCell<Box<dyn Fn(&Context)>>>,
    hover_start: Rc<RefCell<Instant>>,
    hover_t: Rc<RefCell<f32>>,
    hovered: Rc<RefCell<bool>>,
}

impl Button {
    pub fn new<D: Fn(Rect, &mut Layer, &Context) + 'static, C: Fn(&Context) + 'static>(
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
            draw_contents: Box::new(move |rect, layer, cx| draw_contents(rect, layer, cx)),
            anim_length: DEFAULT_ANIM_LENGTH,

            on_clicked: Rc::new(RefCell::new(Box::new(move |cx| on_clicked(cx)))),
            hover_start: Rc::new(RefCell::new(Instant::now())),
            hover_t: Rc::new(RefCell::new(1.5)),
            hovered: Rc::new(RefCell::new(false)),
        }
    }

    pub fn with_anim_length(self, anim_length: f32) -> Self {
        Self {
            anim_length,
            ..self
        }
    }

    fn hover_start(&self) -> Instant {
        *self.hover_start.borrow()
    }

    fn hover_t(&self) -> f32 {
        *self.hover_t.borrow()
    }

    fn hovered(&self) -> bool {
        *self.hovered.borrow()
    }

    pub fn update(&mut self, cx: &Context) -> bool {
        let was_drawing = self.hover_t() < 1.5;
        *self.hover_t.borrow_mut() = self.hover_start().elapsed().as_secs_f32() / self.anim_length;
        was_drawing || self.hover_t() < 1.5
    }

    pub fn draw(&self, layer: &mut Layer, cx: &mut DrawContext) {
        cx.add_mouse_region(
            MouseRegion::new(self.token, self.rect)
                .on_hover({
                    let hover_start = self.hover_start.clone();
                    let hovered = self.hovered.clone();
                    move |_cx| {
                        let was_hovered = *hovered.borrow();
                        if !was_hovered {
                            *hovered.borrow_mut() = true;
                            *hover_start.borrow_mut() = Instant::now();
                        }
                    }
                })
                .on_leave({
                    let hover_start = self.hover_start.clone();
                    let hovered = self.hovered.clone();
                    move |_cx| {
                        let was_hovered = *hovered.borrow();
                        if was_hovered {
                            *hovered.borrow_mut() = false;
                            *hover_start.borrow_mut() = Instant::now();
                        }
                    }
                })
                .on_clicked({
                    let on_clicked = self.on_clicked.clone();
                    move |cx| {
                        let on_clicked = on_clicked.borrow_mut();
                        on_clicked(cx);
                    }
                }),
        );

        if self.rect.contains(&cx.mouse_position()) {
            layer.add_quad(Quad::new(
                self.rect,
                self.idle_background
                    .mix(&self.hover_background, self.hover_t()),
            ));
        } else {
            layer.add_quad(Quad::new(
                self.rect,
                self.hover_background
                    .mix(&self.idle_background, self.hover_t()),
            ));
        }

        (self.draw_contents)(self.rect, layer, cx);
    }
}
