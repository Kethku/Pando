use glamour::prelude::*;
use palette::Srgba;
use vide::*;

use super::button::Button;
use crate::{
    framework::{
        context::{Context, DrawContext, UpdateContext},
        mouse_region::MouseRegion,
        token::Token,
    },
    util::*,
};

const TITLEBAR_HEIGHT: f32 = 34.;
const BUTTON_ASPECT_RATIO: f32 = 1.666;
const X_HEIGHT: f32 = 10.;

pub struct WindowButtons {
    token: Token,
    close: Button,
    maximize: Button,
    minimize: Button,
}

impl WindowButtons {
    pub fn new() -> Self {
        Self {
            token: Token::new(),
            close: Button::new(
                Default::default(),
                Srgba::new(0., 0., 0., 0.),
                *CLOSE,
                |rect, layer, _cx| Self::draw_close_icon(rect, layer),
                |cx| cx.close(),
            ),
            maximize: Button::new(
                Default::default(),
                Srgba::new(0., 0., 0., 0.),
                *BACKGROUND3,
                |rect, layer, cx| Self::draw_maximize_icon(rect, layer, cx),
                |cx| cx.toggle_maximized(),
            ),
            minimize: Button::new(
                Default::default(),
                Srgba::new(0., 0., 0., 0.),
                *BACKGROUND3,
                |rect, layer, _cx| Self::draw_minimize_icon(rect, layer),
                |cx| cx.minimize(),
            ),
        }
    }

    pub fn update(&mut self, cx: &mut UpdateContext) {
        self.close.rect = Rect::new(
            point2!(
                cx.window_size().width - TITLEBAR_HEIGHT * BUTTON_ASPECT_RATIO,
                0.
            ),
            size2!(TITLEBAR_HEIGHT * BUTTON_ASPECT_RATIO, TITLEBAR_HEIGHT),
        );
        self.maximize.rect = Rect::new(
            point2!(
                cx.window_size().width - TITLEBAR_HEIGHT * BUTTON_ASPECT_RATIO * 2.,
                0.
            ),
            size2!(TITLEBAR_HEIGHT * BUTTON_ASPECT_RATIO, TITLEBAR_HEIGHT),
        );
        self.minimize.rect = Rect::new(
            point2!(
                cx.window_size().width - TITLEBAR_HEIGHT * BUTTON_ASPECT_RATIO * 3.,
                0.
            ),
            size2!(TITLEBAR_HEIGHT * BUTTON_ASPECT_RATIO, TITLEBAR_HEIGHT),
        );

        self.close.update(cx);
        self.maximize.update(cx);
        self.minimize.update(cx);
    }

    pub fn draw(&self, cx: &mut DrawContext) {
        cx.add_layer(Layer::new());
        let mut layer = Layer::new();

        let bar_rect = Rect::new(
            point2!(0., 0.),
            size!(cx.window_size().width, TITLEBAR_HEIGHT),
        );
        layer.add_quad(Quad::new(bar_rect, *BACKGROUND2));

        cx.add_mouse_region(MouseRegion::new(self.token, bar_rect).on_down({
            move |cx| {
                cx.drag_window();
            }
        }));

        self.close.draw(&mut layer, cx);
        self.maximize.draw(&mut layer, cx);
        self.minimize.draw(&mut layer, cx);

        cx.add_layer(layer);
    }

    fn draw_close_icon(rect: Rect, layer: &mut Layer) {
        layer.add_path(
            Path::new_line(
                1.,
                *FOREGROUND,
                rect.center() - vector!(X_HEIGHT / 2., X_HEIGHT / 2.),
            )
            .with_line_to(rect.center() + vector!(X_HEIGHT / 2., X_HEIGHT / 2.)),
        );
        layer.add_path(
            Path::new_line(
                1.,
                *FOREGROUND,
                rect.center() + vector!(-X_HEIGHT / 2., X_HEIGHT / 2.),
            )
            .with_line_to(rect.center() + vector!(X_HEIGHT / 2., -X_HEIGHT / 2.)),
        );
    }

    fn draw_maximize_icon(rect: Rect, layer: &mut Layer, cx: &Context) {
        const RESTORE_OFFSET: f32 = 3.;
        let icon_rect = Rect::new(
            rect.center() - vector!(X_HEIGHT / 2., X_HEIGHT / 2.),
            size2!(X_HEIGHT, X_HEIGHT),
        );
        let icon_corners = icon_rect.corners();

        if cx.is_maximized() {
            // Outer border
            layer.add_path(
                Path::new_line(
                    1.,
                    *FOREGROUND,
                    icon_corners[0] + vector!(RESTORE_OFFSET, 0.),
                )
                .with_line_to(icon_corners[1])
                .with_line_to(icon_corners[2] + vector!(0., -RESTORE_OFFSET)),
            );

            // Inner rect
            layer.add_path(
                Path::new_stroke(
                    1.,
                    *FOREGROUND,
                    icon_corners[0] + vector!(0., RESTORE_OFFSET),
                )
                .with_line_to(icon_corners[1] + vector!(-RESTORE_OFFSET, RESTORE_OFFSET))
                .with_line_to(icon_corners[2] + vector!(-RESTORE_OFFSET, 0.))
                .with_line_to(icon_corners[3]),
            );
        } else {
            layer.add_path(
                Path::new_stroke(1., *FOREGROUND, icon_corners[0])
                    .with_line_to(icon_corners[1])
                    .with_line_to(icon_corners[2])
                    .with_line_to(icon_corners[3]),
            );
        }
    }

    fn draw_minimize_icon(rect: Rect, layer: &mut Layer) {
        let icon_rect = Rect::new(
            rect.center() - vector!(X_HEIGHT / 2., X_HEIGHT / 2.),
            size2!(X_HEIGHT, X_HEIGHT),
        );

        layer.add_path(
            Path::new_line(
                1.,
                *FOREGROUND,
                icon_rect.center() + vector!(-icon_rect.width() / 2., 0.),
            )
            .with_line_to(icon_rect.center() + vector!(icon_rect.width() / 2., 0.)),
        );
    }
}
